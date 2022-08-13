//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::consts::{MAX_BINARY_CONTENT_SIZE_BYTES, MAX_TIME_DRIFT_NANO_SECS};
use crate::model::extensions::Validatable;
use crate::model::types::{MessageId, MessageUserdata, PublicKey};
use anyhow::{anyhow, bail, Result};
use base::hex_utils::short_hex_string;
use base::time_utils;
use bytes::Bytes;
use chrono::Utc;
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for MessageUserdata {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Message Id: {}", self.message_id.as_ref().unwrap())?;
        writeln!(
            f,
            "Eph pub key: {}",
            short_hex_string(self.eph_pub_key.as_ref().unwrap().key.as_ref())
        )?;
        writeln!(
            f,
            "Author pub key: {}",
            self.sender_public_key.as_ref().unwrap()
        )?;

        writeln!(f, "To: {}", self.get_recipient_pub_key().unwrap())?;

        writeln!(f, "Sent: {}", time_utils::local_date(self.created))?;

        if let Some(payment) = self.payment.as_ref() {
            writeln!(f, "Payment: {}", payment)?;
        } else {
            writeln!(f, "Free message")?;
        }

        Ok(())
    }
}

impl MessageUserdata {
    pub fn get_eph_pub_key(&self) -> Result<x25519_dalek::PublicKey> {
        let res: x25519_dalek::PublicKey = self
            .eph_pub_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing key data"))?
            .clone()
            .into();

        Ok(res)
    }

    /// Get message globally unique storage key: { thread_id || msg_id }
    pub(crate) fn get_message_id(&self) -> Result<&MessageId> {
        self.message_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing message id"))
    }

    /// Returns the author public key from the message
    pub fn get_author_public_key(&self) -> Result<&PublicKey> {
        self.sender_public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing message id"))
    }

    pub(crate) fn get_recipient_pub_key(&self) -> Result<&PublicKey> {
        self.recipient_public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing message id"))
    }

    // Returns the message store key which is { thread_id || message_id }
    pub(crate) fn get_message_id_bytes(&self) -> Result<Bytes> {
        Ok(self
            .message_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing messageId"))?
            .get_message_id_bytes())
    }

    pub(crate) fn has_payment(&self) -> Result<()> {
        self.payment
            .as_ref()
            .ok_or_else(|| anyhow!("missing payment"))?;
        Ok(())
    }
}

impl Validatable for MessageUserdata {
    fn validate(&self) -> Result<()> {
        let now = Utc::now().timestamp_nanos();
        if (now - self.created as i64).abs() > MAX_TIME_DRIFT_NANO_SECS {
            bail!("invalid time stamp")
        }

        self.eph_pub_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing eph pub key"))?;

        self.recipient_public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing recipient public key"))?;

        self.sender_public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing author public key"))?;

        self.message_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing msg id"))?;

        if self.content.is_empty() {
            bail!("missing message content")
        }

        // the content is encrypted for receiver - we check it is under reasonable size limit to avoid system abuse
        if self.content.len() > MAX_BINARY_CONTENT_SIZE_BYTES {
            bail!(
                "content too big. Must be up to {} kb",
                MAX_BINARY_CONTENT_SIZE_BYTES / 1024
            )
        }

        Ok(())
    }
}
