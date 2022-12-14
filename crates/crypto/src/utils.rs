// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};
use base::snp::snp_core_types::EntityId;
use curve25519_dalek::edwards::CompressedEdwardsY;
use ed25519_dalek::PUBLIC_KEY_LENGTH;
use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use sha2::{Digest, Sha512};
use std::convert::TryFrom;
use x25519_dalek::StaticSecret;

const ADDRESS_LEN: usize = 20; // bytes

/// Returns sha512 of the data
pub fn sha512(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Return a new EntityId for a x25519 public key
pub fn entity_from_x25519_pub_key(pub_key: x25519_dalek::PublicKey, nickname: String) -> EntityId {
    let k = base::snp::snp_core_types::PublicKey {
        key: pub_key.to_bytes().as_ref().to_vec(),
    };

    EntityId {
        public_key: Some(k),
        nickname,
    }
}

/// Return a new EntityId for an ed25519 public key
pub fn entity_from_ed25519_pub_key(
    pub_key: &ed25519_dalek::PublicKey,
    nickname: String,
) -> EntityId {
    let k = base::snp::snp_core_types::PublicKey {
        key: pub_key.as_bytes().to_vec(),
    };
    EntityId {
        public_key: Some(k),
        nickname,
    }
}

pub fn entity_from_pub_key(
    pub_key: &base::snp::snp_core_types::PublicKey,
    nickname: String,
) -> EntityId {
    EntityId {
        public_key: Some(pub_key.clone()),
        nickname,
    }
}

/// Converts from an ed25519 public key to an x25519 public key
pub struct PublicKeyWrapper(pub x25519_dalek::PublicKey);
impl From<PublicKey> for PublicKeyWrapper {
    // todo: change to TryFrom because decompress() can return an error
    fn from(key: PublicKey) -> Self {
        let ed25519_pk_c = CompressedEdwardsY::from_slice(key.as_bytes());
        let ed25519_pk = ed25519_pk_c.decompress().unwrap();
        let pub_key = x25519_dalek::PublicKey::from(ed25519_pk.to_montgomery().to_bytes());
        PublicKeyWrapper(pub_key)
    }
}

/// Converts from bytes array to x25519::StaticSecret
pub struct StaticSecretWrapper(pub StaticSecret);
impl TryFrom<&[u8]> for StaticSecretWrapper {
    type Error = anyhow::Error;

    fn try_from(slice: &[u8]) -> Result<Self> {
        if slice.len() != 32 {
            return Err(anyhow!("invalid slice size != 32"));
        }

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(slice);
        Ok(StaticSecretWrapper(StaticSecret::from(bytes)))
    }
}

pub struct X25519PublicKeyWrapper(pub x25519_dalek::PublicKey);

/// Converts from bytes to an x25519::PublicKey
impl TryFrom<&[u8]> for X25519PublicKeyWrapper {
    type Error = anyhow::Error;

    fn try_from(slice: &[u8]) -> Result<Self> {
        if slice.len() != 32 {
            return Err(anyhow!("invalid slice size != 32"));
        }

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(slice);
        Ok(X25519PublicKeyWrapper(x25519_dalek::PublicKey::from(bytes)))
    }
}

/// Convert an ed25519 secret key to an x25519 static secret
impl From<&SecretKey> for StaticSecretWrapper {
    fn from(key: &SecretKey) -> Self {
        let mut hasher = Sha512::new();
        hasher.update(key.as_bytes().to_vec());
        let hash = hasher.finalize();
        let mut data = [0; 32];
        for i in 0..32 {
            data[i] = hash[i];
        }
        StaticSecretWrapper(StaticSecret::from(data))
    }
}

/// Generate a pair of ed25519 identity keys
#[allow(dead_code)]
pub fn create_key_pair() -> Keypair {
    Keypair::generate(&mut rand_core::OsRng)
}

/// Create an immutable Address from a public key
#[allow(dead_code)]
pub fn create_address(key: &[u8]) -> Vec<u8> {
    key[(PUBLIC_KEY_LENGTH - ADDRESS_LEN)..].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{create_address, create_key_pair};
    use base::test_helpers::enable_logger;

    #[test]
    fn test_utils() {
        enable_logger();

        let keys = create_key_pair();

        debug!(
            "Pub key: {:?}, Private kay: {:?}",
            hex::encode(keys.public),
            hex::encode(keys.secret),
        );
        let address = create_address(keys.public.as_ref());
        debug!("Address {:?}", hex::encode(address.to_vec()));

        assert_eq!(address.len(), ADDRESS_LEN, "expected non-empty address");
        assert_eq!(
            address,
            &keys.public.as_bytes()[(PUBLIC_KEY_LENGTH - ADDRESS_LEN)..]
        );
    }
}
