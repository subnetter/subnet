// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;

use anyhow::{anyhow, bail, Result};
use base::snp::snp_payments::Payment;
use base::snp::snp_server_api::dr_message::Data;
use base::snp::snp_server_api::{
    ClientMessagesMetadata, DeliverClientMessagesRequest, DeliverClientMessagesResponse, DrMessage,
    MessageType, TypedMessage,
};
use chrono::prelude::*;
use tonic::Streaming;
use xactor::*;

impl SimpleClient {
    pub(crate) async fn provider_messages_handler(mut stream: Streaming<DrMessage>) {
        debug!("setup process for receiving new message from provider");
        loop {
            match stream.message().await {
                Ok(res) => match res {
                    Some(message) => {
                        // We use client actor here to ensure serialized access to state
                        let client = SimpleClient::from_registry().await.unwrap();
                        let res = client.call(IncomingDrMessage(message)).await.unwrap();
                        if res.is_err() {
                            error!(
                                "error handling new message from provider: {:?}",
                                res.err().unwrap()
                            )
                        }
                    }
                    None => debug!("expected a dr message"),
                },
                Err(e) => error!("error getting dr message from stream: {:?}", e),
            }
        }
    }
}

#[message(result = "Result<()>")]
pub struct IncomingDrMessage(DrMessage);

#[async_trait::async_trait]
impl Handler<IncomingDrMessage> for SimpleClient {
    /// Safely handle new incoming messages from server streamed into this client
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: IncomingDrMessage) -> Result<()> {
        debug!("got a new incoming message from provider over stream...");

        let message = match msg.0.data.ok_or_else(|| anyhow!("missing data"))? {
            Data::Message(msg) => msg,
            Data::NewSessionRequest(_) => bail!("unexpected message from provider"),
        };

        let typed_message = SimpleClient::decode_incoming_dr_message(message).await?;

        let msg_type = MessageType::from_i32(typed_message.msg_type)
            .ok_or_else(|| anyhow!("Unrecognized message type"))?;

        match msg_type {
            // MessageType::ForwardedMessage => self.handle_forwarded_msg(typed_message).await,
            MessageType::ClientMessagesMetadata => {
                self.handle_messages_metadata(typed_message).await
            }
            _ => bail!("unsupported message from provider"),
        }
    }
}

impl SimpleClient {
    /// Handle metadata about new messages that provider has for this client sent by this client's provider
    async fn handle_messages_metadata(&mut self, msg: TypedMessage) -> Result<()> {
        use prost::Message;
        let meta_data: ClientMessagesMetadata =
            ClientMessagesMetadata::decode(msg.message.as_slice())?;

        // this is meta-data about a new message provider has for this client
        // client can decide which messages to query and need to make an L2 payment to get them
        // Currently, we just request all these messages from our provider without providing a payment
        // todo: integrate with payments

        let ids: Vec<u64> = meta_data.messages_metadata.iter().map(|v| v.id).collect();

        debug!("got messages metadata pushed from provider: {:?}", ids);

        let payment = Payment {
            time_stamp: Utc::now().timestamp_nanos() as u64,
            item_ids: ids,
            user_id: vec![],
            provider_id: vec![],
            amount: None,
            signature: vec![],
        };

        // todo: sign the payment

        // todo: create payment with ids here, sign it and add to request

        // Create a request to deliver messages with payment receipt
        let req = DeliverClientMessagesRequest {
            payment: Some(payment),
        };
        let mut buff: Vec<u8> = Vec::with_capacity(req.encoded_len());
        req.encode(&mut buff)?;

        // Send the request to the server and get the pending message(s)
        let resp = self
            .send_message_to_provider(MessageType::DeliverClientMessagesRequest, buff)
            .await?;

        if resp.msg_type != MessageType::DeliverClientMessagesResponse as i32 {
            bail!("unexpected response type")
        }

        let delivery_resp: DeliverClientMessagesResponse =
            DeliverClientMessagesResponse::decode(resp.message.as_slice())?;

        debug!("dispatching new message(s) from provider");

        // Go over the response message and send them to the appropriate handler for processing
        for msg in delivery_resp.messages {
            let data = msg.data.ok_or_else(|| anyhow!("missing message data"))?;
            match data {
                Data::Message(msg) => self.handle_new_dr_message_from_entity(msg).await?,
                Data::NewSessionRequest(msg) => {
                    self.handle_new_session_req_from_entity(msg).await?
                }
            };
        }

        Ok(())
    }
}
