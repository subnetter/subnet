// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::server_service::SNP_PROTOCOL_VERSION;
use crate::services::provider_id::ProviderIdService;
use crate::services::provider_id_service::GetCurrentIdentityBundle;
use anyhow::{anyhow, Result};
use base::api_types_extensions::{Signed, SignedWithExternalVerifier};
use base::hex_utils::short_hex_string;
use base::snp::snp_core_types::{DialupInfo, PrivateProviderIdentityBundle};
use base::snp::snp_server_api::provider_core_service_client::ProviderCoreServiceClient;
use base::snp::snp_server_api::{
    DrSessionHeader, GetIdentityBundleRequest, MessageType, NewSessionRequest, TypedMessage,
};
use bytes::Bytes;
use chrono::prelude::*;
use common::dr_service::DRService;
use common::network_salt::NET_SALT;
use common::typed_msg_extensions::TypedMessageExtensions;
use crypto::x2dh;
use crypto::x2dh::ProtocolInputAlice;
use double_ratchet::chain_key::ChainKey;
use double_ratchet::dr::DoubleRatchet;
use double_ratchet::session_key::SessionKey;
use ed25519_dalek::PublicKey;
use rand_core::OsRng;
use std::collections::HashMap;
use tonic::transport::Channel;
use xactor::*;

/// ServerToServer is a system service which provides a service for sending an arbitrary typed message to
/// another server that this server has dial-up info of.
#[derive(Debug)]
pub struct ServerToServerService {
    servers_net_clients: HashMap<Vec<u8>, ProviderCoreServiceClient<Channel>>,
}

impl Default for ServerToServerService {
    fn default() -> Self {
        ServerToServerService {
            servers_net_clients: HashMap::new(),
        }
    }
}

impl Service for ServerToServerService {}

#[async_trait::async_trait]
impl Actor for ServerToServerService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("ServerToServerService started");
        Ok(())
    }
}

/// Send a message to another provider based on his public key and dialup info
#[message(result = "Result<(TypedMessage)>")]
pub struct SendMessageToServer {
    pub dialup_info: DialupInfo,   // Provider dialup info
    pub receiver_id: PublicKey,    // ed25519 public key of provider
    pub message_type: MessageType, // Protobuf message type
    pub message: Bytes,            // Protobuf serialized data
}

