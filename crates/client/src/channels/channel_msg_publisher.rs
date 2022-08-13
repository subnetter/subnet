// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::channels::channels_data_service::{ChannelsService, GetChannel};
use crate::simple_client::SimpleClient;
use anyhow::{anyhow, bail, Result};
use base::api_types_extensions::Signed;
use base::snp::snp_core_types::{ChannelData, ChannelType, ContentItem, EntityId};
use xactor::*;

/// Simple text only channel message
#[message(result = "Result<u64>")]
pub(crate) struct PublishNewChannelMessage {
    pub(crate) channel_id: EntityId,
    pub(crate) text: String,
    pub(crate) reply_to: u64,
}

/// Publish a new status update, a reply to an update or a group message to a channel.
/// Supports sending a status update or a group message by status updates or group creator,
/// sending reply to a status update channel this client is subscribed to,
/// and sending a group message in a group that this client is member of.
#[async_trait::async_trait]
impl Handler<PublishNewChannelMessage> for SimpleClient {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: PublishNewChannelMessage,
    ) -> Result<u64> {
        // for now this method only deal with messages sent by group or channel creator
        let channel_id = msg
            .channel_id
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing pub key"))?
            .key
            .clone();

        let me = self.get_client_entity()?;
        let mut content_item =
            ContentItem::new_channel_text_message(msg.text, me, channel_id.clone(), msg.reply_to);
        content_item.reply_to = msg.reply_to;
        content_item.sign(&self.client_id)?;
        let message_id = content_item.id;

        let bundle = self.channels_subscriptions.get(&channel_id);

        // This is a channel that this client is subscribed to - it was created by another client
        if bundle.is_some() {
            let data = bundle.unwrap().clone();
            debug!("publish new status update reply or a group message as subscriber or member...");
            self.send_channel_message_request(data, content_item)
                .await?;
            return Ok(message_id);
        }

        let channels_service = ChannelsService::from_registry().await?;

        debug!("publish new update or group message as creator...");

        let data: ChannelData = channels_service
            .call(GetChannel(channel_id.clone()))
            .await??
            .ok_or_else(|| anyhow!("unrecognized channel"))?;

        let bundle = data
            .bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing bundle"))?;

        match bundle.channel_type {
            t if t == ChannelType::Group as i32 => {
                self.publish_group_message(&data, content_item).await?
            }
            t if t == ChannelType::StatusFeed as i32 => {
                self.publish_to_status_update_channel(&data, content_item)
                    .await?
            }
            _ => bail!("unsupported channel type"),
        }

        Ok(message_id)
    }
}
