//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::messaging::messaging_service::ServerMessagingService;
use crate::services::messaging::messaging_service_new_msg::IncomingMessageContext;
use crate::services::provider_id::ProviderIdService;
use crate::services::provider_id_service::{GetCurrentIdentityBundle, GetIdentityBundle};
use anyhow::Result;
use base::api_types_extensions::Signed;
use base::hex_utils::short_hex_string;
use base::snp::snp_core_types::{EntityId, PrivateProviderIdentityBundle, PublicKey};
use base::snp::snp_server_api;
use base::snp::snp_server_api::{DrSessionHeader, Message, MessageType};
use base::typed_msgs_dispatcher::{Publish, TypedMessagesDispatcher};
use common::dr_service::DRService;
use common::typed_msg_extensions::TypedMessageExtensions;
use tonic::Status;
use xactor::Service;

/// MyMessagingService helpers - should be private if rust would allow it
/// as they are designed to only be used by MyMessagingService actor handlers.
/// The helpers here have no state, change no state of other entities and are concurrency safe
impl ServerMessagingService {
    /// process a new incoming msg and return the response ms or an error.
    /// Used both by new_session and new_message server methods as all logic is shared
    /// in these 2 flows. Note that this function will update the dr session if enc was ok and will store
    /// in the db the updated dr session
    pub async fn process_incoming_msg(mut context: IncomingMessageContext) -> Result<Message> {
        // Dispatch the message to obtain a result from a message handler (app logic)

        let msg_type = MessageType::from_i32(context.msg.msg_type).unwrap();

        debug!(
            "Publishing incoming message type id {:?} to the dispatcher...",
            msg_type
        );

        let mut resp_msg = TypedMessagesDispatcher::from_registry()
            .await
            .map_err(|e| {
                Status::internal(format!("internal error - failed to find dispatcher: {}", e))
            })?
            .call(Publish(context.msg))
            .await
            .map_err(|e| {
                Status::internal(format!("internal error - failed to call dispatcher: {}", e))
            })?
            .map_err(|e| {
                Status::internal(format!(
                    "internal error - failed to get typed msg response: {}",
                    e
                ))
            })?;

        debug!("Got message response back from messages dispatcher");

        // Add sender and receiver to response
        resp_msg.sender = Some(EntityId {
            public_key: Some(PublicKey {
                key: context.ikb_pair.public.as_ref().to_vec(),
            }),
            nickname: "".to_string(),
        });

        resp_msg.receiver = Some(EntityId {
            public_key: Some(PublicKey {
                key: context.ika.as_ref().to_vec(),
            }),
            nickname: "".to_string(),
        });

        resp_msg.sign(&context.ikb_pair)?;

        // The key to encrypt response message to Alice
        let bob_send_key = context.dr.next_sending_key().unwrap();

        let ad = context
            .dr
            .ad
            .as_ref()
            .ok_or_else(|| Status::internal("missing ad from dr session"))?;

        debug!(
            ">>> bob dr send key: {}",
            short_hex_string(bob_send_key.1.as_bytes())
        );

        // Bob's current public dr key - to be sent in the message header to alice
        let bob_pub_ratchet_key = context.dr.get_public_key().unwrap();

        debug!(
            "bob pub ratchet key: {}",
            short_hex_string(bob_pub_ratchet_key.as_bytes())
        );

        let session_id = context.dr.session_id;

        // Encode the message to the caller (encrypt inner type message) using bob's sending key
        let enc_msg = TypedMessageExtensions::encrypt_msg(resp_msg, &bob_send_key.1, ad.as_ref())
            .map_err(|_| Status::internal("failed to encrypt message"))?;

        // Save the dr session with alice (ika) so it can be used later
        // and only after we updated it to give us the keys (above) and only after we were able to decrypt
        // with it (this ensures alice is on the other side of this ratchet)

        // Save the updated dr session using the dr server

        DRService::save_dr_session(context.ika, context.dr).await?;

        // prepare and return Message to caller to be sent back to remote request caller
        Ok(snp_server_api::Message {
            header: Some(DrSessionHeader {
                session_id,
                dr_pub_key: Some(PublicKey {
                    key: bob_pub_ratchet_key.as_bytes().to_vec(),
                }),
                prev_count: 0, // todo: get to the bottom of this - what should be sent here?
                count: bob_send_key.0,
            }),
            enc_typed_msg: enc_msg.to_vec(),
        })
    }

    /// Get provider id bundle by id. Return status if fails to get it
    pub async fn get_provider_id_bundle(
        bundle_id: u64,
    ) -> Result<PrivateProviderIdentityBundle, Status> {
        let provider = match ProviderIdService::from_registry().await {
            Ok(addr) => addr,
            Err(_err) => return Err(Status::internal("can't locate provider id bundle")),
        };

        let bundle = match provider.call(GetIdentityBundle(bundle_id)).await {
            Ok(res) => match res {
                Ok(opt) => match opt {
                    Some(bundle) => bundle,
                    None => return Err(Status::not_found("unrecognized receiver bundle id")),
                },
                Err(_err) => return Err(Status::internal("error trying to get bundle by id")),
            },
            Err(_err) => return Err(Status::internal("error calling get bundle by id")),
        };

        Ok(bundle)
    }

    pub async fn get_curr_provider_id_bundle() -> Result<PrivateProviderIdentityBundle, Status> {
        let bundle = ProviderIdService::from_registry()
            .await
            .map_err(|e| Status::internal(format!("failed to get provider service: {:?}", e)))?
            .call(GetCurrentIdentityBundle {})
            .await
            .map_err(|e| Status::internal(format!("failed to call provider service: {:?}", e)))?
            .map_err(|e| Status::internal(format!("provider service error: {:?}", e)))?;

        Ok(bundle)
    }
}
