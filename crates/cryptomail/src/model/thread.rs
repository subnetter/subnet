//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//
use crate::consts::THREADS_COL_FAMILY;
use crate::model::types::{MessageId, Thread};
use anyhow::{anyhow, bail, Result};
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};

use base::hex_utils::hex_string;
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for Thread {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Thread id: {}. ", hex_string(self.id.as_ref()))?;
        writeln!(f, "Total messages: {}", self.msgs_ids.len())?;
        write!(f, "Messages ids: {:?}", self.msgs_ids)
    }
}

impl Thread {
    // Get a MessageId for a thread message identified by a message thread id
    pub fn get_message_id(&self, message_thread_id: &[u8]) -> MessageId {
        MessageId {
            message_thread_id: message_thread_id.to_vec(),
            thread_id: self.id.clone(),
        }
    }

    // Returns a vector of all MessageIds of message in this read
    pub fn get_messages_ids(&self) -> Vec<MessageId> {
        let mut res = vec![];
        for id in self.msgs_ids.iter() {
            let message_id = self.get_message_id(id);
            res.push(message_id)
        }
        res
    }

    // Returns ture iff thread is in store
    pub(crate) async fn is_in_store(thread_id: &[u8]) -> Result<bool> {
        let key = thread_id.to_vec();
        Ok(DatabaseService::read(ReadItem {
            key: Bytes::from(key),
            cf: THREADS_COL_FAMILY,
        })
        .await?
        .is_some())
    }

    pub(crate) async fn read_from_store(thread_id: &[u8]) -> Result<Option<Thread>> {
        let key = thread_id.to_vec().clone();
        if let Some(data) = DatabaseService::read(ReadItem {
            key: Bytes::from(key),
            cf: THREADS_COL_FAMILY,
        })
        .await?
        {
            use prost::Message;
            let thread = Thread::decode(data.0.as_ref())?;
            Ok(Some(thread))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn store(&self) -> Result<()> {
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        if self.encode(&mut buf).is_err() {
            bail!("internal server error")
        };

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(self.id.clone()),
                value: Bytes::from(buf),
            },
            cf: THREADS_COL_FAMILY,
            ttl: 0,
        })
        .await
        .map_err(|e| anyhow!("internal server error - failed to store thread: {}", e))
    }
}
