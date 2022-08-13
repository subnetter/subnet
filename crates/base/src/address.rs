// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::hex_utils::hex_string;
use crate::snp::snp_core_types::PublicKey;
use crate::snp::snp_payments::Address;
use ed25519_dalek::PUBLIC_KEY_LENGTH;
use std::fmt;
use std::fmt::{Display, Formatter};

const ADDRESS_LEN: usize = 20; // bytes

impl Display for Address {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", hex_string(self.data.as_ref()))
    }
}

impl Address {
    pub fn new(pub_key: &PublicKey) -> Self {
        Address {
            data: pub_key.key[(PUBLIC_KEY_LENGTH - ADDRESS_LEN)..].to_vec(),
        }
    }
}
