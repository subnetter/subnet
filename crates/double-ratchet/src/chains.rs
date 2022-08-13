// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::chain::Chain;
use crate::chain_key::ChainKey;
use crate::message_key::MessageKey;
use crate::session_key::SessionKey;
use anyhow::{anyhow, bail, Result};

use crate::kdf::{ChainKdf, RootKdf};
use base::hex_utils::short_hex_string;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Chains is the main data structure used by the DR algorithm with another party.
/// Chains includes 3 chains - Root, Sending and Receiving.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Chains {
    root: Chain<RootKdf>,
    sending: Chain<ChainKdf>,
    receiving: Chain<ChainKdf>,
    // a set of receiving key that were skipped in the current receiving chain
    receiving_keys: HashMap<u32, MessageKey>,
}

impl Chains {
    // Init a new Chains with input bytes and a chain key.
    pub fn init(
        root_input: SessionKey,   // some shared salt between two peers
        root_chain_key: ChainKey, // the root key to use for the root chain
    ) -> Chains {
        let mut root = Chain::new(RootKdf(root_input.0));
        root.next_chain(root_chain_key);

        let sending = Chain::new(ChainKdf);

        let receiving = Chain::new(ChainKdf);

        Chains {
            root,
            sending,
            receiving,
            receiving_keys: HashMap::new(),
        }
    }

    // Advance the sending chain and the root chain
    pub fn next_sending_chain(&mut self, key: SessionKey) -> Result<()> {
        // first we advance the root chain and get a new session key
        let key = self
            .root
            .advance(key)
            .map_err(|_| anyhow!("failed to advance the root chain"))?
            .1;

        // set the new session key as the sending chain key
        self.sending.next_chain(key);
        Ok(())
    }

    /// Advance the receiving chain and the root chain.
    /// pn is PN in the DR paper - the number of keys in the previous sending chain
    pub fn next_receiving_chain(&mut self, key: SessionKey, pn: u32) -> Result<()> {
        // this will store any skipped keys in the previous chain (see section 2.6 in the DR paper)
        let _ = self.get_receiving_key(pn);

        // Advance the root chain...
        let key = self
            .root
            .advance(key)
            .map_err(|_| anyhow!("failed to advance the root chain"))?
            .1;

        // advance the receiving chain...
        self.receiving.next_chain(key);

        Ok(())
    }

    /// Get a next sending key from the current sending chain.
    /// Calling this advances the sending chain but not the root chain.
    pub fn next_sending_key(&mut self) -> Result<(u32, MessageKey)> {
        let key = self.sending.advance(())?;
        debug!(
            "++++ Sending chain advance: { }, { }",
            key.0,
            short_hex_string(key.1.as_bytes())
        );
        Ok(key)
    }

    /// Get receiving key at a specific index - store all skipped keys if any in this session
    /// todo: this needs to be heavily tested as this code is what enables out of order messages decryption
    pub fn get_receiving_key(&mut self, index: u32) -> Result<MessageKey> {
        loop {
            // get next receiving key and compare with index
            let next_key = self.receiving.advance(())?;

            if index < next_key.0 {
                // query is for a skipped key we should have - it should be in our store
                if let Some(skipped_key) = self.receiving_keys.get(&index) {
                    // return a clone of store key as it may be queried again
                    return Ok(*skipped_key);
                } else {
                    // we should have this key but we don't
                    bail!("could not find old receiving key")
                }
            }

            if next_key.0 == index {
                // we have the requested key and it is going to be used for decryption
                return Ok(next_key.1);
            }

            // save the key in this session so it can be used later in a data structure
            // before getting next key in a new iteration of this loop
            self.receiving_keys.insert(next_key.0, next_key.1);
        }

        // todo: figure out when it is okay to remove old keys from the hash
        // currently, it grows with the session - there should be a way to remove old unused keys
        // maybe in a chain previous to the previous chain as these keys can't be used anymore?
    }
}
