//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::consts::{MSG_ID_LEN, MSG_THREAD_ID_LEN, THREAD_ID_LEN};
use crate::model::extensions::Validatable;
use crate::model::types::MessageId;
use anyhow::{bail, Result};
use base::hex_utils::hex_string;
use bytes::{BufMut, Bytes, BytesMut};
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for MessageId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Thread id: {}, ", hex_string(self.thread_id.as_ref()))?;
        write!(
            f,
            "Message thread id: {}, ",
            hex_string(self.message_thread_id.as_ref())
        )?;
        writeln!(
            f,
            "Db Id: {}",
            hex_string(self.get_message_id_bytes().as_ref())
        )
    }
}

impl Validatable for MessageId {
    fn validate(&self) -> Result<()> {
        if self.message_thread_id.len() != MSG_THREAD_ID_LEN {
            bail!("invalid message thread id length - expected 8 bytes");
        }

        if self.thread_id.len() != THREAD_ID_LEN {
            bail!("invalid message thread id length - expected 8 bytes");
        }

        Ok(())
    }
}

impl MessageId {
    pub(crate) fn get_message_id_bytes(&self) -> Bytes {
        let mut buf = BytesMut::with_capacity(MSG_ID_LEN);
        buf.put(self.thread_id.clone().as_ref());
        buf.put(self.message_thread_id.clone().as_ref());
        buf.freeze()
    }
}
