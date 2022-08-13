//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::model::api::CreateAccountRequest;

use crate::consts::MAX_TIME_DRIFT_NANO_SECS;
use crate::model::extensions::{Signed, Validatable};
use anyhow::{anyhow, bail, Result};
use base::hex_utils::hex_string;
use chrono::Utc;
use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::{Keypair, Signer, Verifier};

impl Validatable for CreateAccountRequest {
    fn validate(&self) -> Result<()> {
        let now = Utc::now().timestamp_nanos();
        if (now - self.time_stamp as i64).abs() > MAX_TIME_DRIFT_NANO_SECS {
            bail!("invalid time stamp")
        }

        if self.public_account_info.is_none() {
            bail!("missing account info")
        }

        if self.settings.is_none() {
            bail!("missing settings")
        }

        if self.public_key.is_none() {
            bail!("missing pub key")
        }

        let payment_settings = self
            .public_account_info
            .as_ref()
            .unwrap()
            .payment_settings
            .as_ref();

        if payment_settings.is_none() {
            bail!("missing payment settings")
        }

        let account_name = self.public_account_info.as_ref().unwrap().name.as_ref();
        payment_settings.unwrap().validate(account_name)?;

        Ok(())
    }
}

impl CreateAccountRequest {
    /// Add validate here

    /// Returns the account's pre key as an x25519 public key
    pub fn get_x_pre_key(&self) -> Result<x25519_dalek::PublicKey> {
        self.public_account_info
            .as_ref()
            .ok_or_else(|| anyhow!("missing account info"))?
            .pre_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing account info"))?
            .get_x25519_pub_key()
    }
}

impl crate::model::extensions::Signer for CreateAccountRequest {
    fn sign(&mut self, signer: &Keypair) -> Result<()> {
        self.signature = vec![];
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;
        self.signature = signer.sign(&buf).as_ref().to_vec();
        Ok(())
    }
}

impl Signed for CreateAccountRequest {
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

        // sha512 of data...
        let hash = crypto::utils::sha512(&buf);
        info!("Sha 512 of request bytes: {}", hex_string(&hash));

        let signature = ed25519_dalek::Signature::from_bytes(self.signature.as_slice())?;
        let signer_pub_key = ed25519_dalek::PublicKey::from_bytes(signer.key.as_ref())?;
        Ok(signer_pub_key.verify(&buf, &signature)?)
    }
}
