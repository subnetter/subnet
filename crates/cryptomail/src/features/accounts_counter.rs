//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::consts::{ACCOUNTS_COUNTER_ID_KEY, SYSTEM_COL_FAMILY};
use anyhow::{anyhow, Result};
use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use std::ops::Deref;

impl CryptoMailService {
    /// Returns a new account index counter (og counter)
    pub(crate) async fn get_and_update_og_counter() -> Result<u64> {
        let counter_key = Bytes::from(ACCOUNTS_COUNTER_ID_KEY.as_bytes());

        let res = DatabaseService::read(ReadItem {
            key: counter_key.clone(),
            cf: SYSTEM_COL_FAMILY,
        })
        .await?;

        let mut counter: u64 = match res {
            None => 0_u64,
            Some(data) => LittleEndian::read_u64(data.0.deref()),
        };

        counter += 1;

        let mut buf = [0; 8];
        LittleEndian::write_u64(&mut buf, counter);

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: counter_key,
                value: Bytes::from(buf.to_vec()),
            },
            cf: SYSTEM_COL_FAMILY,
            ttl: 0,
        })
        .await
        .map_err(|_| anyhow!("internal server error - failed to raise counter"))?;

        Ok(counter)
    }
}
