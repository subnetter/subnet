// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::channels::channels_data_service::{ChannelsService, GetChannel};
use crate::simple_client::SimpleClient;
use anyhow::{anyhow, bail, Result};
use base::api_types_extensions::Signed;
use base::snp::snp_client_to_client::NewChannelMessageRequest;
use base::snp::snp_core_types::{ChannelData, ChannelType};
use base::snp::snp_server_api::TypedMessage;
use xactor::Service;

impl SimpleClient {
    /// Handles a request from another remote client to send a message to a group owned by this client
    /// or a reply to a status update in a channel created by this client
    pub(crate) async fn handle_channel_message_request(&mut self, msg: TypedMessage) -> Result<()> {
        use prost::Message;
        let request: NewChannelMessageRequest =
            NewChannelMessageRequest::decode(msg.message.as_slice())
                .map_err(|e| anyhow!("failed to decode message {:?}", e))?;

        let content_item = request
            .content_item
            .as_ref()
            .ok_or_else(|| anyhow!("missing channel content item"))?;

        // verify author wrote this content item
        content_item.verify_signature()?;

        let channel_id = content_item.channel_id.clone();
        let author_id = content_item
            .author
            .as_ref()
            .ok_or_else(|| anyhow!("missing author"))?;
        let author_id_key = author_id.get_id()?;
        let channels_service = ChannelsService::from_registry().await?;
        let channel_data: ChannelData = channels_service
            .call(GetChannel(channel_id))
            .await??
            .ok_or_else(|| anyhow!("unknown channel"))?;
        let channel_bundle = channel_data
            .bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing channel bundle"))?;

        match channel_bundle.channel_type {
            t if t == ChannelType::StatusFeed as i32 => {
                if content_item.reply_to == 0 {
                    // this is not a reply - shouldn't publish...
                    bail!("content must be a reply to a status update")
                }
                // check that author is a subscriber
                match channel_data.get_subscriber(author_id_key)? {
                    Some(_) => {
                        // author is subscriber to this channel

                        self.publish_to_status_update_channel(&channel_data, content_item.clone())
                            .await?;
                        Ok(())
                    }
                    None => {
                        warn!("didn't find user in subscribers list");
                        Ok(())
                    }
                }
            }

            t if t == ChannelType::Group as i32 => {
                let members = channel_data
                    .group_members
                    .as_ref()
                    .ok_or_else(|| anyhow!("missing group members"))?;

                if let Some(_member) = members.get_member(author_id_key) {
                    self.publish_group_message(&channel_data, content_item.clone())
                        .await?;
                } else {
                    warn!("author is not a group member");
                }

                Ok(())
            }
            _ => bail!("unrecognized channel type"),
        }
    }
}
