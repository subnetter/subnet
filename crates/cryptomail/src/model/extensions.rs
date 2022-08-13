//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;

/// A signed message with verifier public key for verification in the message
pub trait Signed {
    fn verify_signature(&self) -> Result<()>;
}

// A validatable message
// We use rust structs generated from proto3 specs. In proto3 each reference value data item is optional.
// Validation checks that reference values are included in a struct.
pub trait Validatable {
    fn validate(&self) -> Result<()>;
}

pub trait Signer {
    fn sign(&mut self, signer: &ed25519_dalek::Keypair) -> Result<()>;
}
