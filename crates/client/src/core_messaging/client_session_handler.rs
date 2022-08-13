// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, bail, Result};
use base::api_types_extensions::{Signed, SignedWithExternalVerifier};
use base::hex_utils::short_hex_string;
use base::snp::snp_server_api::{Message, NewSessionRequest};
use bytes::Bytes;
use common::dr_service::DRService;
use common::network_salt;
use common::typed_msg_extensions::TypedMessageExtensions;
use common::x2dh_service::{ExecuteProtocolAsBob, X2DHService};
use crypto::x2dh::{ProtocolInputBob, ProtocolOutputBob};
use double_ratchet::chain_key::ChainKey;
use double_ratchet::dr::DoubleRatchet;
use double_ratchet::session_key::SessionKey;
use ed25519_dalek::Keypair;
use rand_core::OsRng;
use xactor::*;

impl SimpleClient {
    /// Handle a new DR session request + message from another client on the network (not current provider)
    /// Must only be called from SimpleClient Actor handlers
    pub(crate) async fn handle_new_session_req_from_entity(
        &mut self,
        req_data: NewSessionRequest,
    ) -> Result<()> {
        debug!("hello :-)");

        //
        // In this flow we are Bob (b) and the other party which initiated the protocol is Alice (a).

        // check that caller is using our only client bundle id
        let client_bundle = self
            .client_bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing our own client bundle"))?;

        if client_bundle.time_stamp != req_data.receiver_bundle_id {
            bail!("caller used an old client bundle - this is not supported yet")
        }

        // step 1 - create X2DH ProtocolInputBob for X2DH protocol
        let eka = req_data.get_eka().map_err(|_| anyhow!("invalid eka"))?;

        debug!(
            "eka public: {}",
            short_hex_string(eka.as_bytes().to_vec().as_ref())
        );

        let ikb_pair = Keypair::from_bytes(self.client_id.to_bytes().as_ref())
            .map_err(|_| anyhow!("invalid data"))?;

        let input_bob = ProtocolInputBob {
            eka,
            ikb_pair,
            pkb_private: self.pre_key.clone(),
            b_bundle_id: req_data.receiver_bundle_id,
        };

        // step 3 - call X2DHService to execute x2dh with Alice (obtain AD and shared secret)
        let x2dh_output_bob: ProtocolOutputBob = X2DHService::from_registry()
            .await
            .map_err(|_| anyhow!("failed to use X2DH service"))?
            .call(ExecuteProtocolAsBob(input_bob))
            .await
            .map_err(|_| anyhow!("failed to execute X2DH protocol"))?
            .map_err(|_| anyhow!("failed to execute X2DH protocol"))?;

        // step 4 - Start a new DR ratchet with Alice using (shared secret, AD) from X2DH output
        let root_chain_key = ChainKey::from(x2dh_output_bob.shared_secret);
        let session_key = SessionKey::from(network_salt::NET_SALT.to_vec().as_slice());
        let session_id = req_data
            .get_dr_session_id()
            .map_err(|e| anyhow!(format!("missing session id: {:?}", e)))?;

        let key = self.pre_key.clone();

        // Bob inits his dr session w Alice with the shared secret and his pkb private key
        let mut dr = DoubleRatchet::new_with_keys(
            session_key,
            root_chain_key,
            key,
            Bytes::from(x2dh_output_bob.ad.as_ref().to_vec()),
            session_id,
        );

        /////////////////////

        // decrypt TypedMessage using the dr session (same logic below for incoming msg in dr session)
        let alice_pub_dr_key = req_data
            .get_sender_dr_pub_key()
            .map_err(|_| anyhow!("invalid sender dr pub key"))?;

        // Bob performs a full ratchet step with Alice's pub dr key per the protocol
        dr.ratchet(&mut OsRng, &alice_pub_dr_key, 0)
            .map_err(|_| anyhow!("invalid dr data"))?;
        let bob_receive_key = dr.get_receiving_key(0)?;
        let typed_message = TypedMessageExtensions::decrypt_msg(
            req_data.message.as_ref().unwrap().enc_typed_msg.as_slice(),
            &bob_receive_key,
            x2dh_output_bob.ad.as_ref(),
        )
        .map_err(|e| anyhow!(format!("invalid enc message: {:?}", e)))?;

        // get ika from message and authenticate the whole request using it and signature
        let ika = typed_message
            .get_ika()
            .map_err(|_| anyhow!("invalid sender key"))?;

        debug!("Caller public id: {}", short_hex_string(ika.as_ref()));

        // verify the whole request
        req_data
            .verify_signature(&ika)
            .map_err(|_| anyhow!("Failed to authenticate message"))?;

        typed_message.verify_signature()?;

        // Store dr session
        DRService::save_dr_session(ika, dr).await?;

        // step 5 - process message

        Ok(self.dispatch_incoming_client_message(typed_message).await?)
    }

    /// Handle a new message in what the sender claims is an existing dr session between him and this client.
    /// Must only be called from SimpleClient Actor handlers
    pub(crate) async fn handle_new_dr_message_from_entity(
        &mut self,
        message: Message,
    ) -> Result<()> {
        debug!("hello :-)");
        let header = message
            .header
            .as_ref()
            .ok_or_else(|| anyhow!("missing header"))?;
        let (mut dr, ika) = DRService::get_dr_session_by_id(header.session_id)
            .await?
            .ok_or_else(|| anyhow!("failed to load dr session"))?;

        // decrypt TypedMessage using the dr session (same logic below for incoming msg in dr session)
        let alice_pub_dr_key = message
            .get_sender_dr_pub_key()
            .map_err(|_| anyhow!("invalid sender dr pub key"))?;

        if header.count == 0 {
            // Bob performs a full ratchet step with Alice's pub dr key per the protocol if send count is 0
            dr.ratchet(&mut OsRng, &alice_pub_dr_key, header.prev_count)
                .map_err(|_| anyhow!("invalid dr data"))?;
        }

        let bob_receive_key = dr.get_receiving_key(header.count)?;
        let typed_message = TypedMessageExtensions::decrypt_msg(
            message.enc_typed_msg.as_slice(),
            &bob_receive_key,
            dr.get_ad()?,
        )
        .map_err(|e| anyhow!(format!("invalid enc message: {:?}", e)))?;

        // get ika from message and authenticate the whole request using it and signature
        let ika_from_message = typed_message
            .get_ika()
            .map_err(|_| anyhow!("invalid sender key"))?;

        debug!(
            "Caller public id: {}",
            short_hex_string(ika_from_message.as_ref())
        );

        if ika != ika_from_message {
            bail!("sender id mismatch between message and stored dr session")
        }

        typed_message.verify_signature()?;
        DRService::save_dr_session(ika, dr).await?;
        Ok(self.dispatch_incoming_client_message(typed_message).await?)
    }
}
