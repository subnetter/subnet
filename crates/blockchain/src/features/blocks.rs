// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::consts::{BLOCKCHAIN_CF, BLOCKS_CF, CURRENT_BLOCK_KEY};
use crate::service::SimpleBlockchainService;
use anyhow::{anyhow, bail, Result};
use base::snp::snp_blockchain::Block;
use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use std::ops::Deref;

impl SimpleBlockchainService {
    pub(crate) async fn store_block(block: &Block) -> Result<()> {
        let mut block_id = [0; 8];
        LittleEndian::write_u64(&mut block_id, block.id);

        use prost::Message;
        let mut block_data = Vec::with_capacity(block.encoded_len());
        if block.encode(&mut block_data).is_err() {
            bail!("internal server error - failed to encode block")
        };

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(block_id.to_vec()),
                value: Bytes::from(block_data.to_vec()),
            },
            cf: BLOCKS_CF,
            ttl: 0,
        })
        .await
        .map_err(|e| anyhow!("internal server error - failed to store block: {}", e))?;

        Ok(())
    }

    pub(crate) async fn read_block(block_id: u64) -> Result<Option<Block>> {
        let mut buf = [0; 8];
        LittleEndian::write_u64(&mut buf, block_id);
        if let Some(data) = DatabaseService::read(ReadItem {
            key: Bytes::from(buf.to_vec()),
            cf: BLOCKS_CF,
        })
        .await?
        {
            use prost::Message;
            let block = Block::decode(data.0.as_ref())?;
            Ok(Some(block))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn read_current_block_id() -> Result<u64> {
        if let Some(data) = DatabaseService::read(ReadItem {
            key: Bytes::from(CURRENT_BLOCK_KEY.as_bytes()),
            cf: BLOCKCHAIN_CF,
        })
        .await?
        {
            Ok(LittleEndian::read_u64(data.0.deref()))
        } else {
            Ok(0)
        }
    }

    pub(crate) async fn write_current_block_id(id: u64) -> Result<()> {
        let mut buf = [0; 8];
        LittleEndian::write_u64(&mut buf, id);

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(CURRENT_BLOCK_KEY.as_bytes()),
                value: Bytes::from(buf.to_vec()),
            },
            cf: BLOCKCHAIN_CF,
            ttl: 0,
        })
        .await
        .map_err(|e| anyhow!("internal server error - failed to save block: {}", e))?;

        Ok(())
    }
}
