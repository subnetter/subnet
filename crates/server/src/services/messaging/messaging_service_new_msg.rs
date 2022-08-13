//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::messaging::messaging_service::ServerMessagingService;
use anyhow::Result;
use base::hex_utils::short_hex_string;
use base::snp::snp_server_api::{Message, TypedMessage};
use common::dr_service::{DRService, GetSessionById};
use common::typed_msg_extensions::TypedMessageExtensions;
use double_ratchet::dr::DoubleRatchet;
use rand_core::OsRng;
use tonic::Status;
use xactor::Service;

/// An incoming decrypted message and its context
pub struct IncomingMessageContext {
    pub dr: DoubleRatchet,                // dr session to use to encrypt response
    pub msg: TypedMessage,                // the new message
    pub ikb_pair: ed25519_dalek::Keypair, // this provider bob's id keypair
    pub ika: ed25519_dalek::PublicKey,    // alice (new message sender) public id
}

/// MyMessagingService new_message api method implementation
/// Other entities use this method to send a new message to this provider
/// using an existing dr session with it
/// Note that caller is responsible to save the updated DR session used for decoding as this method
/// does not persist it
impl ServerMessagingService {
    pub async fn new_message_handler(
        &self,
        message: Message,
    ) -> Result<IncomingMessageContext, Status> {
        let header = message
            .header
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing header"))?;

        let bundle = ServerMessagingService::get_curr_provider_id_bundle().await?;

        // step 1 - attempt to load the dr session between sender and the receiver by id
        // Alice the caller requesting to use an existing dr session with it.
        // We try to load it from storage by id to get Alice's id we stored wth it last time we used it

        let dr_service = DRService::from_registry()
            .await
            .map_err(|_| Status::internal("failed to get provider service"))?;

        let dr_session = dr_service
            .call(GetSessionById(header.session_id))
            .await
            .map_err(|_| Status::internal("internal error - failed to call"))?
            .map_err(|_| Status::internal("internal error - failed to call"))?
            .ok_or_else(|| {
                Status::invalid_argument(format!(
                    "could not find dr session by session id: {}",
                    header.session_id
                ))
            })?;

        let mut dr: DoubleRatchet = dr_session.0;

        let alice_pub_dr_key = message
            .get_sender_dr_pub_key()
            .map_err(|_| Status::invalid_argument("invalid provided sender public dr key"))?;

        debug!(
            ">>> alice pub dr key: {}",
            short_hex_string(alice_pub_dr_key.as_bytes())
        );

        // Bob should only perform full ratchet if sending key count is 0
        // otherwise he already did a ratchet with alice and should just advance is receiving key
        let header = message.header.unwrap();
        let index = header.count;
        debug!("sending key index: {}", index);
        if index == 0 {
            // Bob performs a full ratchet step with Alice's pub dr key per the protocol
            dr.ratchet(&mut OsRng, &alice_pub_dr_key, header.prev_count)
                .map_err(|e| Status::internal(format!("ratchet failed: {:?}", e)))?;
        } else {
            debug!("@@@ not doing ratchet index != 0");
        }

        // The requested message decryption key (compare counter with message)
        let bob_receive_key = dr
            .get_receiving_key(index)
            .map_err(|e| Status::internal(format!("failed to get DR receiving key: {:?}", e)))?;

        debug!(
            ">>> bob dr receiver key: [{}]: {}",
            index,
            short_hex_string(bob_receive_key.as_bytes())
        );

        let ad = dr
            .ad
            .as_ref()
            .ok_or_else(|| Status::internal("missing ad from dr session"))?;

        debug!(">>> bob AD: {}", short_hex_string(ad.to_vec().as_ref()));

        // decrypt and authenticate the inner message
        let typed_message = TypedMessageExtensions::decrypt_msg(
            message.enc_typed_msg.as_slice(),
            &bob_receive_key,
            ad,
        )
        .map_err(|e| Status::invalid_argument(format!("failed to decode dr message: {:?}", e)))?;

        let ika = typed_message
            .get_ika()
            .map_err(|_| Status::invalid_argument("invalid public key"))?;

        debug!("Caller public id: {}", short_hex_string(ika.as_ref()));

        if ika != dr_session.1 {
            // Signer of typed message is not the the same
            // as creator of this dr session
            return Err(Status::invalid_argument("sender id mismatch"));
        }

        // Save the updated dr session using the dr service as we no longer use it.
        // it will be used to encrypt messages sent on the stream

        debug!("saving dr session...");
        DRService::save_dr_session(ika, dr.clone())
            .await
            .map_err(|e| Status::internal(format!("failed to save dr session: {:?}", e)))?;

        let ikb_pair = bundle
            .provider_id_keypair
            .as_ref()
            .unwrap()
            .to_ed2559_kaypair();

        Ok(IncomingMessageContext {
            dr,
            msg: typed_message,
            ikb_pair,
            ika,
        })
    }
}
