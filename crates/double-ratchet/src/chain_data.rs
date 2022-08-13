// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::chain_key::ChainKey;
use crate::kdf::Kdf;
use anyhow::Result;

use serde::{Deserialize, Serialize};
/// ChainData is the data for a stateful kdf chain.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) struct ChainData<K: Kdf> {
    key: ChainKey, // chain-key
    kdf: K,        // the key derivation function
    n: u32,        // key nonce / index
}

impl<K: Kdf> ChainData<K> {
    // Create a new chain data from a dkf and a chain key.
    pub(crate) fn new(kdf: K, key: ChainKey) -> ChainData<K> {
        ChainData { key, kdf, n: 0 }
    }

    // Advance the chain by providing new input
    pub(crate) fn advance(&mut self, input: K::Input) -> Result<(u32, K::Output)> {
        let (key, output) = self.kdf.derive(self.key, input)?;
        self.key = key;
        let n = self.n;
        self.n += 1;

        Ok((n, output))
    }

    pub(crate) fn into_kdf(self) -> K {
        self.kdf
    }
}
