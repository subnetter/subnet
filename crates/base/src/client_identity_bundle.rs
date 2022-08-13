// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::api_types_extensions::Signed;
use crate::snp::snp_core_types::{
    ClientIdentityBundle, EntityId, PreKey, ProviderIdentityBundle, PublicKey,
};
use crate::snp::snp_payments::Address;
use anyhow::{anyhow, Result};
use chrono::Utc;
use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::{Keypair, Signer, Verifier};
use x25519_dalek::StaticSecret;

impl Signed for ClientIdentityBundle {
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

    /// Verify the client public id signature on the bundle
    fn verify_signature(&self) -> Result<()> {
        if self.client_id.is_none() {
            return Err(anyhow!("missing client id"));
        }

        let pub_key_data = self
            .client_id
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
        use prost::Message;
        let mut buf = Vec::with_capacity(data.encoded_len());
        if data.encode(&mut buf).is_err() {
            return Err(anyhow!("failed to encode source data to binary data"));
        };

        let signature = ed25519_dalek::Signature::from_bytes(signature_data.signature.as_slice())?;

        if pub_key.verify(&buf, &signature).is_err() {
            return Err(anyhow!("failed to verify client signature"));
        };

        Ok(())
    }
}

impl ClientIdentityBundle {
    /// Get provider x25519 prekey from an identity bundle
    pub fn get_client_x25519_pre_key(&self) -> Result<x25519_dalek::PublicKey> {
        self.get_client_pre_key()?.as_x25519_pub_key()
    }

    pub fn get_client_entity(&self) -> Result<EntityId> {
        Ok(self.client_id.as_ref().unwrap().clone())
    }

    pub fn get_client_pre_key(&self) -> Result<&PublicKey> {
        self.pre_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing prekey data"))?
            .key
            .as_ref()
            .ok_or_else(|| anyhow!("missing prekey data"))
    }

    /// Get provider identity
    pub fn get_client_id_ed25519_public_key(&self) -> Result<ed25519_dalek::PublicKey> {
        self.get_client_id_public_key()?.as_pub_key()
    }

    pub fn get_client_id_public_key(&self) -> Result<&PublicKey> {
        self.client_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing provider id data"))?
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing public key data"))
    }

    /// Creates a new signed client bundle for the provided identity
    pub fn new(
        key_pair: &Keypair,
        pre_key: &StaticSecret,
        nickname: String,
        provider: &ProviderIdentityBundle,
        payment_address: &Address,
    ) -> Result<ClientIdentityBundle> {
        let client_pre_key_pub_data: x25519_dalek::PublicKey = (pre_key).into();
        let client_pre_key_public: PublicKey = PublicKey {
            key: client_pre_key_pub_data.to_bytes().to_vec(),
        };
        let client_id_pub_key = PublicKey {
            key: key_pair.public.as_ref().to_vec(),
        };

        // our entity with client name as nickname
        let client_entity = EntityId {
            public_key: Some(client_id_pub_key.clone()),
            nickname,
        };

        let mut client_bundle = ClientIdentityBundle {
            time_stamp: Utc::now().timestamp_nanos() as u64,
            client_id: Some(client_entity),
            // for now - we just create an address for the entity's pub key. This should come from wallet.
            address: Some(payment_address.clone()),
            provider_bundle: Some(provider.clone()),
            pre_key: Some(PreKey {
                x2dh_version: "".to_string(),
                key: Some(client_pre_key_public),
                key_id: 0,
            }),
            one_time_keys: vec![],
            profile_image: None,
            signature: None,
            net_id: 0,
        };

        client_bundle.sign(key_pair)?;
        Ok(client_bundle)
    }
}
