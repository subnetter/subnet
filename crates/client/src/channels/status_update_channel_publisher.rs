// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, Result};
use base::hex_utils::short_hex_string;
use base::snp::snp_core_types::{ChannelData, ContentItem};
use bytes::Bytes;

impl SimpleClient {
    /// Send a ContentItem to all channel subscribers.
    /// ContentItem can be a status update from channel creator or a reply to a status update by any subscriber
    pub(crate) async fn publish_to_status_update_channel(
        &mut self,
        data: &ChannelData,
        content_item: ContentItem,
    ) -> Result<()> {
        let channel_id = content_item.channel_id.clone();
        let author = content_item
            .author
            .as_ref()
            .ok_or_else(|| anyhow!("missing message author"))?;

        let author_id = author.get_id()?.as_slice();
        let channel_owner = self.get_client_entity()?;
        let channel_owner_id = channel_owner.get_id()?.as_slice();

        debug!(
            "publishing status update id {}. Total subscribers: {}",
            content_item.id,
            data.subscribers.len()
        );

        if channel_owner_id != author_id {
            info!(
                "🎉 👋 Status update replay from subscriber, {}: {}. Reply to status id: {}",
                short_hex_string(author_id),
                content_item.get_simple_text_content()?,
                content_item.reply_to,
            );
        }

        for subscriber in data.subscribers.iter() {
            let sub_entity = subscriber
                .user_id
                .as_ref()
                .ok_or_else(|| anyhow!("missing user id"))?;

            let sub_key = sub_entity
                .public_key
                .as_ref()
                .ok_or_else(|| anyhow!("missing pub key"))?
                .as_pub_key()?;

            if author_id == sub_key.as_ref() {
                debug!("skipping sending reply to its author");
                continue;
            }

            debug!(
                "sending update to subscriber: {:}",
                short_hex_string(sub_key.as_ref())
            );

            let message = self
                .new_channel_message(&sub_key, channel_id.as_ref(), content_item.clone())
                .await?;
            let receiver_id = Bytes::from(subscriber.get_subscriber_id()?.clone());
            self.send_typed_message(message, receiver_id).await?;
        }

        Ok(())
    }
}
