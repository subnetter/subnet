// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::{SimpleClient, SNP_PROTOCOL_VERSION};
use anyhow::Result;
use base::api_types_extensions::SignedWithExternalVerifier;
use base::snp::snp_core_types::{ClientIdentityBundle, EntityId, PublicKey};
use base::snp::snp_server_api::{DrSessionHeader, NewSessionRequest, TypedMessage};
use chrono::prelude::*;
use common::dr_service::DRService;
use common::network_salt::NET_SALT;
use common::typed_msg_extensions::TypedMessageExtensions;
use crypto::x2dh;
use crypto::x2dh::ProtocolInputAlice;
use double_ratchet::chain_key::ChainKey;
use double_ratchet::dr::DoubleRatchet;
use double_ratchet::session_key::SessionKey;
use rand_core::OsRng;

/// Creates a new session message with a payload message designated to a receiving client.
impl SimpleClient {
    /// Create a new session with a remote client and include a message to send to it
    pub(crate) async fn new_session_message_to_client(
        &self,
        bob_bundle: &ClientIdentityBundle, // recipient client
        message: TypedMessage,             // the message to send in this session
    ) -> Result<NewSessionRequest> {
        let ikb = bob_bundle.get_client_id_ed25519_public_key().unwrap();
        let pkb = bob_bundle.get_client_x25519_pre_key().unwrap();

        // Alice X2DH protocol input
        let input_alice = ProtocolInputAlice {
            ikb,
            pkb,
            b_bundle_id: bob_bundle.time_stamp,
        };

        // Alice executes x2dh with bob and get the output
        let output_alice = x2dh::execute_alice(&input_alice);

        //debug!("Alice x2dh output: {:?}", output_alice);

        // Alice creates a DR session with bob and using it to get the enc key for her first message with bob
        let input = SessionKey::from(NET_SALT.as_ref());
        let dr_root_chain_key = ChainKey::from(output_alice.shared_secret.as_ref());

        let mut dr = DoubleRatchet::new_with_peer(
            input,
            dr_root_chain_key,
            &mut OsRng,
            &pkb,
            output_alice.ad.clone(),
        )
        .unwrap();

        // Alice sends her current public dr key with a first message (enc w first send message key) to bob
        let alice_pub_dr_key = dr.get_public_key().unwrap();
        let alice_send_key = dr.next_sending_key().unwrap();
        let enc_msg = TypedMessageExtensions::encrypt_msg(
            message,
            &alice_send_key.1,
            output_alice.ad.as_ref(),
        )?;

        let message = base::snp::snp_server_api::Message {
            header: Some(DrSessionHeader {
                session_id: dr.session_id,
                dr_pub_key: Some(PublicKey {
                    key: alice_pub_dr_key.as_bytes().to_vec(),
                }),
                prev_count: 0,
                count: alice_send_key.0,
            }),
            enc_typed_msg: enc_msg.to_vec(),
        };

        DRService::save_dr_session(ikb, dr).await?;

        let eka = PublicKey {
            // Alice ephemeral pub key for X2DH protocol
            key: output_alice.eka.as_bytes().to_vec(),
        };

        let ikb_pub = bob_bundle.get_client_id_public_key().unwrap();
        let bob_entity = EntityId {
            public_key: Some(ikb_pub.clone()),
            nickname: "".to_string(),
        };

        // Note that alice id is not exposed in this message clear-text !!!
        let mut new_session_request = NewSessionRequest {
            time_stamp: Utc::now().timestamp_nanos() as u64,
            receiver: Some(bob_entity),
            sender_ephemeral_key: Some(eka),
            receiver_one_time_prekey_id: 0,
            message: Some(message),
            sender_signature: None,
            receiver_bundle_id: bob_bundle.time_stamp,
            net_id: 0,
            protocol_version: SNP_PROTOCOL_VERSION.into(),
        };

        // debug!("new session request: {:?}", new_session_request);

        // Sign and add signature to the request
        new_session_request.sign(&self.client_id)?;

        Ok(new_session_request)
    }
}
