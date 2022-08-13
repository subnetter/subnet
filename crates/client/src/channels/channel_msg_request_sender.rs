// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, bail, Result};
use base::snp::snp_core_types::{ChannelBundle, ContentItem};
use bytes::Bytes;

impl SimpleClient {
    // Send a request to a channel owner to publish content in his channel.
    // Content can be a replay to a status update in a status update channel, or a group message in a group.
    // Content must be signed by author.s
    pub(crate) async fn send_channel_message_request(
        &mut self,
        data: ChannelBundle,
        content_item: ContentItem,
    ) -> Result<()> {
        if content_item.signature.as_ref().is_none() {
            bail!("missing author signature on content")
        }

        let creator_id = data
            .creator_id
            .ok_or_else(|| anyhow!("missing creator id"))?;

        let creator_pub_key = creator_id.get_ed_pub_key()?;

        let message = self
            .new_channel_message_request(&creator_pub_key, content_item)
            .await?;
        let receiver_id = Bytes::from(creator_pub_key.to_bytes().to_vec());
        self.send_typed_message(message, receiver_id).await?;

        Ok(())
    }
}
