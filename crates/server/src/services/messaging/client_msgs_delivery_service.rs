// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, bail, Result};
use base::typed_msgs_dispatcher::{
    Subscribe, TypedMessageHandler, TypedMessagesDispatcher, Unsubscribe,
};
use chrono::prelude::*;
use prost::Message;
use xactor::*;

use crate::clients_data::service::ClientsDataService;
use base::snp::snp_server_api::{
    DeliverClientMessagesRequest, DeliverClientMessagesResponse, MessageType, TypedMessage,
};

/// ClientMessagesDeliveryService is a service which handles DeliverClientMessagesRequest client messages.
/// A client send to this provider a DeliverClientMessagesRequest with a payment and a set of message metadata it wishes to receive.
/// Provider should verify the payment, send the messages to the client and remove them and their meta-data from its store.
#[derive(Debug, Default)]
pub(crate) struct ClientMessagesDeliveryService {}
impl Service for ClientMessagesDeliveryService {}

#[async_trait::async_trait]
impl Actor for ClientMessagesDeliveryService {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        // subscribe to DeliverClientMessagesRequest incoming messages
        let subscribe_msg = Subscribe {
            message_type: MessageType::DeliverClientMessagesRequest as i32,
            subscriber: ctx.address().caller(),
        };

        let dispatcher = TypedMessagesDispatcher::from_registry().await.unwrap();
        dispatcher.call(subscribe_msg).await??;
        debug!(
            "ClientMessagesDeliveryService started and subscribed to handle RouteMessageRequests"
        );
        Ok(())
    }

    async fn stopped(&mut self, _ctx: &mut Context<Self>) {
        // Unsubscribe from the dispatcher
        let dispatcher = TypedMessagesDispatcher::from_registry().await.unwrap();
        let _res = dispatcher
            .call(Unsubscribe {
                id: MessageType::DeliverClientMessagesRequest as i32,
            })
            .await;
    }
}

/// Handle a RouteMessageRequest form a served client
#[async_trait::async_trait]
impl Handler<TypedMessageHandler> for ClientMessagesDeliveryService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: TypedMessageHandler,
    ) -> Result<TypedMessage> {
        // Step 1 - verify we know how to handle the message
        if msg.0.msg_type != (MessageType::DeliverClientMessagesRequest as i32) {
            debug!("Unexpected message type");
            return Err(anyhow!("Unexpected message type {}", msg.0.msg_type));
        };

        // Step 2 - Verify that we are serving this client before processing the message!!!!
        let ika = msg
            .0
            .get_ika()
            .map_err(|_| anyhow!("missing sender from msg"))?;

        if ClientsDataService::get_client_service_data(&ika)
            .await?
            .is_none()
        {
            bail!("unrecognized client - not served by this provider")
        }

        // Step 3 - verify that the request is a DeliverClientMessagesRequest
        let req: DeliverClientMessagesRequest =
            DeliverClientMessagesRequest::decode(msg.0.message.as_slice())
                .map_err(|e| anyhow!("failed to decode DeliverClientMessagesRequest: {:?}", e))?;

        // Step 4 - todo: verify the payment - implement me
        let payment = req.payment.ok_or_else(|| anyhow!("missing payment data"))?;

        // Step 5: load messages from store deliver the messages to the client in a response

        let messages = ClientsDataService::load_client_messages(payment.item_ids.clone()).await?;

        // Step 6 - delete the messages and messages meta-data from store - note that responding may fail.
        // Consider handling Ack from client that he got messages in order to delete them from the db or
        // schedule to delete them later. They have a ttl anyhow but removing them sooner will save some storage.
        // With the following, messages will be lost if connection goes down before client is able to get all messages
        // from the response message.
        ClientsDataService::delete_client_messages(&ika, payment.item_ids).await?;

        // Create and return response with messages
        let resp = DeliverClientMessagesResponse {
            receipt_id: 0,
            messages,
        };
        let mut buff = Vec::with_capacity(resp.encoded_len());
        resp.encode(&mut buff)?;

        Ok(TypedMessage {
            time_stamp: Utc::now().timestamp_nanos() as u64,
            msg_type: MessageType::DeliverClientMessagesResponse as i32,
            message: buff,
            receiver: None,
            sender: None,
            signature: None,
        })
    }
}
