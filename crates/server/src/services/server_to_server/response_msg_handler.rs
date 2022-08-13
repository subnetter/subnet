// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use super::server_to_server_service::ServerToServerService;
use anyhow::{anyhow, bail, Result};
use base::api_types_extensions::Signed;
use base::snp::snp_server_api::TypedMessage;
use common::dr_service::DRService;
use common::typed_msg_extensions::TypedMessageExtensions;
use crypto::utils::X25519PublicKeyWrapper;
use double_ratchet::dr::DoubleRatchet;
use ed25519_dalek::PublicKey;
use rand_core::OsRng;
use std::convert::TryFrom;

/// ServerToServerService facilitates p2p communications with other servers
impl ServerToServerService {
    /// Handle a DR Message response from another server
    pub async fn handle_server_response_message(
        &mut self,
        message: base::snp::snp_server_api::Message,
        dr_session: &mut DoubleRatchet,
        ikb: PublicKey, // expected other party public key
    ) -> Result<TypedMessage> {
        let resp_dr_header = message.header.unwrap();
        let key_data = resp_dr_header.dr_pub_key.unwrap();
        let bob_dr_key_wrapper = X25519PublicKeyWrapper::try_from(key_data.key.as_slice()).unwrap();

        let index = resp_dr_header.count;

        if index == 0 {
            dr_session.ratchet(
                &mut OsRng,
                &bob_dr_key_wrapper.0.clone(),
                resp_dr_header.prev_count,
            )?;
        }

        let alice_receive_key = dr_session.get_receiving_key(index)?;
        let ad = dr_session
            .ad
            .as_ref()
            .ok_or_else(|| anyhow!("missing ad"))?;

        let resp_msg = TypedMessageExtensions::decrypt_msg(
            message.enc_typed_msg.as_slice(),
            &alice_receive_key,
            ad.as_ref(),
        )?;

        // Verify that bob signed the typed message
        resp_msg.verify_signature()?;
        let bob_key_from_resp = resp_msg.get_ika()?;

        if bob_key_from_resp != ikb {
            bail!("invalid response - unexpected public id in response");
        }

        // Save the updated dr session using the dr server
        DRService::save_dr_session(ikb, dr_session.clone()).await?;

        Ok(resp_msg)
    }
}
