// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::playground::Playground;
use anyhow::{anyhow, Result};
use base::snp::upsetter_simple_client::UserSubscribeRequest;

impl Playground {
    pub(crate) async fn channel_subscribe(
        &mut self,
        client_name: &str,
        channel_name: &str,
    ) -> Result<()> {
        let client = self.clients.get_mut(client_name);

        if client.is_none() {
            return Err(anyhow!("unknown client"));
        }

        let channel_bundle = self.channels.get(channel_name);
        if channel_bundle.is_none() {
            return Err(anyhow!("channel bundle not found"));
        }

        client
            .unwrap()
            .user_subscribe_to_status_updates(UserSubscribeRequest {
                channel_bundle: Some(channel_bundle.unwrap().clone()),
            })
            .await
            .map_err(|e| anyhow!(format!("error subscribing: {:?}", e)))?
            .into_inner();

        println!("🖖 subscribed to channel {}", channel_name);
        Ok(())
    }
}
