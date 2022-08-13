//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::consts::PUB_ACCOUNTS_COL_FAMILY;
use crate::model::types::PublicKey;
use anyhow::{anyhow, Result};
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, DeleteItem, WriteItem};

// Public accounts registry
#[allow(dead_code)]
impl CryptoMailService {
    /// Sets an account to private
    pub(crate) async fn remove_account_from_public_listing(name: &str) -> Result<()> {
        DatabaseService::delete(DeleteItem {
            key: Bytes::from(name.to_lowercase()),
            cf: PUB_ACCOUNTS_COL_FAMILY,
        })
        .await
        .map_err(|e| {
            anyhow!(
                "internal server error - failed to remove pub account: {}",
                e
            )
        })?;

        Ok(())
    }

    /// Sets an account as public and register its address
    pub(crate) async fn public_list_account(name: &str, pub_key: &PublicKey) -> Result<()> {
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(name.to_lowercase()),
                value: Bytes::from(pub_key.key.clone()),
            },
            cf: PUB_ACCOUNTS_COL_FAMILY,
            ttl: 0,
        })
        .await
        .map_err(|e| anyhow!("internal server error - failed to save account: {}", e))
    }
}
