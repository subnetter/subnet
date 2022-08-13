// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Formatter};

/// The output of a sending or a receiving chain.
/// Used to encrypt and decrypt one message.
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageKey(pub(crate) [u8; 32]);

impl std::ops::Deref for MessageKey {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl MessageKey {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Serialize this key into a byte array
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0
    }
}

impl Debug for MessageKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let data = self.as_bytes().to_vec();
        base::hex_utils::short_hex_format(&data, f)
    }
}

impl From<&[u8]> for MessageKey {
    fn from(slice: &[u8]) -> MessageKey {
        let len = if slice.len() < 32 { slice.len() } else { 32 };
        let mut arr = [0; 32];
        arr[..len].clone_from_slice(&slice[..len]);

        MessageKey(arr)
    }
}
