// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use bytes::{BufMut, BytesMut};
use ed25519_dalek::KEYPAIR_LENGTH;

use crate::snp::snp_core_types::KeyPair;

impl KeyPair {
    pub fn to_ed2559_kaypair(&self) -> ed25519_dalek::Keypair {
        let mut buf = BytesMut::with_capacity(KEYPAIR_LENGTH);

        buf.put(self.private_key.as_ref().unwrap().key.as_slice());
        buf.put(self.public_key.as_ref().unwrap().key.as_slice());

        ed25519_dalek::Keypair::from_bytes(buf.as_ref()).unwrap()
    }
}
