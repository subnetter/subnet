// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::api_types_extensions::Signed;
use crate::snp::snp_core_types::{DialupInfo, ProviderNetInfo};
use anyhow::{anyhow, Result};
use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::{Signer, Verifier};
use std::fmt;
use std::fmt::{Display, Formatter};

impl ProviderNetInfo {
    pub fn get_pub_id(&self) -> Result<&[u8]> {
        Ok(self
            .provider_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing provider id"))?
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing public key"))?
            .key
            .as_ref())
    }

    pub fn get_dialup_info(&self) -> Result<&DialupInfo> {
        self.dial_up_info
            .as_ref()
            .ok_or_else(|| anyhow!("missing dialup info"))
    }
}

impl Display for ProviderNetInfo {
    fn fmt(&self, _f: &mut Formatter) -> fmt::Result {
        // todo: add all props here
        Ok(())
    }
}

impl Signed for ProviderNetInfo {
    fn sign(&mut self, member_key_pair: &ed25519_dalek::Keypair) -> Result<()> {
        self.signature = None;
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;

        use crate::snp::snp_core_types::Signature;
        self.signature = Some(Signature {
            scheme_id: 0,
            signature: member_key_pair.sign(&buf).as_ref().to_vec(),
        });

        Ok(())
    }

    fn verify_signature(&self) -> Result<()> {
        let signature = self
            .signature
            .as_ref()
            .ok_or_else(|| anyhow!("missing author signature"))?
            .clone();

        let mut data = self.clone();
        data.signature = None;

        use prost::Message;
        let mut buf = Vec::with_capacity(data.encoded_len());
        if data.encode(&mut buf).is_err() {
            return Err(anyhow!("failed to encode source data to binary data"));
        };

        let signature = ed25519_dalek::Signature::from_bytes(signature.signature.as_slice())?;

        let signer = self
            .provider_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing user id"))?;

        let signer_pub_key = ed25519_dalek::PublicKey::from_bytes(signer.get_id()?.as_slice())?;
        Ok(signer_pub_key.verify(&buf, &signature)?)
    }
}
