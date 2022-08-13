// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::playground::Playground;
use anyhow::{anyhow, Result};
use base::snp::upsetter_simple_client::UserCreateGroupRequest;

impl Playground {
    pub(crate) async fn create_group(&mut self, client_name: &str, group_name: &str) -> Result<()> {
        let client = self.clients.get_mut(client_name);

        if client.is_none() {
            return Err(anyhow!("unknown client"));
        }

        let resp = client
            .unwrap()
            .user_create_group(UserCreateGroupRequest {
                group_name: group_name.to_string(),
            })
            .await
            .unwrap()
            .into_inner();

        match resp.channel_bundle {
            Some(channel_bundle) => {
                // let channel_id: &EntityId = channel_bundle.channel_id.as_ref().unwrap();
                // let channel_name = short_hex_string(channel_id.get_id().unwrap().as_ref());
                self.channels.insert(group_name.to_string(), channel_bundle);
                println!("🖖 created group {}", group_name);
                Ok(())
            }
            None => Err(anyhow!("failed to create group")),
        }
    }
}
