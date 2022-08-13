// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;

use anyhow::{anyhow, Result};
use base::api_types_extensions::Signed;
use base::snp::snp_server_api::{Message, TypedMessage};
use common::dr_service::DRService;
use common::typed_msg_extensions::TypedMessageExtensions;
use rand_core::OsRng;
use std::convert::TryFrom;

impl SimpleClient {
    /// Decode a TypedMessage from a DR Message
    /// Helper method - should ONLY be called from SimpleClient actor handlers to ensure state consistency
    pub(crate) async fn decode_incoming_dr_message(message: Message) -> Result<TypedMessage> {
        let dr_header = message.header.ok_or_else(|| anyhow!("missing dr header"))?;
        let dr_session = DRService::get_dr_session_by_id(dr_header.session_id)
            .await?
            .ok_or_else(|| anyhow!("didn't find existing dr session with sender"))?;

        let mut dr = dr_session.0;
        let sender_pub_key = dr_session.1;

        let key_data = dr_header
            .dr_pub_key
            .ok_or_else(|| anyhow!("missing dr pub key"))?;

        use crypto::utils::X25519PublicKeyWrapper;
        let bob_dr_key_wrapper = X25519PublicKeyWrapper::try_from(key_data.key.as_slice()).unwrap();

        if dr_header.count == 0 {
            // we only should ratchet and update with new dr pub key if sending chain key is != 0
            dr.ratchet(
                &mut OsRng,
                &bob_dr_key_wrapper.0.clone(),
                dr_header.prev_count,
            )?;
        }

        let receive_key = dr.get_receiving_key(dr_header.count).unwrap();
        let ad = dr.get_ad().map_err(|_| anyhow!("missing ad"))?;

        let typed_message = TypedMessageExtensions::decrypt_msg(
            message.enc_typed_msg.as_slice(),
            &receive_key,
            ad,
        )?;

        typed_message.verify_signature()?;
        DRService::save_dr_session(sender_pub_key, dr).await?;
        Ok(typed_message)
    }
}
