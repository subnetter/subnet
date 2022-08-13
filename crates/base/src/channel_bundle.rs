// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::snp::snp_core_types::ChannelBundle;
use anyhow::{anyhow, Result};
use ed25519_dalek::Signer;

impl ChannelBundle {
    pub fn get_channel_id(&self) -> Result<Vec<u8>> {
        Ok(self
            .channel_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing data"))?
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing data"))?
            .key
            .clone())
    }

    /// Sign a channel bundle by channel id and by channel creator
    pub fn sign(
        &mut self,
        client_pair: &ed25519_dalek::Keypair,
        channel_pair: &ed25519_dalek::Keypair,
    ) -> Result<()> {
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;

        use crate::snp::snp_core_types::Signature;
        self.signature = Some(Signature {
            scheme_id: 0,
            signature: channel_pair.sign(&buf).as_ref().to_vec(),
        });

        let mut buf1 = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf1)?;

        self.creator_signature = Some(Signature {
            scheme_id: 0,
            signature: client_pair.sign(&buf1).as_ref().to_vec(),
        });
        Ok(())
    }
}
