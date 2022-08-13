// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::snp::snp_server_api::ForwardMessagePayload;
use anyhow::anyhow;

impl ForwardMessagePayload {
    pub fn get_receiver_pub_key(&self) -> anyhow::Result<ed25519_dalek::PublicKey> {
        self.receiver
            .as_ref()
            .ok_or_else(|| anyhow!("missing receiver id"))?
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing pub key"))?
            .as_pub_key()
    }
}
