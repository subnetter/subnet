// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::api_types_extensions::Signed;
use crate::snp::snp_core_types::{EntityId, ProviderSignedClientIdentityBundle};
use anyhow::{anyhow, Result};
use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::{Signer, Verifier};

impl ProviderSignedClientIdentityBundle {
    // Get the public client id raw data
    pub fn get_client_id(&self) -> anyhow::Result<Vec<u8>> {
        Ok(self
            .get_client_entity()?
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing pub key"))?
            .key
            .clone())
    }

    pub fn get_client_entity(&self) -> anyhow::Result<EntityId> {
        Ok(self
            .client_bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing client bundle"))?
            .client_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing client id"))?
            .clone())
    }
}

impl Signed for ProviderSignedClientIdentityBundle {
    fn sign(&mut self, signer: &ed25519_dalek::Keypair) -> Result<()> {
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;

        use crate::snp::snp_core_types::Signature;
        self.signature = Some(Signature {
            scheme_id: 0,
            signature: signer.sign(&buf).as_ref().to_vec(),
        });
        Ok(())
    }

    /// Verify (i) provider signature on the client bundle, (ii) the client signature on its bundle,
    /// and (iii) the provider signature on its own bundle
    fn verify_signature(&self) -> Result<()> {
        let client_bundle = self
            .client_bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing client bundle"))?;

        let provider_bundle = client_bundle
            .provider_bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing provider bundle"))?;

        client_bundle.verify_signature()?;
        provider_bundle.verify_signature()?;

        let provider_key_data = provider_bundle
            .provider_id
            .as_ref()
            .unwrap()
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing public key"))?;

        let provider_pub_key = provider_key_data.as_pub_key()?;

        if self.signature.is_none() {
            return Err(anyhow!("missing message signature"));
        }

        let signature_data = self.signature.as_ref().unwrap().clone();

        // create message binary data for signature verification
        let mut data = self.clone();
        data.signature = None; // remove signature from message before verification
        use prost::Message;
        let mut buf = Vec::with_capacity(data.encoded_len());
        if data.encode(&mut buf).is_err() {
            return Err(anyhow!("failed to encode source data to binary data"));
        };

        let signature = ed25519_dalek::Signature::from_bytes(signature_data.signature.as_slice())?;

        if provider_pub_key.verify(&buf, &signature).is_err() {
            return Err(anyhow!("failed to verify provider signature"));
        };

        Ok(())
    }
}
