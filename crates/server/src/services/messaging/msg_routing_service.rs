//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, bail, Result};
use base::typed_msgs_dispatcher::{
    Subscribe, TypedMessageHandler, TypedMessagesDispatcher, Unsubscribe,
};
use bytes::Bytes;
use chrono::prelude::*;
use prost::Message;
use xactor::*;

use crate::clients_data::service::ClientsDataService;
use crate::services::server_to_server::server_to_server_service::{
    SendMessageToServer, ServerToServerService,
};
use base::hex_utils::short_hex_string;
use base::snp::snp_server_api::{
    MessageType, RouteMessageRequest, RouteMessageResponse, TypedMessage,
};

/// MessageRoutingService is a service which handles RouteMessageRequest requests.
/// A client send to this provider a RouteMessageRequest that it wants to route to another provider.
/// This is use in the core client-to-client messaging core algorithm of SNP.
/// Note that is not an internal messages router / dispatcher. It is designed for handling remote route requests.
#[derive(Debug, Default)]
pub struct MessageRoutingService {}
impl Service for MessageRoutingService {}

#[async_trait::async_trait]
impl Actor for MessageRoutingService {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        // subscribe to RouteMessageRequest incoming messages
        let subscribe_msg = Subscribe {
            message_type: MessageType::RouteMessageRequest as i32,
            subscriber: ctx.address().caller(),
        };

        let dispatcher = TypedMessagesDispatcher::from_registry().await.unwrap();
        dispatcher.call(subscribe_msg).await??;
        debug!("RouteMessageService started and subscribed to handle RouteMessageRequests");
        Ok(())
    }

    async fn stopped(&mut self, _ctx: &mut Context<Self>) {
        // Unsubscribe from the dispatcher
        let dispatcher = TypedMessagesDispatcher::from_registry().await.unwrap();
        let _res = dispatcher
            .call(Unsubscribe {
                id: MessageType::RouteMessageRequest as i32,
            })
            .await;
    }
}

/// Handle a RouteMessageRequest form a served client
#[async_trait::async_trait]
impl Handler<TypedMessageHandler> for MessageRoutingService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: TypedMessageHandler,
    ) -> Result<TypedMessage> {
        // Step 1 - verify we know how to handle the message
        if msg.0.msg_type != (MessageType::RouteMessageRequest as i32) {
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

        // Step 3 - verify that the request is a ForwardMessageRequest
        let route_msg_req: RouteMessageRequest =
            RouteMessageRequest::decode(msg.0.message.as_slice())
                .map_err(|e| anyhow!("failed to decode RouteMessageRequest: {:?}", e))?;

        // Step 4 - create and send SB a forward_msg_req message
        let forward_msg_req = route_msg_req
            .forward_message
            .ok_or_else(|| anyhow!("missing forward msg request"))?;

        let mut buff: Vec<u8> = Vec::with_capacity(forward_msg_req.encoded_len());
        forward_msg_req.encode(&mut buff)?;

        let server_to_server_service = ServerToServerService::from_registry()
            .await
            .map_err(|_| anyhow!("failed to get s2s service"))?;

        // Get receiver id - this should be another service provider
        let receiver = forward_msg_req
            .receiver
            .ok_or_else(|| anyhow!("missing receiver service provider SPB"))?;

        let receiver_id = receiver
            .public_key
            .ok_or_else(|| anyhow!("missing pub key"))?
            .as_pub_key()?;

        let dialup_info = route_msg_req
            .dialup_info
            .as_ref()
            .ok_or_else(|| anyhow!("missing dialup info"))?
            .clone();

        debug!(
            "got a request to forward client message to provider {}:{} with id: {:?}",
            dialup_info.ip_address,
            dialup_info.port,
            short_hex_string(receiver_id.to_bytes().as_ref())
        );

        let resp: TypedMessage = server_to_server_service
            .call(SendMessageToServer {
                dialup_info,
                receiver_id,
                message_type: MessageType::ForwardMessageRequest,
                message: Bytes::from(buff),
            })
            .await
            .map_err(|e| anyhow!("(*) internal error - failed to call: {:?}", e))?
            .map_err(|e| anyhow!("(**) internal error - failed to call: {:?}", e))?;

        // verify we got a response indicating message was forwarded
        if resp.msg_type != MessageType::ForwardMessageResponse as i32 {
            bail!("unexpected response from remote provider")
        }

        debug!("got a response from other server - returning to client a RouteMessageResponse");

        // Create and return response to client
        let resp = RouteMessageResponse {};
        let mut buff = Vec::with_capacity(resp.encoded_len());
        resp.encode(&mut buff)?;

        Ok(TypedMessage {
            time_stamp: Utc::now().timestamp_nanos() as u64,
            msg_type: MessageType::RouteMessageResponse as i32,
            message: buff,
            receiver: None,
            sender: None,
            signature: None,
        })
    }
}
