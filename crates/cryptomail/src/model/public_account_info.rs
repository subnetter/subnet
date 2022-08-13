//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::model::extensions::Signed;
use crate::model::types::PublicAccountInfo;
use anyhow::{anyhow, bail, Result};
use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::{Keypair, Signer, Verifier};

use crate::consts::{ALLOW_HTTP_USER_MEDIA_KEY, MAX_ACCOUNT_NAME, PRE_KEY_LEN};
use base::server_config_service::ServerConfigService;
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for PublicAccountInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Public key: {}, ", self.public_key.as_ref().unwrap())?;
        writeln!(f, "Name: {}, ", self.name)?;
        write!(f, "Pre key: {}", self.pre_key.as_ref().unwrap())?;
        writeln!(f, "Bio: {}", self.profile)?;
        writeln!(f, "Eth name: {}", self.eth_name)?;
        writeln!(f, "Profile url: {}", self.small_profile_image_url)?;

        writeln!(
            f,
            "Payment settings: {}",
            self.payment_settings.as_ref().unwrap()
        )?;

        write!(f, "Profile url (small): {}", self.profile_image_url)
    }
}

impl PublicAccountInfo {
    pub async fn validate(&self) -> Result<()> {
        if self.pre_key.is_none() {
            bail!("missing prekey")
        }

        let pre_key_len = self.pre_key.as_ref().unwrap().key.len();
        if pre_key_len < PRE_KEY_LEN {
            bail!("Pre key invalid length: {} != {}", pre_key_len, PRE_KEY_LEN)
        }

        if self.name.is_empty() {
            bail!("missing account name")
        }

        if self.name.len() > MAX_ACCOUNT_NAME {
            bail!(
                "Account name too long. Max chars length is: {}",
                MAX_ACCOUNT_NAME
            )
        }

        let allow_http_media = ServerConfigService::get_bool(ALLOW_HTTP_USER_MEDIA_KEY.into())
            .await?
            .unwrap();

        // if http is allowed we just skip the checks as this is a de build
        if !allow_http_media {
            let prefix = "https://";

            if !self.custom_profile_background_image_url.is_empty()
                && !self.custom_profile_background_image_url.starts_with(prefix)
            {
                bail!("invalid large profile background url. Must be an https:// url")
            }

            if !self.profile_image_url.is_empty() && !self.profile_image_url.starts_with(prefix) {
                bail!("invalid small profile image url. Must be an https::// url")
            }

            if !self.small_profile_image_url.is_empty()
                && !self.small_profile_image_url.starts_with(prefix)
            {
                bail!("invalid small profile image url. Must be an https:// url")
            }

            for r in self.profile_urls.iter().as_ref() {
                if !r.url.is_empty() && !r.url.starts_with("https://") {
                    bail!("invalid url {}. Must be an https:// url", r.url)
                }
            }
        }

        let payment_settings = self
            .payment_settings
            .as_ref()
            .ok_or_else(|| anyhow!("missing payment settings"))?;

        // todo: enable this when all infos provide good eth signatures..
        payment_settings
            .validate(&self.name)
            .map_err(|e| anyhow!(format!("invalid payment settings: {}", e)))?;

        Ok(())
    }
}

impl crate::model::extensions::Signer for PublicAccountInfo {
    fn sign(&mut self, signer: &Keypair) -> Result<()> {
        self.signature = vec![];
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;
        self.signature = signer.sign(&buf).as_ref().to_vec();
        Ok(())
    }
}

impl Signed for PublicAccountInfo {
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
