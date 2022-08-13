//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::server_service::SNP_PROTOCOL_VERSION;
use anyhow::{anyhow, Result};
use base::api_types_extensions::Signed;
use base::hex_utils::short_hex_string;
use base::server_config_service::{GetValue, ServerConfigService};
use base::snp::snp_core_types::{
    ApiEndPoint, DialupInfo, PrivateProviderIdentityBundle, ProviderNetInfo,
};
use base::snp::snp_payments::Address;
use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;
use db::db_service;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use db::types::IntDbKey;
use ed25519_dalek::Keypair;
use prost::Message;
use xactor::*;

// db keys
const ACCOUNT_KEYPAIR_KEY: &str = "p_account_keypair_key";
const CURR_ID_BUNDLE_KEY: &str = "p_curr_id_bundle_key";
const ID_KEYPAIR_KEY: &str = "p_id_keypair_key";

/// ProviderIdService maintains provider id data and provides data service to clients via
/// a system service interface.
/// Note that non-actor messages are designed to only be called
/// from the provider's actor handlers. They are not concurrency friendly.
pub struct ProviderIdService {
    // Provider should be initialized with long-term provider id and account id
    // created elsewhere and coming from a config file. For now, we generate these
    // on-demand when provider is created.
    // It may have 0 or more public id bundles with this data
    // Provider's bundles are stored in the db and provider always have
    // one most-recent bundle it can load without having the bundle id.
    // Older bundles can be used by retrieving them from the db by unique id.
    pub provider_id: Option<Keypair>, // pub because needs to be accessed by actor
    pub payments_account_id: Option<Keypair>,
}

/// ProviderIdService implementation

impl Default for ProviderIdService {
    fn default() -> Self {
        ProviderIdService {
            provider_id: None, // this is basically cached data from the db for quicker access
            payments_account_id: None, // this is basically cached data from the db for quicker access
        }
    }
}

/// Note that this methods are designed to only be called by ProviderIdService actor handler implementation
/// and not by other clients. They are not concurrency safe. There are internal helper methods and should
/// be marked as such but rust requires them to have pub accessibility to be used by provider_id code in other files
impl ProviderIdService {
    /// Init this provider. Called from actor started method
    /// Don't forget to call this from unit tests if actor is not started
    pub async fn init(&mut self) -> Result<()> {
        self.init_payment_account().await?;
        self.init_provider_id().await?;

        // this will create a new identity bundle or load the existing one if it was stored
        let _ = self.get_identity_bundle(false).await?;
        Ok(())
    }

    /// Returns signed provider current ProviderNetInfo
    pub async fn get_provider_net_info(&mut self) -> Result<ProviderNetInfo> {
        // todo: this should also be persisted and not computed on every startup
        let dialup_info = self.get_dialup_info().await?;
        let id_bundle = self.get_identity_bundle(false).await?;
        let entity = id_bundle.get_provider_id_entity()?;
        let mut info = ProviderNetInfo {
            provider_id: Some(entity.clone()),
            dial_up_info: Some(dialup_info),
            signature: None,
        };
        info.sign(self.provider_id.as_ref().unwrap())?;

        Ok(info)
    }

    /// get_dialup_info gets the provider current dialup info
    pub async fn get_dialup_info(&self) -> Result<DialupInfo> {
        let config = ServerConfigService::from_registry().await?;

        let name = config
            .call(GetValue("peer_name".into()))
            .await?
            .ok_or_else(|| anyhow!("expected value in config"))?;

        let grpc_server_port = config
            .call(GetValue("grpc_server_port".into()))
            .await?
            .ok_or_else(|| anyhow!("expected value in config"))?
            .parse::<u32>()
            .map_err(|e| anyhow!("invalid port number in config: {:?}", e))?;

        // todo: get supported grpc api version from config

        // todo: take provider ip address from config in the future. For now, localhost only.
        let address = "[::1]";

        Ok(DialupInfo {
            end_point: ApiEndPoint::GrpcWeb2 as i32,
            api_version: SNP_PROTOCOL_VERSION.into(),
            ip_address: address.into(),
            port: grpc_server_port,
            net_id: 0,
            name,
        })
    }

