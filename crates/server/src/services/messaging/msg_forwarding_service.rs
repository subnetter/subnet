//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::clients_data::service::ClientsDataService;
use crate::services::clients_service::{ClientsService, SendMessageToClient};
use crate::services::provider_id::ProviderIdService;
use crate::services::provider_id_service::GetIdentityBundle;
use anyhow::{anyhow, bail, Result};
use base::hex_utils::short_hex_string;
use base::snp::snp_core_types::PrivateProviderIdentityBundle;
use base::snp::snp_server_api::{
    ClientMessagesMetadata, ForwardMessagePayload, ForwardMessageRequest, ForwardMessageResponse,
    MessageType, TypedMessage,
};
use base::typed_msgs_dispatcher::{
    Subscribe, TypedMessageHandler, TypedMessagesDispatcher, Unsubscribe,
};
use bytes::Bytes;
use chrono::prelude::*;
use common::aead::AEAD;
use xactor::*;

/// MessageForwardingService is a system service which handles ForwardMessageRequests.
/// A client (via its service provider) send to this provider a ForwardMessageRequest that it wants to route to another provider.
/// This is use in the core client-to-client messaging core algorithm of SNP.
/// Note that is not an internal messages router / dispatcher. It is designed for handling incoming net messages only.
#[derive(Debug, Default)]
pub struct MessageForwardingService {}
impl Service for MessageForwardingService {}

#[async_trait::async_trait]
impl Actor for MessageForwardingService {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        // subscribe to ForwardMessageRequest incoming messages
        let subscribe_msg = Subscribe {
            message_type: MessageType::ForwardMessageRequest as i32,
            subscriber: ctx.address().caller(),
        };

        let dispatcher = TypedMessagesDispatcher::from_registry().await.unwrap();
        dispatcher.call(subscribe_msg).await??;
        debug!("ForwardMessageService started and subscribed to handle ForwardMessageRequests");
        Ok(())
    }

    async fn stopped(&mut self, _ctx: &mut Context<Self>) {
        // Unsubscribe from the dispatcher
        let dispatcher = TypedMessagesDispatcher::from_registry().await.unwrap();
        let _res = dispatcher
            .call(Unsubscribe {
                id: MessageType::ForwardMessageRequest as i32,
            })
            .await;
    }
}

/// Handle a ForwardMessageRequest form a served client
#[async_trait::async_trait]
impl Handler<TypedMessageHandler> for MessageForwardingService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: TypedMessageHandler,
    ) -> Result<TypedMessage> {
        debug!("(1)");

        // Step 1 - verify we know how to handle the incoming message (defensive)
        if msg.0.msg_type != (MessageType::ForwardMessageRequest as i32) {
            debug!("Unexpected message type");
            return Err(anyhow!("Unexpected message type {}", msg.0.msg_type));
        };

        // Step 2 - get the request from the payload
        let forward_msg_req: ForwardMessageRequest =
            ForwardMessageRequest::decode(msg.0.message.as_slice())
                .map_err(|e| anyhow!("failed to decode ForwardMessageRequest: {:?}", e))?;

        debug!("(1a)");

        // Use an identity bundle as requested by caller
        let provider_id_server = ProviderIdService::from_registry().await?;
        let bob_bundle: PrivateProviderIdentityBundle = provider_id_server
            .call(GetIdentityBundle(forward_msg_req.receiver_bundle_id))
            .await??
            .ok_or_else(|| anyhow!("unrecognized bundle"))?;

        debug!("(1b)");

        // prepare data for x2dh execution
        let eka = forward_msg_req
            .sender_ephemeral_key
            .ok_or_else(|| anyhow!("missing eka in request"))?
            .as_x25519_pub_key()?;
        let pre_key_public = bob_bundle
            .public_bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing public bundle"))?
            .pre_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing pre key"))?;
        let pre_key_pub_data = pre_key_public
            .key
            .as_ref()
            .ok_or_else(|| anyhow!("missing key data"))?;

        let pre_key_info = pre_key_pub_data.as_x25519_pub_key()?;
        let pkb_private = bob_bundle.get_prekey_as_static_secret()?;
        let shared_secret = pkb_private.diffie_hellman(&eka);

        // Step 4 - the forward_msg_req payload is encrypted for this provider.
        // decrypt it to obtain the inner message designated for one of this provider served clients
        // Perform an eph-dh with sender (using his eph key) and our bundle - do this in another method
        // to obtain the inner message (new session or message to another entity)
        let ad = common::edh::compute_ad(&eka, &pre_key_info);

        debug!("%%%% ad: {}", short_hex_string(ad.as_ref()));
        debug!(
            "%%%% shared secret: {}",
            short_hex_string(shared_secret.to_bytes().as_ref())
        );

        let payload_bytes = AEAD::decrypt(
            forward_msg_req.enc_payload.as_ref(),
            &shared_secret.to_bytes(),
            ad.as_ref(),
        )?;

        use prost::Message;
        let payload: ForwardMessagePayload = ForwardMessagePayload::decode(payload_bytes.as_ref())?;

        // step 5 - verify that this provider is serving the designated receiver
        let ika = payload.get_receiver_pub_key()?;
        if ClientsDataService::get_client_service_data(&ika)
            .await?
            .is_none()
        {
            bail!("unrecognized client - not served by this provider")
        }

        // Step 6 - Store message and message metadata for client

        let data = payload
            .dr_message
            .ok_or_else(|| anyhow!("missing payload data"))?;

        debug!(
            "Storing a message to client: {:?}",
            short_hex_string(ika.as_ref())
        );

        let msg_meta_data = ClientsDataService::store_new_message_for_client(ika, data).await?;

        let forwarded_msg = ClientMessagesMetadata {
            messages_metadata: vec![msg_meta_data],
        };
        let mut buff: Vec<u8> = Vec::with_capacity(forwarded_msg.encoded_len());
        forwarded_msg.encode(&mut buff)?;

        // Attempt to send the metadata to the client but don't fail on error.
        // In case there is not connection with client the meta-data about the message will be sent to the client next time he connects.
        let _ = ClientsService::send_message_to_client(SendMessageToClient {
            client_id: ika,
            message_type: MessageType::ClientMessagesMetadata,
            message: Bytes::from(buff),
        })
        .await;

        // Step 7 - create and return response to the forwarding provider to ack we got it and going to forward to the designated client
        let resp = ForwardMessageResponse {};
        let mut buff = Vec::with_capacity(resp.encoded_len());
        resp.encode(&mut buff)?;

        Ok(TypedMessage {
            time_stamp: Utc::now().timestamp_nanos() as u64,
            msg_type: MessageType::ForwardMessageResponse as i32,
            message: buff,
            receiver: None,
            sender: None,
            signature: None,
        })
    }
}
