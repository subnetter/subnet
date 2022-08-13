// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, Result};
use base::hex_utils::short_hex_string;
use base::snp::snp_core_types::{ChannelData, ContentItem};
use bytes::Bytes;

impl SimpleClient {
    // Publish a group message from a group member - send it to all group members that should get it
    pub(crate) async fn publish_group_message(
        &mut self,
        data: &ChannelData,
        content_item: ContentItem,
    ) -> Result<()> {
        let author = content_item
            .author
            .as_ref()
            .ok_or_else(|| anyhow!("missing message author"))?;

        let author_id = author.get_id()?.as_slice();
        let channel_owner = self.get_client_entity()?;
        let channel_owner_id = channel_owner.get_id()?.as_slice();

        if channel_owner_id != author_id {
            info!(
                "🎉 👋 incoming group message from group member, {}: {}. Reply to: {}",
                short_hex_string(author_id),
                content_item.get_simple_text_content()?,
                content_item.reply_to,
            );
        }

        let members_bundle = data
            .group_members
            .as_ref()
            .ok_or_else(|| anyhow!("missing group members"))?;

        debug!("Group members: {}", members_bundle.members.len());

        let channel_id = data.get_channel_id()?;

        for member in members_bundle.members.iter() {
            let id = member
                .user_id
                .as_ref()
                .ok_or_else(|| anyhow!("missing user id"))?;

            let key = id
                .public_key
                .as_ref()
                .ok_or_else(|| anyhow!("missing pub key"))?
                .as_pub_key()?;

            if key.as_ref() == author_id {
                continue;
            }

            if key.as_ref() == channel_owner_id {
                // skip sending the message to ourselves (as channel owners)
                continue;
            }

            debug!(
                "sending update to member: {:}",
                short_hex_string(key.as_ref())
            );

            let message = self
                .new_channel_message(&key, channel_id.clone().as_ref(), content_item.clone())
                .await?;

            let receiver_id = Bytes::from(member.get_member_id()?);
            self.send_typed_message(message, receiver_id).await?;
        }

        Ok(())
    }
}
