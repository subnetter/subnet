//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::consts::ACCOUNTS_COL_FAMILY;
use crate::model::types::{Account, PublicAccountInfo, PublicKey};
use anyhow::{anyhow, bail, Result};
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, DeleteItem, ReadAllItems, ReadItem, WriteItem};

impl CryptoMailService {
    /// Get all accounts from optional name up to ma results
    pub(crate) async fn read_all_accounts_from_store(
        from: String,
        max_results: u32,
    ) -> Result<Vec<Account>> {
        let mut results = vec![];

        let from_opt = match from.len() {
            0 => None,
            _ => Some(from),
        };

        let data = DatabaseService::read_all_items(ReadAllItems {
            from: from_opt,
            max_results,
            cf: ACCOUNTS_COL_FAMILY,
        })
        .await?;

        for item in data.items {
            use prost::Message;
            let account = Account::decode(item.1.value.as_ref())?;
            results.push(account)
        }

        Ok(results)
    }

    /// Returns account public info by address if it exists
    pub(crate) async fn _account_public_info_by_pub_key(
        pub_key: &PublicKey,
    ) -> Result<Option<PublicAccountInfo>> {
        match CryptoMailService::load_account_from_store(pub_key).await? {
            Some(account) => Ok(account.public_account_info),
            None => Ok(None),
        }
    }

    /// write account to store
    pub(crate) async fn store_account(account: &Account) -> Result<()> {
        use prost::Message;
        let mut buf = Vec::with_capacity(account.encoded_len());
        if account.encode(&mut buf).is_err() {
            bail!("internal server error - failed to encode data")
        };

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(account.get_public_key().key.clone()),
                value: Bytes::from(buf),
            },
            cf: ACCOUNTS_COL_FAMILY,
            ttl: 0,
        })
        .await
        .map_err(|e| anyhow!("internal server error - failed to save account: {}", e))
    }

    /// Load an account from the accounts store
    pub(crate) async fn load_account_from_store(pub_key: &PublicKey) -> Result<Option<Account>> {
        let res = DatabaseService::read(ReadItem {
            key: Bytes::from(pub_key.key.clone()),
            cf: ACCOUNTS_COL_FAMILY,
        })
        .await
        .map_err(|_| anyhow!("internal error"))?;

        match res {
            Some(res) => {
                use prost::Message;
                let account = Account::decode(res.0.to_vec().as_slice())?;
                Ok(Some(account))
            }
            None => Ok(None),
        }
    }

    /// Remove an account from store
    pub(crate) async fn delete_account_from_store(account: &Account) -> Result<()> {
        use prost::Message;
        let mut buf = Vec::with_capacity(account.encoded_len());
        if account.encode(&mut buf).is_err() {
            bail!("internal server error - failed to encode data")
        };

        DatabaseService::delete(DeleteItem {
            key: Bytes::from(account.get_public_key().key.clone()),
            cf: ACCOUNTS_COL_FAMILY,
        })
        .await
        .map_err(|e| {
            anyhow!(
                "internal server error - failed to remove account from store: {}",
                e
            )
        })
    }
}
