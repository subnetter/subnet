//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::consts::MAX_TIME_DRIFT_NANO_SECS;
use crate::model::api::DeleteAccountRequest;
use crate::model::extensions::{Signed, Validatable};
use anyhow::{anyhow, bail, Result};
use chrono::Utc;
use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::{Keypair, Signer, Verifier};

impl Validatable for DeleteAccountRequest {
    fn validate(&self) -> Result<()> {
        let now = Utc::now().timestamp_nanos();
        if (now - self.time_stamp as i64).abs() > MAX_TIME_DRIFT_NANO_SECS {
            bail!("invalid time stamp")
        }

        self.public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing public key"))?;

        Ok(())
    }
}

impl crate::model::extensions::Signer for DeleteAccountRequest {
    fn sign(&mut self, signer: &Keypair) -> Result<()> {
        self.signature = vec![];
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;
        self.signature = signer.sign(&buf).as_ref().to_vec();
        Ok(())
    }
}

impl Signed for DeleteAccountRequest {
    fn verify_signature(&self) -> Result<()> {
        let signer = self
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing public key"))?;

        let mut data = self.clone();
        data.signature = vec![];

        use prost::Message;
        let mut buf = Vec::with_capacity(data.encoded_len());
        if data.encode(&mut buf).is_err() {
            return Err(anyhow!("failed to encode source data to binary data"));
        };

        let signature = ed25519_dalek::Signature::from_bytes(self.signature.as_slice())?;
        let signer_pub_key = ed25519_dalek::PublicKey::from_bytes(signer.key.as_ref())?;
        Ok(signer_pub_key.verify(&buf, &signature)?)
    }
}
