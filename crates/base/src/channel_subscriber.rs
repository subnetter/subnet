// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::snp::snp_core_types::ChannelSubscriber;
use anyhow::anyhow;

impl ChannelSubscriber {
    pub fn get_subscriber_id(&self) -> anyhow::Result<&Vec<u8>> {
        Ok(self
            .user_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing client id"))?
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing pub key"))?
            .key
            .as_ref())
    }

    pub fn has_subscriber_id(&self, an_id: &[u8]) -> bool {
        let id: &[u8] = self
            .user_id
            .as_ref()
            .unwrap()
            .public_key
            .as_ref()
            .unwrap()
            .key
            .as_ref();

        id == an_id
    }
}
