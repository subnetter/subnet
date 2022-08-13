// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::anyhow;
use std::convert::TryInto;

use crate::snp::snp_server_api::NewSessionRequest;

impl NewSessionRequest {
    pub fn get_receiver(&self) -> anyhow::Result<ed25519_dalek::PublicKey> {
        self.receiver
            .as_ref()
            .ok_or_else(|| anyhow!("missing receiver"))?
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing public key"))?
            .as_pub_key()
    }

    pub fn get_sender_dr_pub_key(&self) -> anyhow::Result<x25519_dalek::PublicKey> {
        self.message
            .as_ref()
            .ok_or_else(|| anyhow!("missing msg"))?
            .get_sender_dr_pub_key()
    }

    pub fn get_dr_session_id(&self) -> anyhow::Result<u64> {
        Ok(self
            .message
            .as_ref()
            .ok_or_else(|| anyhow!("missing message"))?
            .header
            .as_ref()
            .ok_or_else(|| anyhow!("missing header"))?
            .session_id)
    }

    pub fn get_eka(&self) -> anyhow::Result<x25519_dalek::PublicKey> {
        let eka_key_data: [u8; 32] = self
            .sender_ephemeral_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing eph key"))?
            .key
            .as_slice()
            .try_into()
            .map_err(|_e| anyhow!("bad eka size"))?;

        let eka: x25519_dalek::PublicKey = eka_key_data.into();

        Ok(eka)
    }
}
