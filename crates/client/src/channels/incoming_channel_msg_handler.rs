// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, bail, Result};
use base::api_types_extensions::SignedWithExternalVerifier;
use base::hex_utils::short_hex_string;
use base::snp::snp_client_to_client::NewChannelMessage;
use base::snp::snp_core_types::ChannelType;
use base::snp::snp_server_api::TypedMessage;

impl SimpleClient {
    /// Handle an incoming status update or group message sent by channel creator
    pub(crate) async fn handle_new_incoming_channel_message(
        &self,
        msg: TypedMessage,
    ) -> Result<()> {
        use prost::Message;
        let channel_message: NewChannelMessage = NewChannelMessage::decode(msg.message.as_slice())
            .map_err(|e| anyhow!("failed to decode channel message: {:?}", e))?;

        let channel_creator = msg
            .sender
            .as_ref()
            .ok_or_else(|| anyhow!("missing channel creator id"))?
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing public key"))?
            .as_pub_key()
            .map_err(|_| anyhow!("invalid public key"))?;

        let content = channel_message
            .content_item
            .ok_or_else(|| anyhow!("missing update content"))?;

        content.verify_signature(&channel_creator)?;

        let item = content
            .content_item
            .ok_or_else(|| anyhow!("missing content"))?;
        let sender = msg.get_ika()?;

        let channel_id = item.channel_id.clone();

        let channel_bundle = self
            .channels_subscriptions
            .get(&channel_id)
            .ok_or_else(|| anyhow!("not subscribed to this channel"))?;

        match channel_bundle.channel_type {
            t if t == ChannelType::Group as i32 => info!(
                "🎉 👋 incoming group message from, {}: {}. Reply to {}. Id: {}",
                short_hex_string(sender.as_ref()),
                item.get_simple_text_content()?,
                item.reply_to,
                item.id
            ),
            t if t == ChannelType::StatusFeed as i32 => info!(
                "🎉 👋 incoming status update from, {}: {}. Reply to: {}. Id: {}",
                short_hex_string(sender.as_ref()),
                item.get_simple_text_content()?,
                item.reply_to,
                item.id
            ),
            _ => bail!("unsupported channel type"),
        }

        Ok(())
    }
}
