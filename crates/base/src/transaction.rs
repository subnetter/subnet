// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::api_types_extensions::Signed;
use crate::snp::snp_blockchain::Transaction;
use anyhow::{anyhow, bail, Result};
use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::{Keypair, Signer, Verifier};
use orion::hazardous::hash::sha2::sha512::Sha512;

impl Signed for Transaction {
    /// Sign the transaction using the provided keypair
    fn sign(&mut self, signer: &Keypair) -> Result<()> {
        self.signature = vec![];

        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;

        self.signature = signer.sign(&buf).as_ref().to_vec();
        Ok(())
    }

    /// Verify transaction is properly signed by its sender_pub_key
    fn verify_signature(&self) -> Result<()> {
        let signature = self.signature.clone();
        let mut data = self.clone();
        data.signature = vec![];

        if !data.fee_signature.is_empty() {
            // Fee was paid by another party. Remove the parts signed by the fee_signature as it was not signed by sender.
            data.fee = None;
            data.fee_signature = vec![];
        }

        use prost::Message;
        let mut buf = Vec::with_capacity(data.encoded_len());
        if data.encode(&mut buf).is_err() {
            return Err(anyhow!("failed to encode source data to binary data"));
        };

        let signature = ed25519_dalek::Signature::from_bytes(signature.as_slice())?;
        let signer_pub_key = ed25519_dalek::PublicKey::from_bytes(self.sender_pub_key.as_ref())?;

        Ok(signer_pub_key.verify(&buf, &signature)?)
    }
}
impl Transaction {
    /// Returns true if fee was paid by a 3rd party and false when it was paid by tx sender
    pub fn third_party_fee_payer(&self) -> bool {
        !self.fee_signature.is_empty()
    }

    /// Validates that tx has valid fee data
    pub fn validate_fee(&self) -> Result<()> {
        if self.fee.is_none() {
            bail!("missing fee data")
        }

        let fee = self.fee.as_ref().unwrap();
        if fee.amount.is_none() {
            bail!("missing fee amount")
        }

        if !fee.payer_public_key.is_empty() && self.fee_signature.is_empty() {
            bail!("expected fee signature on tx fee paid by 3rd party")
        }

        if !fee.payer_public_key.is_empty() && fee.payer_public_key == self.sender_pub_key {
            bail!("expected different sender and fee payer public id")
        }

        Ok(())
    }

    /// Verify transactions' fee signature if tx fee was not paid by tx sender
    pub fn verify_fee_signature(&self) -> Result<()> {
        if self.fee_signature.is_empty() {
            bail!("missing fee signature. Sender provided the fee")
        }

        let fee = self
            .fee
            .as_ref()
            .ok_or_else(|| anyhow!("missing expected fee data"))?;

        let signature = self.fee_signature.clone();
        let mut data = self.clone();
        data.fee_signature = vec![];

        use prost::Message;
        let mut buf = Vec::with_capacity(data.encoded_len());
        if data.encode(&mut buf).is_err() {
            return Err(anyhow!("failed to encode source data to binary data"));
        };

        let signature = ed25519_dalek::Signature::from_bytes(signature.as_slice())?;
        let signer_pub_key = ed25519_dalek::PublicKey::from_bytes(fee.payer_public_key.as_ref())?;

        Ok(signer_pub_key.verify(&buf, &signature)?)
    }

    pub fn get_sender_address(&self) -> Vec<u8> {
        self.sender_pub_key.as_slice()[self.sender_pub_key.len() - 20..].to_vec()
    }

    /// Returns fee payer address
    pub fn get_fee_payer_address(&self) -> Vec<u8> {
        self.fee.as_ref().unwrap().payer_public_key.as_slice()[self.sender_pub_key.len() - 20..]
            .to_vec()
    }

    /// Returns the transaction's id which is Sha512 hash of its binary data
    pub fn get_tx_id(&self) -> Result<Vec<u8>> {
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;

        let mut hasher = Sha512::new();
        hasher.update(&buf)?;
        Ok(hasher.finalize()?.as_ref().to_vec())
    }
}
