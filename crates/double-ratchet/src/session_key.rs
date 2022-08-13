// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use serde::{Deserialize, Serialize};

/// A session key is an input of a root chain
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionKey(pub(crate) [u8; 32]);

impl std::ops::Deref for SessionKey {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl From<&[u8]> for SessionKey {
    fn from(slice: &[u8]) -> SessionKey {
        let len = if slice.len() < 32 { slice.len() } else { 32 };
        let mut arr = [0; 32];
        arr[..len].clone_from_slice(&slice[..len]);
        SessionKey(arr)
    }
}

/* uncomment when needed
impl SessionKey {
    fn new(data: [u8; 32]) -> SessionKey {
        SessionKey(data)
    }
}*/
