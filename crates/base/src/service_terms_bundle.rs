// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::api_types_extensions::Signed;
use crate::snp::snp_core_types::ServiceTermsBundle;
use anyhow::anyhow;
use ed25519_dalek::ed25519::signature::Signature;
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for ServiceTermsBundle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // todo: proper implement this to output all terms
        write!(f, "")
    }
}

impl Signed for ServiceTermsBundle {
    fn sign(&mut self, signer: &ed25519_dalek::Keypair) -> anyhow::Result<()> {
        use ed25519_dalek::Signer;
        use prost::Message;
        self.signature = None;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;
        let signature = signer.sign(&buf);
        use crate::snp::snp_core_types::Signature;
        self.signature = Some(Signature {
            scheme_id: 0,
            signature: signature.to_bytes().to_vec(),
        });
        Ok(())
    }

    /// Verify the provider public signature on the bundle
    /// todo: get rid of all duplicated code to sign and verify structures such as this and use 1 helper function for this capability.
    fn verify_signature(&self) -> anyhow::Result<()> {
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

        if self.signature.is_none() {
            return Err(anyhow!("missing message signature"));
        }

        let signature_data = self.signature.as_ref().unwrap().clone();

        // create message binary data for signature verification
        let mut data = self.clone();
        data.signature = None; // remove signature from message before verification
        use ed25519_dalek::Verifier;
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
