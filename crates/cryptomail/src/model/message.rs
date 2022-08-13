//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::consts::MESSAGES_COL_FAMILY;
use crate::features::eth_api_client::EthApiClient;
use crate::model::extensions::Validatable;
use crate::model::types::{
    DepositConfirmation, DepositData, Message, MessageId, MessageServerData, MessageUserdata,
};
use anyhow::{anyhow, bail, Result};
use bytes::Bytes;
use chrono::Utc;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for Message {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // todo: decode binary author data and display...
        // writeln!(f, "Author data: {}", self.author_data.as_ref().unwrap())?;
        writeln!(f, "Server data: {}", self.server_data.as_ref().unwrap())
    }
}

/// validate message
impl Validatable for Message {
    // todo: use me!

    fn validate(&self) -> Result<()> {
        if self.author_data.is_empty() {
            bail!("missing author message data");
        }

        self.server_data
            .as_ref()
            .ok_or_else(|| anyhow!("missing server data"))?;

        if self.signature.is_empty() {
            bail!("missing server data")
        }

        Ok(())
    }
}

impl Message {
    // Create a new message with user data
    pub(crate) fn new(
        message_id: &MessageId,
        message_user_data: &[u8],
        message_user_data_signature: &[u8],
    ) -> Self {
        Message {
            message_id: Some(message_id.clone()),
            author_data: message_user_data.to_vec(),
            server_data: Some(MessageServerData {
                opened: false,
                replied: false,
                deposit_data: Some(DepositData {
                    verify_attempts: 0,
                    last_verify_attempt: 0,
                    deposit_confirmation: None,
                }),
                signature: vec![],
            }),
            signature: message_user_data_signature.to_vec(),
        }
    }

    pub(crate) async fn verify_deposit(&mut self, eth_client: &EthApiClient) -> Result<bool> {
        self.incr_confirmation_attempt();
        match eth_client
            .get_deposit_data(self.message_id.as_ref().unwrap())
            .await
        {
            Ok(res) => {
                if let Some(deposit_confirm) = res {
                    info!("storing deposit confirmation for message");
                    self.set_deposit_confirmation(deposit_confirm);
                    Ok(true)
                } else {
                    info!("no deposit data on chain for message yet");
                    Ok(false)
                }
            }
            Err(e) => {
                bail!(
                    "failed to get deposit data for message {}: {} ",
                    self.message_id.as_ref().unwrap(),
                    e
                )
            }
        }
    }

    // increment message confirmation attempt by one
    fn incr_confirmation_attempt(&mut self) {
        let mut data = self
            .server_data
            .as_mut()
            .unwrap()
            .deposit_data
            .as_mut()
            .unwrap();
        data.verify_attempts += 1;
        data.last_verify_attempt = Utc::now().timestamp_nanos() as u64;
    }

    fn set_deposit_confirmation(&mut self, confirm: DepositConfirmation) {
        self.server_data
            .as_mut()
            .unwrap()
            .deposit_data
            .as_mut()
            .unwrap()
            .deposit_confirmation = Some(confirm);
    }

    pub(crate) fn get_deposit_confirmation(&self) -> Option<&DepositConfirmation> {
        self.server_data
            .as_ref()
            .unwrap()
            .deposit_data
            .as_ref()
            .unwrap()
            .deposit_confirmation
            .as_ref()
    }

    pub(crate) fn _get_deposit_data(&self) -> Option<&DepositData> {
        self.server_data.as_ref().unwrap().deposit_data.as_ref()
    }

    /// Store message in the data store
    pub(crate) async fn store_message(&self) -> Result<()> {
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        if self.encode(&mut buf).is_err() {
            bail!("internal store error")
        };

        let key = self.get_message_user_data()?.get_message_id_bytes()?;

        DatabaseService::write(WriteItem {
            data: DataItem {
                key,
                value: Bytes::from(buf),
            },
            cf: MESSAGES_COL_FAMILY,
            ttl: 0,
        })
        .await
        .map_err(|e| anyhow!("internal server error - failed to save message: {}", e))
    }

    pub fn get_message_user_data(&self) -> Result<MessageUserdata> {
        use prost::Message;
        let res: MessageUserdata = MessageUserdata::decode(self.author_data.as_slice())?;
        Ok(res)
    }

    /// Load a message from store by store key
    pub async fn load_message(id: &MessageId) -> Result<Option<Message>> {
        let key = id.get_message_id_bytes();
        let res = DatabaseService::read(ReadItem {
            key,
            cf: MESSAGES_COL_FAMILY,
        })
        .await?;

        match res {
            None => Ok(None),
            Some(data) => {
                use prost::Message;
                let msg = Message::decode(data.0.as_ref())?;
                Ok(Some(msg))
            }
        }
    }
}
