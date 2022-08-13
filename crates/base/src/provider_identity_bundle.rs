// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::api_types_extensions::Signed;
use crate::snp::snp_core_types::{ProviderIdentityBundle, PublicKey};
use anyhow::{anyhow, Result};
use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::{Signer, Verifier};

impl Signed for ProviderIdentityBundle {
    fn sign(&mut self, signer: &ed25519_dalek::Keypair) -> Result<()> {
        use prost::Message;

        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;
        let signature = signer.sign(&buf);
        use crate::snp::snp_core_types::Signature;
        self.provider_signature = Some(Signature {
            scheme_id: 0,
            signature: signature.to_bytes().to_vec(),
        });

        Ok(())
    }

    /// Verify the provider public signature on the bundle
    fn verify_signature(&self) -> Result<()> {
        if self.provider_id.is_none() {
            return Err(anyhow!("missing client id"));
        }

        let pub_key_data = self
            .provider_id
            .as_ref()
            .unwrap()
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing public key"))?;

        let pub_key = pub_key_data.as_pub_key()?;

        if self.provider_signature.is_none() {
            return Err(anyhow!("missing message signature"));
        }

        let signature_data = self.provider_signature.as_ref().unwrap().clone();

        // create message binary data for signature verification
        let mut data = self.clone();
        data.provider_signature = None; // remove signature from message before verification
        use prost::Message;
        let mut buf = Vec::with_capacity(data.encoded_len());
        if data.encode(&mut buf).is_err() {
            return Err(anyhow!("failed to encode source data to binary data"));
        };

        let signature = ed25519_dalek::Signature::from_bytes(signature_data.signature.as_slice())?;

        if pub_key.verify(&buf, &signature).is_err() {
            return Err(anyhow!("failed to verify provider signature"));
        };

        Ok(())
    }
}
impl ProviderIdentityBundle {
    /// Get provider x25519 prekey from an identity bundle
    pub fn get_provider_x25519_pre_key(&self) -> Result<x25519_dalek::PublicKey> {
        self.get_provider_pre_key()?.as_x25519_pub_key()
    }

    pub fn get_provider_pre_key(&self) -> Result<&PublicKey> {
        self.pre_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing prekey data"))?
            .key
            .as_ref()
            .ok_or_else(|| anyhow!("missing prekey data"))
    }

    /// Get provider identity
    pub fn get_provider_id_ed25519_public_key(&self) -> Result<ed25519_dalek::PublicKey> {
        self.get_provider_id_public_key()?.as_pub_key()
    }

    pub fn get_provider_id_public_key(&self) -> Result<&PublicKey> {
        self.provider_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing provider id data"))?
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing public key data"))
    }
}
