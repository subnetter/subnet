// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{bail, Result};
use base::snp::snp_server_api::{MessageType, TypedMessage};

impl SimpleClient {
    /// App-level processing of messages sent by another client to this client
    /// Must only be called from a SimpleClient actor message handler or a method called from an actor message handler.
    pub(crate) async fn dispatch_incoming_client_message(
        &mut self,
        msg: TypedMessage,
    ) -> Result<()> {
        match msg.msg_type {
            t if t == MessageType::TextMessageRequest as i32 => self.handle_text_message(msg).await,

            t if t == MessageType::ChannelMessage as i32 => {
                self.handle_new_incoming_channel_message(msg).await
            }
            t if t == MessageType::ChannelSubscribeRequest as i32 => {
                // handles both status updates and group join requests
                self.handle_subscribe_to_channel_message(msg).await
            }

            t if t == MessageType::ChannelSubscribeResponse as i32 => {
                // unsubscribe from channel or leave group
                self.handle_subscribe_response_message(msg).await
            }

            t if t == MessageType::ChannelUnsubscribeRequest as i32 => {
                // unsubscribe from channel or leave group
                self.handle_unsubscribe_from_channel_message(msg).await
            }

            t if t == MessageType::ChannelUnsubscribeResponse as i32 => {
                // unsubscribe from channel or leave group
                self.handle_unsubscribe_response_message(msg).await
            }

            // Request to a creator client from a provided client to post a message
            // to status update or group
            t if t == MessageType::ChannelMessageRequest as i32 => {
                self.handle_channel_message_request(msg).await
            }

            t if t == MessageType::BuyItemRequest as i32 => self.handle_buy_item_request(msg).await,

            t if t == MessageType::ListPaidItemsRequest as i32 => {
                self.handle_list_items_request(msg).await
            }

            t if t == MessageType::ListPaidItemsResponse as i32 => {
                self.handle_incoming_items_list(msg).await
            }

            t if t == MessageType::BuyItemResponse as i32 => self.handle_incoming_item(msg).await,
            _ => bail!("received an unsupported message of type: {}", msg.msg_type),
        }
    }
}
