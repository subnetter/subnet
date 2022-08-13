// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::snp::snp_core_types::{ChannelBundle, ChannelData, ChannelSubscriber};
use anyhow::{anyhow, Result};

impl ChannelData {
    pub fn get_bundle(&self) -> Result<&ChannelBundle> {
        Ok(self
            .bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing bundle"))?)
    }

    pub fn get_channel_id(&self) -> Result<Vec<u8>> {
        Ok(self
            .bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing data"))?
            .get_channel_id()?)
    }

    // Get subscriber for id
    // todo: this needs to be optimized by holding in-memory hash-maps for the channel
    // and avoid iterating over a vector to find by key.... we'd like to support big groups and users
    // with a large number of followers...
    pub fn get_subscriber(&self, subscriber_id: &[u8]) -> Result<Option<ChannelSubscriber>> {
        match self
            .subscribers
            .iter()
            .position(|sub| sub.get_subscriber_id().unwrap().as_slice() == subscriber_id)
        {
            Some(idx) => Ok(Some(self.subscribers[idx].clone())),
            None => Ok(None),
        }
    }
}
