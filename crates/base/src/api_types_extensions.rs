// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;

/// A signed message with verifier public key for verification in the message
pub trait Signed {
    fn sign(&mut self, signer: &ed25519_dalek::Keypair) -> Result<()>;
    fn verify_signature(&self) -> Result<()>;
}

/// Signed message with external verifier
pub trait SignedWithExternalVerifier {
    fn sign(&mut self, signer: &ed25519_dalek::Keypair) -> Result<()>;
    fn verify_signature(&self, signer: &ed25519_dalek::PublicKey) -> Result<()>;
}
