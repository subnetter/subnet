// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::chain_data::ChainData;
use crate::chain_key::ChainKey;
use crate::kdf::Kdf;
use serde::{Deserialize, Serialize};

/// A Chainer holds a kdf function and knows how to convert itself into ChainData for a ChainKey using its kdf.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) struct Chainer<K: Kdf> {
    kdf: K,
}

impl<K: Kdf> Chainer<K> {
    pub(crate) fn new(kdf: K) -> Chainer<K> {
        Chainer { kdf }
    }

    pub(crate) fn into_chain_data(self, key: ChainKey) -> ChainData<K> {
        ChainData::new(self.kdf, key)
    }
}
