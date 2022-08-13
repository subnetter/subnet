// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::Result;
use base::hex_utils::short_hex_string;
use base::snp::snp_client_to_client::NewChannelMessageRequest;
use base::snp::snp_core_types::ContentItem;
use base::snp::snp_server_api::{MessageType, TypedMessage};

impl SimpleClient {
    /// Creates a new status update request in a status updates channel or a group message in a group
    /// User by clients to request channel creator or group admin to post replies or group messages
    pub(crate) async fn new_channel_message_request(
        &self,
        to: &ed25519_dalek::PublicKey,
        content_item: ContentItem,
    ) -> Result<TypedMessage> {
        // This is the simple status update or a group message

        let channel_id = content_item.channel_id.clone();

        debug!("Channel id: {:?}", short_hex_string(channel_id.as_ref()));

        let inner_msg = NewChannelMessageRequest {
            content_item: Some(content_item),
        };

        use prost::Message;
        let mut buff = Vec::with_capacity(inner_msg.encoded_len());
        inner_msg.encode(&mut buff).unwrap();

        // this is the status update message
        let typed_msg = self.create_typed_message(MessageType::ChannelMessageRequest, buff, *to)?;

        Ok(typed_msg)
    }
}
