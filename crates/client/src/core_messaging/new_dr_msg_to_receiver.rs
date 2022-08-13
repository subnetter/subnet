// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, Result};
use base::hex_utils::short_hex_string;

use base::snp::snp_core_types::PublicKey;
use base::snp::snp_server_api::{DrSessionHeader, Message, TypedMessage};
use common::dr_service::DRService;
use common::typed_msg_extensions::TypedMessageExtensions;

impl SimpleClient {
    /// Create a DR message designated to any other entity we already have a DR session with.
    /// This should be used with both providers and other clients as the logic is identical for the Message
    pub(crate) async fn create_message_to_receiver(
        &mut self,
        receiver: ed25519_dalek::PublicKey,
        message: TypedMessage,
    ) -> Result<Message> {
        // in this flow Alice is this client and Bob is the receiver

        let mut dr = DRService::get_dr_session(receiver)
            .await?
            .ok_or_else(|| anyhow!("expected dr session with provider"))?;

        let alice_send_key = dr.next_sending_key()?;

        let ad = dr
            .ad
            .as_ref()
            .ok_or_else(|| anyhow!("missing ad from dr"))?;

        debug!(
            "****** Alice AD: {}",
            short_hex_string(ad.to_vec().as_ref())
        );

        debug!(
            "****** Alice sending dr key: [{}] {}",
            alice_send_key.0,
            short_hex_string(alice_send_key.1.as_bytes())
        );

        debug!("****** Alice dr session id: {}", dr.session_id);

        debug!(
            "****** Alice dr pub key: {}",
            short_hex_string(dr.get_public_key().unwrap().as_bytes().to_vec().as_ref())
        );

        // We create a new encrypted msg from the typed message we used for the previous message (get service terms)
        let enc_msg = TypedMessageExtensions::encrypt_msg(message, &alice_send_key.1, ad).unwrap();

        let message = base::snp::snp_server_api::Message {
            header: Some(DrSessionHeader {
                session_id: dr.session_id,
                dr_pub_key: Some(PublicKey {
                    key: dr.get_public_key().unwrap().as_bytes().to_vec(),
                }),
                prev_count: 0,
                count: alice_send_key.0,
            }),
            enc_typed_msg: enc_msg.to_vec(),
        };

        DRService::save_dr_session(receiver, dr).await?;

        Ok(message)
    }
}
