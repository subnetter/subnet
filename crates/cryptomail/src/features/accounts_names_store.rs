//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::consts::ACCOUNTS_NAMES_COL_FAMILY;
use crate::model::types::PublicKey;
use anyhow::{anyhow, Result};
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, DeleteItem, ReadItem, WriteItem};

/// Account names global registry
#[allow(dead_code)]
impl CryptoMailService {
    pub(crate) async fn update_account_name(
        old_name: &str,
        new_name: &str,
        pub_key: &PublicKey,
    ) -> Result<()> {
        if CryptoMailService::read_account_by_name(old_name)
            .await?
            .is_some()
        {
            CryptoMailService::delete_account_name(old_name).await?;
        }

        CryptoMailService::store_account_by_name(new_name, pub_key).await
    }

    /// Delete an account name from the store
    pub(crate) async fn delete_account_name(name: &str) -> Result<()> {
        let key = Bytes::from(name.to_lowercase().clone());
        DatabaseService::delete(DeleteItem {
            key,
            cf: ACCOUNTS_NAMES_COL_FAMILY,
        })
        .await
        .map_err(|_| anyhow!("internal server error - failed to delete old account name"))
    }

    /// Store an account address by name
    pub(crate) async fn store_account_by_name(name: &str, pub_key: &PublicKey) -> Result<()> {
        let key = Bytes::from(name.to_lowercase().clone());
        DatabaseService::write(WriteItem {
            data: DataItem {
                key,
                value: Bytes::from(pub_key.key.clone()),
            },
            cf: ACCOUNTS_NAMES_COL_FAMILY,
            ttl: 0,
        })
        .await
        .map_err(|_| anyhow!("internal server error - failed to save account"))
    }

    /// Read account by name from global registry
    pub(crate) async fn read_account_by_name(name: &str) -> Result<Option<PublicKey>> {
        let key = Bytes::from(name.to_lowercase());

        match DatabaseService::read(ReadItem {
            key,
            cf: ACCOUNTS_NAMES_COL_FAMILY,
        })
        .await
        {
            Ok(res) => match res {
                Some(data) => Ok(Some(PublicKey {
                    key: data.0.to_vec(),
                })),
                None => Ok(None),
            },
            Err(e) => Err(anyhow!("store error: {}", e)),
        }
    }
}
