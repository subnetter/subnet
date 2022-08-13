// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::api_types_extensions::Signed;
use crate::snp::snp_core_types::{
    DialupInfo, EntityId, KeyPair, PreKey, PrivateKey, PrivateProviderIdentityBundle,
    ProviderIdentityBundle, PublicKey,
};
use crate::snp::snp_payments::Address;
use anyhow::{anyhow, Result};
use chrono::Utc;
use ed25519_dalek::{Keypair, SecretKey};
use std::convert::Into;
use x25519_dalek::StaticSecret;

impl PrivateProviderIdentityBundle {
    /// Create a new provider identity and a public identity bundle for the identity, returns the private bundle for it.
    pub fn new(
        dialup_info: &DialupInfo,
        nickname: String,
        payment_address: &Address,
        net_id: u32,
    ) -> Result<PrivateProviderIdentityBundle> {
        let key_pair = Keypair::generate(&mut rand_core::OsRng);
        let pre_key_private = x25519_dalek::StaticSecret::new(&mut rand_core::OsRng);

        PrivateProviderIdentityBundle::new_for_id(
            &key_pair,
            &pre_key_private,
            dialup_info,
            nickname,
            payment_address,
            net_id,
        )
    }

    /// Create a new identity bundle using the provided information
    pub fn new_for_id(
        key_pair: &Keypair,
        pre_key_private: &StaticSecret,
        dialup_info: &DialupInfo,
        nickname: String,
        payment_address: &Address,
        net_id: u32,
    ) -> Result<PrivateProviderIdentityBundle> {
        // prepare provider id keypair
        let id_keypair = KeyPair {
            private_key: Some(PrivateKey {
                key: key_pair.secret.to_bytes().to_vec(),
            }),
            public_key: Some(PublicKey {
                key: key_pair.public.to_bytes().to_vec(),
            }),
        };

        // Create a new per-key pair. Pre keys are x25519 keys and note ed25519 keys...
        let pre_key_pub_data: x25519_dalek::PublicKey = pre_key_private.into();
        let pre_key_public: PublicKey = PublicKey {
            key: pre_key_pub_data.to_bytes().to_vec(),
        };

        let pre_key_pair = KeyPair {
            private_key: Some(PrivateKey {
                key: pre_key_private.to_bytes().to_vec(),
            }),
            public_key: Some(PublicKey {
                key: pre_key_public.key.to_vec(),
            }),
        };

        let bundle_id = Utc::now().timestamp_nanos() as u64;

        let mut pub_bundle = ProviderIdentityBundle {
            time_stamp: bundle_id,
            provider_id: Some(EntityId {
                public_key: Some(id_keypair.public_key.as_ref().unwrap().clone()),
                nickname,
            }),
            address: Some(payment_address.clone()),

            dial_up_info: vec![dialup_info.clone()],
            pre_key: Some(PreKey {
                x2dh_version: "0.1.0".to_string(),
                key: Some(pre_key_pair.clone().public_key.unwrap()),
                key_id: 0, // unused here - same as bundle id
            }),
            one_time_keys: vec![], // empty for now
            profile_image: None,
            current_bond_id: 0,
            provider_signature: None, // needs to be signed next
            net_id,
        };

        pub_bundle.sign(key_pair)?;

        let bundle = PrivateProviderIdentityBundle {
            public_bundle: Some(pub_bundle),
            provider_id_keypair: Some(id_keypair),
            pre_key: Some(pre_key_pair.private_key.unwrap()),
            one_time_keys_pairs: vec![], // unused for node
        };

        Ok(bundle)
    }

    /// Returns the provider identity private key
    pub fn get_provider_private_key(&self) -> Result<SecretKey> {
        let p_id_bytes = self
            .provider_id_keypair
            .as_ref()
            .ok_or_else(|| anyhow!("missing provider id key pair"))?
            .private_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing private key"))?
            .key
            .as_slice();

        match SecretKey::from_bytes(p_id_bytes) {
            Ok(key) => Ok(key),
            Err(_err) => Err(anyhow!("invalid data")),
        }
    }

    pub fn get_provider_id_entity(&self) -> Result<&EntityId> {
        let res = self
            .public_bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing public bundle"))?
            .provider_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing provider id"))?;

        Ok(res)
    }

    /// Get the provuder's payment address
    pub fn get_payment_address(&self) -> Result<Address> {
        Ok(self
            .public_bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing bundke"))?
            .address
            .as_ref()
            .ok_or_else(|| anyhow!("missing address"))?
            .clone())
    }

    /// Return the pre-key private key (x25519::StaticSecret)
    pub fn get_prekey_as_static_secret(&self) -> Result<StaticSecret> {
        let data = self
            .pre_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing pre ket data"))?
            .key
            .as_slice();

        if data.len() != 32 {
            return Err(anyhow!("invalid slice size != 32"));
        }

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(data);
        Ok(StaticSecret::from(bytes))
    }
}
