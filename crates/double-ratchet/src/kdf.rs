// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::chain_key::ChainKey;
use crate::message_key::MessageKey;
use crate::session_key::SessionKey;
use anyhow::Result;
use crypto::hmacer::Hmacer;
use crypto::kdfer::Kdfer;
use serde::{Deserialize, Serialize};

/// A Kdf trait specifies the derive func as well as the input and output data types.
pub(crate) trait Kdf {
    type Input: Copy;
    type Output: Copy;

    // Kdf takes 2 inputs: a chain key and input, and outputs 2 items: output and a new chain key.
    fn derive(&self, key: ChainKey, input: Self::Input) -> Result<(ChainKey, Self::Output)>;
}

/// A root kdf is initialized with a session key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RootKdf(pub(crate) [u8; 32]);

impl RootKdf {
    pub(crate) fn _new(input: SessionKey) -> RootKdf {
        RootKdf(input.0)
    }
}

impl Kdf for RootKdf {
    type Input = SessionKey; // root kdf takes a session key as an input
    type Output = ChainKey; // root kdf outputs a chain key

    fn derive(&self, key: ChainKey, input: SessionKey) -> Result<(ChainKey, ChainKey)> {
        let bytes = Kdfer::hkdf(&key.0, &input.0, &self.0)?;
        let new_key = ChainKey::from(&bytes[0..32]);
        let output = ChainKey::from(&bytes[32..64]);
        Ok((new_key, output))
    }
}

// A hash-chain that derives new MessageKey from an old one w/o any additional input.
// This kdf provides backward secrecy as it is using one-way hash function for derivation.
// Sending and receiving chains use this kdf
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) struct ChainKdf;

impl Kdf for ChainKdf {
    type Input = (); // Chain-kdf has not explicit input. The input is derived from the chain key
    type Output = MessageKey;

    fn derive(&self, key: ChainKey, _input: ()) -> Result<(ChainKey, MessageKey)> {
        let raw_key = Hmacer::hmac_sha512(&key.0, &[0x02])?;
        let raw_out = Hmacer::hmac_sha512(&key.0, &[0x01])?;

        let new_key = ChainKey::from(&raw_key.unprotected_as_bytes()[..]);
        let output = MessageKey::from(&raw_out.unprotected_as_bytes()[..]);

        Ok((new_key, output))
    }
}
