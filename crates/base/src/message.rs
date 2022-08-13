// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::anyhow;
use std::convert::TryInto;

use crate::snp::snp_server_api::Message;

impl Message {
    // Get sender dr public key from a message
    pub fn get_sender_dr_pub_key(&self) -> anyhow::Result<x25519_dalek::PublicKey> {
        let key_data: [u8; 32] = self
            .header
            .as_ref()
            .ok_or_else(|| anyhow!("missing sender"))?
            .dr_pub_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing dr public key"))?
            .key
            .as_slice()
            .try_into()
            .map_err(|_| anyhow!("bad dr pyb key size"))?;

        let pub_key: x25519_dalek::PublicKey = key_data.into();
        Ok(pub_key)
    }
}
