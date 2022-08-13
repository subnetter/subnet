// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::network_salt::NET_SALT;
use anyhow::Result;
use bytes::Bytes;
use crypto::aead_cypher::AeadCipher;

/// AEAD is an aead encryption codec used to encrypt and decrypt messages using a shared secret, network salt and ad.
/// See AEAD paper for additional info.
pub struct AEAD;

impl AEAD {
    /// decrypts ciphertext to cleartext
    pub fn decrypt(ciphertext: &[u8], key: &[u8; 32], ad: &[u8]) -> Result<Bytes> {
        let cipher = AeadCipher::new(
            Bytes::from(NET_SALT.to_vec()),
            Bytes::from(key.to_vec()),
            Bytes::from(ad.to_vec()),
        );

        Ok(Bytes::from(cipher.decrypt(ciphertext)?))
    }

    /// encrypt cleartext to ciphertext
    pub fn encrypt(plaintext: Bytes, key: &[u8; 32], ad: &[u8]) -> Result<Bytes> {
        let cipher = AeadCipher::new(
            Bytes::from(NET_SALT.to_vec()),
            Bytes::from(key.to_vec()),
            Bytes::from(ad.to_vec()),
        );

        cipher.encrypt(plaintext)
    }
}

// todo: add some tests
