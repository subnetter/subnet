// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use serde::{Deserialize, Serialize};

/// ChainKey is a key used in a kdf chain as both input and output
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainKey(pub(crate) [u8; 32]);

impl std::ops::Deref for ChainKey {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 32]> for ChainKey {
    fn from(data: [u8; 32]) -> ChainKey {
        ChainKey(data)
    }
}

impl From<&[u8]> for ChainKey {
    fn from(slice: &[u8]) -> ChainKey {
        let len = if slice.len() < 32 { slice.len() } else { 32 };

        let mut arr = [0; 32];
        arr[..len].clone_from_slice(&slice[..len]);

        ChainKey(arr)
    }
}
/*
impl From<KeyMaterial> for ChainKey {
    fn from(bytes: KeyMaterial) -> ChainKey {
        ChainKey(bytes.to_bytes())
    }
}*/

#[test]
fn test_chain_keys() {
    use crate::rand::RngCore;
    use rand::rngs::OsRng;
    use std::ops::Deref;

    let mut csprng = OsRng::new().unwrap();
    let mut buf = [0; 32];

    csprng.fill_bytes(&mut buf.as_mut());

    let ck = ChainKey::from(buf.as_ref());
    println!("Chain key: {:?}", ck.0);

    let b: &[u8] = ck.deref();
    assert!(buf.eq(b), "expected same data");
}
