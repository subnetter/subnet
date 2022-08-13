//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::consts::{PRE_KEY_LEN, PUB_KEY_LEN};
use crate::model::extensions::Validatable;
use crate::model::types::PublicKey;
use anyhow::{bail, Result};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

impl Display for PublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let data = self.key.to_vec();
        base::hex_utils::short_hex_format(&data, f)
    }
}

impl Hash for PublicKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.key.as_ref())
    }
}

impl Eq for PublicKey {}

impl Into<x25519_dalek::PublicKey> for PublicKey {
    /// Convert this public key into an x25519 public key
    fn into(self) -> x25519_dalek::PublicKey {
        let mut bytes = [0u8; PRE_KEY_LEN];
        let slice = self.key.as_slice();
        bytes.copy_from_slice(slice);
        x25519_dalek::PublicKey::from(bytes)
    }
}

impl From<ed25519_dalek::PublicKey> for PublicKey {
    fn from(key: ed25519_dalek::PublicKey) -> Self {
        PublicKey {
            key: Vec::from(key.as_ref()),
        }
    }
}

impl From<x25519_dalek::PublicKey> for PublicKey {
    fn from(key: x25519_dalek::PublicKey) -> Self {
        PublicKey {
            key: Vec::from(key.as_bytes().as_ref()),
        }
    }
}

impl Validatable for PublicKey {
    fn validate(&self) -> Result<()> {
        match self.key.len() {
            l if l == PUB_KEY_LEN => Ok(()),
            _ => bail!("invalid key length"),
        }
    }
}
