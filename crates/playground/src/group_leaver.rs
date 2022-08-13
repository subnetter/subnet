// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::playground::Playground;
use anyhow::{anyhow, Result};
use base::snp::upsetter_simple_client::UserLeaveGroupRequest;

impl Playground {
    pub(crate) async fn leave_group(
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
            return Err(anyhow!("group bundle not found"));
        }

        client
            .unwrap()
            .user_leave_group(UserLeaveGroupRequest {
                channel_bundle: Some(channel_bundle.unwrap().clone()),
            })
            .await
            .map_err(|e| anyhow!(format!("error leaving: {:?}", e)))?
            .into_inner();

        println!("🖖 left group {}", channel_name);
        Ok(())
    }
}
