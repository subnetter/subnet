// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use sha2::{Digest, Sha512};
use x25519_dalek::PublicKey;

/// A simple AEAD encryption and decryption algorithm based on convention on how 2 parties compute same AD
/// and a SharedSecret between them. For example, by executing diffie-hellman between them.

/// Compute ad. First input is alice eph pub key. Second input is bob's pre key.
pub fn compute_ad(alice_eph_pub_key: &PublicKey, bob_pre_key: &PublicKey) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(alice_eph_pub_key.as_bytes().to_vec());
    hasher.update(bob_pre_key.as_bytes().to_vec());
    hasher.finalize().to_vec()
}