/// SendMessage handler - send a message to a remote server and return result message
#[async_trait::async_trait]
impl Handler<SendMessageToServer> for ServerToServerService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: SendMessageToServer,
    ) -> Result<TypedMessage> {
        // In this flow we call this provider Alice the remote service provider Bob.

        // step 1: check if we have an existing DR session with the remote server

        debug!(
            "Send message request to provider: {:?}",
            short_hex_string(msg.receiver_id.to_bytes().as_ref())
        );

        let dr_session = DRService::get_dr_session(msg.receiver_id).await?;

        // step 2: check if we have a net client with the remote provider and if not then connect to it and store
        // client for future messages
        let mut bob_api_service = match self
            .servers_net_clients
            .get(&msg.receiver_id.as_ref().to_vec())
        {
            Some(c) => {
                // todo: find out what happens if remote server closed the network connection and we need to connect again to it

                c.clone()
            }
            None => {
                let address = format!(
                    "http://{}:{}",
                    msg.dialup_info.ip_address, msg.dialup_info.port
                );

                debug!("Connecting to a remote provider at {} ...", address);
                let new_client = ProviderCoreServiceClient::connect(address).await?;
                self.servers_net_clients
                    .insert(msg.receiver_id.as_ref().to_vec(), new_client.clone());
                new_client
            }
        };

        // Get our current PrivateIdentityBundle as we need it for X2DH and DR execution
        let alice_provider_data = ProviderIdService::from_registry().await.unwrap();
        let alice_provider_bundle: PrivateProviderIdentityBundle = alice_provider_data
            .call(GetCurrentIdentityBundle {})
            .await??;

        // if we have an existing dr session then send the message over that session and return result
        if dr_session.is_some() {
            return self
                .send_message_in_dr_session(
                    &mut dr_session.unwrap(),
                    &msg,
                    msg.receiver_id,
                    alice_provider_bundle,
                    &mut bob_api_service,
                )
                .await;
        }

        debug!("No dr session with remote provider - start a new session...");

        // step 3: get bob's current id bundle via its public api - note that it might be different
        // than the bundle that the payload message is using to encrypt its content.

        let bob_provider_bundle = bob_api_service
            .get_identity_bundle(GetIdentityBundleRequest {
                protocol_version: SNP_PROTOCOL_VERSION.into(),
            })
            .await?
            .into_inner()
            .bundle
            .ok_or_else(|| anyhow!("missing provider bundle from response message"))?;

        // step 4: execute x2dh to create a new dr session with bob

        let alice_id_key_pair = alice_provider_bundle
            .provider_id_keypair
            .as_ref()
            .ok_or_else(|| anyhow!("missing key pair from our bundle"))?;

        let ikb = bob_provider_bundle
            .get_provider_id_ed25519_public_key()
            .map_err(|_| anyhow!("missing other provider pub key"))?;

        let pkb = bob_provider_bundle
            .get_provider_x25519_pre_key()
            .map_err(|_| anyhow!("missing other provider pre-key"))?;

        // Alice x2dh protocol input
        let input_alice = ProtocolInputAlice {
            ikb,
            pkb,
            b_bundle_id: bob_provider_bundle.time_stamp,
        };

        // Alice executes x2dh with bob and get the output
        let output_alice = x2dh::execute_alice(&input_alice);

        // Alice creates a DR session with bob and using it to get the enc key for her first message with bob
        let input = SessionKey::from(NET_SALT.as_ref());
        let dr_root_chain_key = ChainKey::from(output_alice.shared_secret.as_ref());
        let mut alice_dr = DoubleRatchet::new_with_peer(
            input,
            dr_root_chain_key,
            &mut OsRng,
            &pkb,
            output_alice.ad.clone(),
        )
        .map_err(|e| anyhow!("failed to create dr with party: {:?}", e))?;

        // Alice sends her current public dr key with a first message (enc w first send message key) to bob
        let alice_pub_dr_key = alice_dr.get_public_key().unwrap();
        let alice_send_key = alice_dr.next_sending_key().unwrap();

        // step 5: create TypedMessage and encrypt it in dr session

        let alice_entity = alice_provider_bundle.get_provider_id_entity()?;
        let bob_identity = bob_provider_bundle
            .provider_id
            .ok_or_else(|| anyhow!("missing bob id in bundle"))?;

        // this is the message alice sends to bob
        let mut typed_msg = TypedMessage {
            time_stamp: Utc::now().timestamp_nanos() as u64,
            msg_type: msg.message_type as i32,
            message: msg.message.to_vec(),
            receiver: Some(bob_identity.clone()),
            sender: Some(alice_entity.clone()),
            signature: None,
        };

        let ika_pair = alice_id_key_pair.to_ed2559_kaypair();

        typed_msg
            .sign(&ika_pair)
            .map_err(|e| anyhow!("failed to sign message: {:?}", e))?;

        let enc_msg = TypedMessageExtensions::encrypt_msg(
            typed_msg,
            &alice_send_key.1,
            output_alice.ad.as_ref(),
        )
        .map_err(|e| anyhow!("failed to enc message: {:?}", e))?;

        // step 6: save the dr session using the dr server
        DRService::save_dr_session(ikb, alice_dr.clone()).await?;

        // step 7: send NewSessionRequest(message) to bob
        let message = base::snp::snp_server_api::Message {
            header: Some(DrSessionHeader {
                session_id: alice_dr.session_id,
                dr_pub_key: Some(base::snp::snp_core_types::PublicKey {
                    key: alice_pub_dr_key.as_bytes().to_vec(),
                }),
                prev_count: 0,
                count: 0,
            }),
            enc_typed_msg: enc_msg.to_vec(),
        };

        let eka = base::snp::snp_core_types::PublicKey {
            // Alice ephemeral pub key for X2DH protocol
            key: output_alice.eka.as_bytes().to_vec(),
        };

        // Note that alice id is not exposed in this message clear-text !!!
        let mut new_session_request = NewSessionRequest {
            time_stamp: Utc::now().timestamp_nanos() as u64,
            receiver: Some(bob_identity),
            sender_ephemeral_key: Some(eka),
            receiver_one_time_prekey_id: 0,
            message: Some(message),
            sender_signature: None,
            receiver_bundle_id: bob_provider_bundle.time_stamp,
            net_id: 0,
            protocol_version: SNP_PROTOCOL_VERSION.into(),
        };

        new_session_request.sign(&ika_pair).unwrap();

        debug!("sending new_session request to remote provider...");

        let response = bob_api_service
            .new_session(tonic::Request::new(new_session_request))
            .await?
            .into_inner();

        debug!("processing new_session response from remote provider...");

        // step 8: decrypt the response's message in the dr session, validate it and return response TypedMessage to caller
        let message = response
            .message
            .ok_or_else(|| anyhow!("missing message in response"))?;

        let resp_msg = self
            .handle_server_response_message(message, &mut alice_dr, ikb)
            .await?;

        Ok(resp_msg)
    }
}
