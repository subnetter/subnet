// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::{SimpleClient, SNP_PROTOCOL_VERSION};
use anyhow::{anyhow, Result};
use base::api_types_extensions::{Signed, SignedWithExternalVerifier};
use base::hex_utils::short_hex_string;
use base::snp::snp_core_types::{EntityId, PublicKey};
use base::snp::snp_server_api::{DrSessionHeader, MessageType, NewSessionRequest, TypedMessage};
use chrono::prelude::*;
use common::dr_service::DRService;
use common::network_salt::NET_SALT;
use common::typed_msg_extensions::TypedMessageExtensions;
use crypto::utils::X25519PublicKeyWrapper;
use crypto::x2dh;
use crypto::x2dh::ProtocolInputAlice;
use double_ratchet::chain_key::ChainKey;
use double_ratchet::dr::DoubleRatchet;
use double_ratchet::session_key::SessionKey;
use rand_core::OsRng;
use std::convert::TryFrom;

impl SimpleClient {
    /// Send a new session request to provider with an included message
    pub(crate) async fn send_new_session_to_provider(
        &mut self,
        msg_type: MessageType, // included message type
        msg_data: Vec<u8>,     // included message serialized bytes
    ) -> Result<TypedMessage> {
        let provider_bundle = self
            .provider_bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing provider bundle"))?;

        let ikb = provider_bundle
            .get_provider_id_ed25519_public_key()
            .unwrap();

        let pkb = provider_bundle.get_provider_x25519_pre_key().unwrap();

        // Alice X2DH protocol input
        let input_alice = ProtocolInputAlice {
            ikb,
            pkb,
            b_bundle_id: provider_bundle.time_stamp,
        };

        // Alice executes x2dh with bob and get the output
        let output_alice = x2dh::execute_alice(&input_alice);

        // debug!("Alice x2dh output: {:?}", output_alice);

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
        .unwrap();

        // Alice sends her current public dr key with a first message (enc w first send message key) to bob
        let alice_pub_dr_key = alice_dr.get_public_key().unwrap();
        let alice_send_key = alice_dr.next_sending_key().unwrap();
        let ikb_pub = provider_bundle.get_provider_id_public_key().unwrap();
        let ikb_identity = EntityId {
            public_key: Some(ikb_pub.clone()),
            nickname: "".to_string(),
        };

        debug!(">>> alice dr session id: {}", alice_dr.session_id);

        debug!(
            ">>> alice AD: {}",
            short_hex_string(alice_dr.ad.as_ref().unwrap().as_ref())
        );

        debug!(
            ">>> alice dr send key for new session: [{}] {}",
            alice_send_key.0,
            short_hex_string(alice_send_key.1.as_bytes())
        );

        // this is the message alice sends to bob (a start service request)
        let typed_msg = self.create_typed_message(msg_type, msg_data, ikb)?;

        let enc_msg = TypedMessageExtensions::encrypt_msg(
            typed_msg.clone(),
            &alice_send_key.1,
            output_alice.ad.as_ref(),
        )
        .unwrap();

        let message = base::snp::snp_server_api::Message {
            header: Some(DrSessionHeader {
                session_id: alice_dr.session_id,
                dr_pub_key: Some(PublicKey {
                    key: alice_pub_dr_key.as_bytes().to_vec(),
                }),
                prev_count: 0,
                count: alice_send_key.0,
            }),
            enc_typed_msg: enc_msg.to_vec(),
        };

        let eka = PublicKey {
            // Alice ephemeral pub key for X2DH protocol
            key: output_alice.eka.as_bytes().to_vec(),
        };

        // Note that alice id is not exposed in this message clear-text !!!
        let mut new_session_request = NewSessionRequest {
            time_stamp: Utc::now().timestamp_nanos() as u64,
            receiver: Some(ikb_identity),
            sender_ephemeral_key: Some(eka),
            receiver_one_time_prekey_id: 0,
            message: Some(message),
            sender_signature: None,
            receiver_bundle_id: provider_bundle.time_stamp,
            net_id: 0,
            protocol_version: SNP_PROTOCOL_VERSION.into(),
        };

        new_session_request.sign(&self.client_id)?;

        let provider_api_client = self
            .provider_net_client
            .as_mut()
            .ok_or_else(|| anyhow!("need provider net client"))?;

        // save a clone so we can st;il use mutable alice_dr - we'll save it again later
        DRService::save_dr_session(ikb, alice_dr.clone()).await?;

        let response = provider_api_client
            .new_session(tonic::Request::new(new_session_request))
            .await
            .expect("failed to send NewSession request to provider")
            .into_inner();

        // Alice decrypts the response message using the dr session with the server bob
        // Validate it is the expected response to the original request message (get service terms)...

        let message = response.message.unwrap();
        let resp_dr_header = message.header.unwrap();
        let key_data = resp_dr_header.dr_pub_key.unwrap();
        let bob_dr_key_wrapper = X25519PublicKeyWrapper::try_from(key_data.key.as_slice()).unwrap();

        alice_dr.ratchet(
            &mut OsRng,
            &bob_dr_key_wrapper.0.clone(),
            resp_dr_header.prev_count,
        )?;

        let alice_receive_key = alice_dr.get_receiving_key(resp_dr_header.count)?;

        let resp_message = TypedMessageExtensions::decrypt_msg(
            message.enc_typed_msg.as_slice(),
            &alice_receive_key,
            output_alice.ad.as_ref(),
        )
        .unwrap();

        DRService::save_dr_session(ikb, alice_dr).await?;

        debug!(
            "Bob new dr pub key: {}",
            short_hex_string(&bob_dr_key_wrapper.0.as_bytes().to_vec().as_ref())
        );

        // Alice checks that bob signed the typed message
        resp_message
            .verify_signature()
            .expect("failed to verify signature on response message");

        Ok(resp_message)
    }
}
