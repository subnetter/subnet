// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::Result;
use base::api_types_extensions::SignedWithExternalVerifier;
use base::hex_utils::short_hex_string;
use base::snp::snp_client_to_client::NewChannelMessage;
use base::snp::snp_core_types::{ChannelContentItem, ContentItem};
use base::snp::snp_server_api::{MessageType, TypedMessage};

impl SimpleClient {
    /// Creates a new status update in a status updates channel or a group message in a group
    /// This can be used by any client, not just channel creator
    pub(crate) async fn new_channel_message(
        &self,
        to: &ed25519_dalek::PublicKey,
        channel_id: &[u8],
        content_item: ContentItem,
    ) -> Result<TypedMessage> {
        // This is the simple status update or a group message
        debug!("Channel id: {:?}", short_hex_string(channel_id));
        let mut channel_content_item = ChannelContentItem::new(content_item);
        channel_content_item.sign(&self.client_id)?;

        let inner_msg = NewChannelMessage {
            content_item: Some(channel_content_item),
        };

        use prost::Message;
        let mut buff = Vec::with_capacity(inner_msg.encoded_len());
        inner_msg.encode(&mut buff).unwrap();

        // this is the status update message
        let typed_msg = self.create_typed_message(MessageType::ChannelMessage, buff, *to)?;

        Ok(typed_msg)
    }
}