    /// Get an id bundle from the db by a unique bundle id.
    /// Returns the bundle or none if it doesn't exist for the provided id.
    /// This is designed to be used when remote clients use info in this bundle but the
    /// provider already created a new one. In this case, provider needs to load the bundle by id and use it.
    pub async fn get_bundle(&self, id: u64) -> Result<Option<PrivateProviderIdentityBundle>> {
        let key: IntDbKey = id.into();
        let read_item = ReadItem {
            key: key.0,
            cf: db_service::PROVIDER_COL_FAMILY,
        };

        if let Some(data) = DatabaseService::read(read_item).await? {
            let bundle = PrivateProviderIdentityBundle::decode(data.0.as_ref()).map_err(|e| {
                anyhow!(
                    "failed to decode bundle for id {:?} from bytes: {:?}",
                    id,
                    e
                )
            })?;
            Ok(Some(bundle))
        } else {
            Ok(None)
        }
    }

    /// Load the latest provider bundle from the db and return it.
    /// Returns None if no bundle was previously saved to the db.
    async fn load_latest_bundle(&self) -> Result<Option<PrivateProviderIdentityBundle>> {
        let read_item = ReadItem {
            key: CURR_ID_BUNDLE_KEY.into(),
            cf: db_service::PROVIDER_COL_FAMILY,
        };

        if let Some(res) = DatabaseService::read(read_item).await? {
            let id = BigEndian::read_u64(res.0.as_ref());
            if let Some(bundle) = self.get_bundle(id).await? {
                Ok(Some(bundle))
            } else {
                debug!("expected to find bundle in the db with current bundle id");
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// save an id bundle to store and set it as the current one
    async fn save_bundle(&mut self, bundle: &PrivateProviderIdentityBundle) -> Result<()> {
        let mut buf = Vec::with_capacity(bundle.encoded_len());

        bundle.encode(&mut buf)?;

        let key: IntDbKey = bundle
            .public_bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing public bundle"))?
            .time_stamp
            .into();

        let data = DataItem {
            key: key.0.clone(),
            value: Bytes::from(buf),
        };

        // write with key being the unique bundle id
        let write_item = WriteItem {
            data,
            cf: db_service::PROVIDER_COL_FAMILY,
            ttl: 0, // we store this forever
        };

        DatabaseService::write(write_item).await?;

        // debug!("Bundle saved for id: {:?}", key.0.clone());

        // set as current bundle
        let update_curr_bundle = WriteItem {
            data: DataItem {
                key: CURR_ID_BUNDLE_KEY.into(),
                value: key.0,
            },
            cf: db_service::PROVIDER_COL_FAMILY,
            ttl: 0,
        };

        DatabaseService::write(update_curr_bundle).await?;

        Ok(())
    }

    /// Create a new identity bundle from current provider id and account.
    /// This method saves it in the db, set it to the most recent bundle and return it to caller
    async fn create_new_bundle(&mut self) -> Result<PrivateProviderIdentityBundle> {
        // get the current dialup info
        let dialup_info = self.get_dialup_info().await?;

        // ensure provider_id and account_id are loaded
        self.init_payment_account().await?;
        self.init_provider_id().await?;

        let config = ServerConfigService::from_registry().await?;

        // todo: read net_id from config

        let nickname = config
            .call(GetValue("peer_name".into()))
            .await?
            .ok_or_else(|| anyhow!("expected peer_name in config"))?;

        let key_pair = self
            .provider_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing provider id"))?;

        let payment_address = Address {
            data: self.payments_account_id.as_ref().unwrap().public.to_bytes()[12..].to_vec(),
        };

        let pre_key_private = x25519_dalek::StaticSecret::new(&mut rand_core::OsRng);

        let bundle = PrivateProviderIdentityBundle::new_for_id(
            key_pair,
            &pre_key_private,
            &dialup_info,
            nickname,
            &payment_address,
            0,
        )?;

        self.save_bundle(&bundle).await?;

        Ok(bundle)
    }

    /// get_identity_bundle returns the provider's current identity bundle.
    /// It will create a new one if refresh is set to true or a prev created bundle
    /// can't be read from the db.
    pub async fn get_identity_bundle(
        &mut self,
        refresh: bool,
    ) -> Result<PrivateProviderIdentityBundle> {
        if refresh {
            return self.create_new_bundle().await;
        }

        match self.load_latest_bundle().await? {
            Some(b) => Ok(b),
            _ => Ok(self.create_new_bundle().await?),
        }
    }

    /// init_provider_id inits the provider id. Not concurrency safe.
    /// It will attempt to read the id from store and create a new one if none exists in store.
    /// designed to be called only from ProviderIdService actor handler.
    pub async fn init_provider_id(&mut self) -> Result<()> {
        let read_item = ReadItem {
            key: ID_KEYPAIR_KEY.into(),
            cf: db_service::PROVIDER_COL_FAMILY,
        };

        match DatabaseService::read(read_item).await? {
            Some(data) => {
                let res = Keypair::from_bytes(data.0.to_vec().as_slice())?;
                self.provider_id = Some(res);
            }
            None => {
                // no persisted id - create one which lives forever (ttl 0)
                let new_id = crypto::utils::create_key_pair();
                let value = Bytes::from(new_id.to_bytes().to_vec());

                debug!(
                    "my provider pub id: {}",
                    short_hex_string(new_id.public.as_ref())
                );

                let write_item = WriteItem {
                    data: DataItem {
                        key: ID_KEYPAIR_KEY.into(),
                        value,
                    },
                    cf: db_service::PROVIDER_COL_FAMILY,
                    ttl: 0, // we store this forever
                };

                DatabaseService::write(write_item).await?;
                self.provider_id = Some(new_id);
            }
        };

        Ok(())
    }

    /// init_account_id inits the provider payment account. Not concurrency safe.
    /// designed to be called only from ProviderIdService actor handler.
    /// Note: account should come from wallet file. For now we store it in the db.
    pub async fn init_payment_account(&mut self) -> Result<()> {
        let read_item = ReadItem {
            key: ACCOUNT_KEYPAIR_KEY.into(),
            cf: db_service::PROVIDER_COL_FAMILY,
        };

        match DatabaseService::read(read_item).await? {
            Some(data) => {
                let res = Keypair::from_bytes(data.0.to_vec().as_slice())?;
                self.payments_account_id = Some(res);
            }
            None => {
                // no persisted id - create one which lives forever (ttl 0)
                let new_id = crypto::utils::create_key_pair();
                let value = Bytes::from(new_id.to_bytes().to_vec());

                let write_item = WriteItem {
                    data: DataItem {
                        key: ACCOUNT_KEYPAIR_KEY.into(),
                        value,
                    },
                    cf: db_service::PROVIDER_COL_FAMILY,
                    ttl: 0, // we store this forever
                };

                DatabaseService::write(write_item).await?;
                self.payments_account_id = Some(new_id)
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use db::db_service::Destroy;
    use ed25519_dalek::ed25519::signature::Signature;
    use ed25519_dalek::Verifier;

    // helper
    async fn delete_db() {
        let db = DatabaseService::from_registry().await.unwrap();
        let _ = db.call(Destroy).await.unwrap();
    }

    #[tokio::test]
    async fn test_dialup_info() {
        let mut p = ProviderIdService::default();
        p.init().await.unwrap();
        p.get_dialup_info().await.unwrap();
        delete_db().await;
    }

    #[tokio::test]
    async fn test_load_bundle_new_provider() {
        let mut p = ProviderIdService::default();
        p.init().await.unwrap();
        let res = p.load_latest_bundle().await.unwrap();
        assert!(
            res.is_some(),
            "expected latest bundle when provided is loaded"
        );
        delete_db().await;
    }

    #[tokio::test]
    async fn test_new_bundle() {
        let mut p = ProviderIdService::default();
        p.init().await.unwrap();

        let bundle = p.create_new_bundle().await.unwrap();
        let bundle1 = p.load_latest_bundle().await.unwrap().unwrap();
        assert_eq!(
            bundle.public_bundle.unwrap().time_stamp,
            bundle1.public_bundle.unwrap().time_stamp,
            "expected latest bundle to be the newly created one"
        );

        delete_db().await;
    }

    #[tokio::test]
    async fn test_signature_verification() {
        let mut p = ProviderIdService::default();
        p.init().await.unwrap();

        let mut bundle = p.create_new_bundle().await.unwrap().public_bundle.unwrap();

        // get the signature
        let signature = ed25519_dalek::Signature::from_bytes(
            bundle.provider_signature.unwrap().signature.as_slice(),
        )
        .expect("failed to create signature from data");

        // remove signature and get binary data of all other data
        bundle.provider_signature = None;
        let mut buf = Vec::with_capacity(bundle.encoded_len());
        bundle
            .encode(&mut buf)
            .expect("failed to get bundle to binary data");

        // restore public key from data
        let public_key = ed25519_dalek::PublicKey::from_bytes(
            bundle
                .provider_id
                .unwrap()
                .public_key
                .unwrap()
                .key
                .as_slice(),
        )
        .expect("failed to create public key");

        assert!(
            public_key.verify(buf.as_slice(), &signature).is_ok(),
            "sig verification failure"
        );

        delete_db().await;
    }
}
