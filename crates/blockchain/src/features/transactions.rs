// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::consts::TRANSACTIONS_CF;
use crate::service::SimpleBlockchainService;
use anyhow::{anyhow, bail, Result};
use base::snp::snp_blockchain::TransactionInfo;
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};

// todo: store TransactionInfo objects which include the tx block and not just the Transaction itself

impl SimpleBlockchainService {
    /// Load blockchain account from store
    pub(crate) async fn read_transaction(id: &Vec<u8>) -> Result<Option<TransactionInfo>> {
        if let Some(data) = DatabaseService::read(ReadItem {
            key: Bytes::from(id.clone()),
            cf: TRANSACTIONS_CF,
        })
        .await?
        {
            use prost::Message;
            let transaction_info = TransactionInfo::decode(data.0.as_ref())?;
            Ok(Some(transaction_info))
        } else {
            Ok(None)
        }
    }

    /// Save account in store
    pub(crate) async fn store_transaction(transaction_info: &TransactionInfo) -> Result<()> {
        let transaction = transaction_info
            .transaction
            .as_ref()
            .ok_or_else(|| anyhow!("missing tx"))?;

        let id = transaction.get_tx_id()?;

        use prost::Message;
        let mut data = Vec::with_capacity(transaction_info.encoded_len());
        if transaction_info.encode(&mut data).is_err() {
            bail!("internal server error - failed to encode transaction")
        };

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(id),
                value: Bytes::from(data.to_vec()),
            },
            cf: TRANSACTIONS_CF,
            ttl: 0,
        })
        .await
        .map_err(|e| anyhow!("internal server error - failed to save tx: {}", e))?;

        Ok(())
    }
}
