// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::playground::Playground;
use anyhow::{anyhow, Result};
use base::snp::upsetter_simple_client::UserNewPostRequest;

impl Playground {
    pub(crate) async fn status_update(
        &mut self,
        client_name: &str,
        channel_name: &str,
        text: String,
        reply_to: u64,
    ) -> Result<()> {
        let client = self.clients.get_mut(client_name);

        if client.is_none() {
            return Err(anyhow!("unknown sender"));
        }

        let channel_bundle = self.channels.get(channel_name);
        if channel_bundle.is_none() {
            return Err(anyhow!("unknown sender"));
        }

        let channel_id = channel_bundle
            .unwrap()
            .channel_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing channel id"))?
            .clone();

        match client
            .unwrap()
            .user_new_post(UserNewPostRequest {
                channel_id: Some(channel_id),
                text,
                reply_to,
            })
            .await
        {
            Ok(_resp) => {
                /*
                println!(
                    "🖖 Channel message published. Message id: {}",
                    resp.into_inner().post_id
                );*/
                Ok(())
            }
            Err(e) => Err(anyhow!("failed to send message: {}", e)),
        }
    }
}
