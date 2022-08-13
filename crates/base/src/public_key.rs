// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::hex_utils::short_hex_string;
use crate::snp::snp_core_types::PublicKey;
use crate::snp::snp_payments::Address;
use anyhow::{anyhow, Result};
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for PublicKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", short_hex_string(self.key.as_ref()))
    }
}

impl PublicKey {
    // display

    /// PublicKey to ed25519 public key
    pub fn as_pub_key(&self) -> Result<ed25519_dalek::PublicKey> {
        let bytes = self.key.as_slice();
        Ok(ed25519_dalek::PublicKey::from_bytes(bytes)?)
    }

    /// PublicKey to x25519 public key
    pub fn as_x25519_pub_key(&self) -> Result<x25519_dalek::PublicKey> {
        let slice = self.key.as_slice();

        if slice.len() != 32 {
            return Err(anyhow!("invalid data size != 32"));
        }

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(slice);
        Ok(x25519_dalek::PublicKey::from(bytes))
    }

    /// Get an address from the public key
    pub fn get_address(&self) -> Address {
        let mut slice = [0u8; 20];
        slice.copy_from_slice(&self.key.as_slice()[12..]);
        Address {
            data: slice.to_vec(),
        }
    }
}
