// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::consts::ACCOUNTS_CF;
use crate::service::SimpleBlockchainService;
use anyhow::{anyhow, bail, Result};
use base::snp::snp_blockchain::Account;
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};

impl SimpleBlockchainService {
    /// Load blockchain account from store
    pub(crate) async fn read_account(address: &[u8]) -> Result<Option<Account>> {
        let key = address.to_vec();
        if let Some(data) = DatabaseService::read(ReadItem {
            key: Bytes::from(key),
            cf: ACCOUNTS_CF,
        })
        .await?
        {
            use prost::Message;
            let account = Account::decode(data.0.as_ref())?;
            Ok(Some(account))
        } else {
            Ok(None)
        }
    }

    /// Save account in store
    pub(crate) async fn store_account(account: &Account) -> Result<()> {
        let key = account
            .address
            .as_ref()
            .ok_or_else(|| anyhow!("missing account address"))?;

        use prost::Message;
        let mut data = Vec::with_capacity(account.encoded_len());
        if account.encode(&mut data).is_err() {
            bail!("internal server error - failed to encode block")
        };

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(key.data.clone()),
                value: Bytes::from(data.to_vec()),
            },
            cf: ACCOUNTS_CF,
            ttl: 0,
        })
        .await
        .map_err(|e| anyhow!("internal server error - failed to save block: {}", e))?;

        Ok(())
    }
}
