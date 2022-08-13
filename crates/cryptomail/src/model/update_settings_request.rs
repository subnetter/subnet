//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::consts::MAX_TIME_DRIFT_NANO_SECS;
use crate::model::api::UpdateSettingsRequest;
use crate::model::extensions::Signed;
use anyhow::{anyhow, bail, Result};
use chrono::Utc;
use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::{Keypair, Signer, Verifier};

impl UpdateSettingsRequest {
    pub(crate) async fn validate(&self) -> Result<()> {
        let now = Utc::now().timestamp_nanos();
        if (now - self.time_stamp as i64).abs() > MAX_TIME_DRIFT_NANO_SECS {
            bail!("invalid time stamp")
        }

        self.public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing public key"))?;

        if let Some(public_info) = self.public_account_info.as_ref() {
            public_info.validate().await?
        } else {
            bail!("missing public account info")
        }

        if self.settings.is_none() {
            bail!("missing settings")
        }

        Ok(())
    }
}

impl crate::model::extensions::Signer for UpdateSettingsRequest {
    fn sign(&mut self, signer: &Keypair) -> Result<()> {
        self.signature = vec![];
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;
        self.signature = signer.sign(&buf).as_ref().to_vec();
        Ok(())
    }
}

impl Signed for UpdateSettingsRequest {
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
