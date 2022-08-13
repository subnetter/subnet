//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::consts::PRE_KEY_LEN;
use crate::model::types::PreKey;
use anyhow::{bail, Result};
use base::hex_utils::short_hex_format;
use rand_core::OsRng;
use std::fmt;
use std::fmt::{Display, Formatter};
use x25519_dalek::StaticSecret;

impl Display for PreKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        short_hex_format(self.key.as_ref(), f)
    }
}

impl PreKey {
    // Create a new pre-key with id, also returns the static secret used to generate it
    pub fn new_pre_key(id: u32) -> (StaticSecret, PreKey) {
        // this is user's content enc private key - should be stored in his client
        let pre_key_private = StaticSecret::new(OsRng);
        let pre_key_public = x25519_dalek::PublicKey::from(&pre_key_private);

        (
            pre_key_private,
            PreKey {
                id,
                key: pre_key_public.as_bytes().to_vec(),
            },
        )
    }

    pub fn get_x25519_pub_key(&self) -> Result<x25519_dalek::PublicKey> {
        let slice = self.key.as_slice();

        if slice.len() != PRE_KEY_LEN {
            bail!("invalid data size {} != {}", slice.len(), PRE_KEY_LEN);
        }

        let mut bytes = [0u8; PRE_KEY_LEN];
        bytes.copy_from_slice(slice);
        Ok(x25519_dalek::PublicKey::from(bytes))
    }
}
