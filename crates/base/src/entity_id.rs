// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::snp::snp_core_types::{EntityId, PublicKey};
use anyhow::{anyhow, Result};

impl EntityId {
    pub fn get_ed_pub_key(&self) -> Result<ed25519_dalek::PublicKey> {
        let k = self
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing public key"))?;
        Ok(k.as_pub_key()?)
    }

    pub fn new(public_key: PublicKey, nickname: String) -> EntityId {
        EntityId {
            public_key: Some(public_key),
            nickname,
        }
    }

    pub fn get_id(&self) -> Result<&Vec<u8>> {
        Ok(self
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing pub key"))?
            .key
            .as_ref())
    }
}
