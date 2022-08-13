// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::chain_data::ChainData;
use crate::chain_key::ChainKey;
use crate::chainer::Chainer;
use crate::kdf::Kdf;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) enum ChainState<K: Kdf> {
    Init(Chainer<K>),  // has a kdf and can move to run state if a chain key is provided
    Run(ChainData<K>), // running state
}

/// Chain is a stateful kdf chain as defined in the DR protocol.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) struct Chain<K: Kdf> {
    state: Option<ChainState<K>>,
}

impl<K: Kdf> Chain<K> {
    pub(crate) fn new(kdf: K) -> Chain<K> {
        Chain {
            state: Some(ChainState::Init(Chainer::new(kdf))),
        }
    }

    pub(crate) fn next_chain(&mut self, key: ChainKey) {
        self.state = match self.state.take().unwrap() {
            ChainState::Init(chainer) => Some(ChainState::Run(chainer.into_chain_data(key))),
            ChainState::Run(chain_data) => {
                let kdf = chain_data.into_kdf();
                Some(ChainState::Run(ChainData::new(kdf, key)))
            }
        }
    }

    // Advance a chain in the running state
    pub(crate) fn advance(&mut self, input: K::Input) -> Result<(u32, K::Output)> {
        match self.state {
            Some(ChainState::Run(ref mut chain_data)) => chain_data.advance(input),
            _ => Err(anyhow!("chain is not in running state - can't advance")),
        }
    }
}
