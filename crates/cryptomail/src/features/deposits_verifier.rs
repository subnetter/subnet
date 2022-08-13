//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::consts::{
    DEPOSIT_CONFIRMATIONS_CONFIG_KEY, PAID_MESSAGE_RECEIVED_TOKENS_AMOUNT,
    PAID_MESSAGE_SENT_TOKENS_AMOUNT, PAID_REPLY_RECEIVED_TOKENS_AMOUNT,
    PAID_REPLY_SENT_TOKENS_AMOUNT, SYSTEM_COL_FAMILY, UNCONFIRMED_MSGS_IDS_KEY,
};
use crate::model::api::{GetMessageDepositDataRequest, GetMessageDepositDataResponse};
use crate::model::types::{
    DepositConfirmation, Message, MessageId, MessageUserdata, MessagesIds, ThreadBox, ThreadBoxType,
};
use anyhow::{anyhow, bail, Result};
use base::server_config_service::ServerConfigService;
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::Deref;
use std::time::Duration;
use tokio::task::JoinHandle;
use xactor::*;

impl CryptoMailService {
    /// Returns the on-chain deposit information for a message if such exists
    pub async fn message_deposit_data(
        request: GetMessageDepositDataRequest,
    ) -> Result<GetMessageDepositDataResponse> {
        let message_id = request
            .message_id
            .ok_or_else(|| anyhow!("missing message id"))?;

        let service = CryptoMailService::from_registry().await?;

        let deposit_confirmation = service
            .call(GetMessageDepositDataMessage { message_id })
            .await??;

        Ok(GetMessageDepositDataResponse {
            deposit_confirmation,
        })
    }

    /// Start verify deposits background periodic task
    pub(crate) async fn start_verify_deposits_background_task(
        period_secs: u64,
    ) -> Result<JoinHandle<()>> {
        let address = CryptoMailService::from_registry().await?;
        info!(
            "starting background deposit verifier... period {} secs",
            period_secs
        );
        let join_handle = tokio::task::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(period_secs));
            loop {
                interval.tick().await;

                // todo: check if system service actor was stopped and exit loop in this case
                match address.call(VerifyDepositsMessage {}).await {
                    Ok(res) => match res {
                        Ok(_) => info!("verify deposits task completed"),
                        Err(e) => error!("verify deposits task failed: {}", e),
                    },
                    Err(e) => {
                        error!("internal error trying to execute verify deposits: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(join_handle)
    }
}

#[message(result = "Result<()>")]
pub(crate) struct VerifyDepositsMessage {}

/// VerifyDeposits is designed to be executed periodically by a tokio task - it attempt to get on-chain deposit information from all paid messages which has not been verified yet. When a deposit is verified - remove message from unverified and sets its DepositConfirmation data for the msg obj. When a deposit is not on-chain yet, verify attempt is incremented.
#[async_trait::async_trait]
impl Handler<VerifyDepositsMessage> for CryptoMailService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: VerifyDepositsMessage,
    ) -> Result<()> {
        info!("Deposit verifier process starting...");
        let message_ids = self.load_unconfirmed_messages().await?;
        let mut ids_set: HashSet<MessageId> = HashSet::from_iter(message_ids.messages_ids.clone());

        let eth_client = self
            .eth_api_client
            .as_ref()
            .ok_or_else(|| anyhow!("missing eth client"))?;

        let min_confirms = ServerConfigService::get_u64(DEPOSIT_CONFIRMATIONS_CONFIG_KEY.into())
            .await?
            .unwrap();

        info!("{} unconfirmed paid messages deposits", ids_set.len());
        info!("Ethereum net id: {}", eth_client.get_eth_net_id());
        info!("min block confirmations: {}", min_confirms);

        for id in message_ids.messages_ids {
            info!("processing message deposit for message: {}...", id);
            if let Some(mut msg) = Message::load_message(&id).await? {
                match msg.verify_deposit(eth_client).await {
                    Ok(verified) => {
                        let deposit_confirm = msg.get_deposit_confirmation().unwrap();
                        if verified && deposit_confirm.confirmations >= min_confirms {
                            info!("sufficient confirmations for message {}", id);
                            let _ = ids_set.remove(&id);
                            match self.process_confirmed_message(&msg).await {
                                Ok(_) => info!("message deposit successfully processed"),
                                Err(e) => error!("message deposit processing error: {}", e),
                            }
                        } else {
                            info!("message {} deposit not verified yet", id);
                        }
                    }
                    Err(err) => error!("error verifying deposit: {}", err),
                }
                // store message to update confirmation attempts
                msg.store_message().await?;
            } else {
                error!("didn't find message in store for id: {}", id)
            }
        }

        // ids_set now only has the messages that needs to be confirmed later - store these ids
        self.store_unconfirmed_messages_ids(MessagesIds {
            messages_ids: Vec::from_iter(ids_set),
        })
        .await?;

        Ok(())
    }
}

#[message(result = "Result<Option<DepositConfirmation>>")]
pub struct GetMessageDepositDataMessage {
    message_id: MessageId,
}

/// Verify on-chain deposit for a message
#[async_trait::async_trait]
impl Handler<GetMessageDepositDataMessage> for CryptoMailService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetMessageDepositDataMessage,
    ) -> Result<Option<DepositConfirmation>> {
        let result = self
            .eth_api_client
            .as_ref()
            .ok_or_else(|| anyhow!("missing eth client"))?
            .get_deposit_data(&msg.message_id)
            .await;

        match result {
            Ok(opt_data) => match opt_data {
                Some(data) => {
                    info!("deposit data found on chain: {}", data);
                    Ok(Some(data))
                }
                None => {
                    info!("deposit data not found on chain");
                    Ok(None)
                }
            },
            Err(e) => {
                error!("failed to get deposit data: {}", e);
                bail!("failed to get dep data: {}", e)
            }
        }
    }
}
//////////////////////////

/// todo: store message ids as keys. When read - iterate over all
impl CryptoMailService {
    /// Process a message with a confirmed deposit
    async fn process_confirmed_message(&self, message: &Message) -> Result<()> {
        // decode the original message author data
        use prost::Message;
        let message_user_data: MessageUserdata =
            MessageUserdata::decode(message.author_data.clone().as_slice())?;

        // verify recipient account exists
        let opt_recipient_account = CryptoMailService::load_account_from_store(
            message_user_data.recipient_public_key.as_ref().unwrap(),
        )
        .await?;
        if opt_recipient_account.is_none() {
            bail!("unknown recipient account")
        }
        let mut recipient_account = opt_recipient_account.unwrap();

        let sender_account_opt = CryptoMailService::load_account_from_store(
            message_user_data.sender_public_key.as_ref().unwrap(),
        )
        .await?;
        if sender_account_opt.is_none() {
            bail!("unknown sender account")
        }
        let mut sender_account = sender_account_opt.unwrap();

        // todo: update sender reputation (and maybe receiver's?)

        let thread_id = message_user_data.get_message_id()?.thread_id.clone();
        let is_reply = !message_user_data.reply_to.is_empty();

        if !is_reply {
            // message is a new thread message - add thread to receiver's inbox...
            let inbox_data = recipient_account
                .load_thread_box(ThreadBoxType::Inbox)
                .await?;

            let mut inbox = match inbox_data {
                Some(ibx) => ibx,
                None => ThreadBox::new(ThreadBoxType::Inbox),
            };

            for inbox_thread_id in inbox.thread_ids.iter() {
                if *inbox_thread_id == thread_id {
                    bail!("recipient inbox already got a thread with this id");
                }
            }

            inbox.thread_ids.insert(0, thread_id.clone());
            recipient_account.save_thread_box(inbox).await?;

            sender_account.add_cmail_tokens(PAID_MESSAGE_SENT_TOKENS_AMOUNT)?;
            recipient_account.add_cmail_tokens(PAID_MESSAGE_RECEIVED_TOKENS_AMOUNT)?;

            info!("added new thread to recipient's inbox");
        } else {
            // Message is a reply message in an existing thread - add its thread to recipient inbox

            let inbox_data = recipient_account
                .load_thread_box(ThreadBoxType::Inbox)
                .await?;

            let mut inbox = match inbox_data {
                Some(ibx) => ibx,
                None => ThreadBox::new(ThreadBoxType::Inbox),
            };

            if !inbox.has_thread(&thread_id) {
                inbox.thread_ids.insert(0, thread_id.clone());
                recipient_account.save_thread_box(inbox).await?;
                info!("added reply message's thread to recipient's inbox")
            } else {
                info!("thread of reply was already in recipient's inbox")
            }

            sender_account.add_cmail_tokens(PAID_REPLY_SENT_TOKENS_AMOUNT)?;
            recipient_account.add_cmail_tokens(PAID_REPLY_RECEIVED_TOKENS_AMOUNT)?;

            info!("added reply's thread to recipient's inbox and assigned tokens to users");
        }

        // tokens grant for a new confirmed paid message for both sender and receiver
        CryptoMailService::store_account(&recipient_account).await?;
        CryptoMailService::store_account(&sender_account).await?;

        // store the message - it should have a deposit confirmation
        message.store_message().await?;

        Ok(())
    }

    /// add a message to unconfirmed_message data set
    pub(crate) async fn add_to_unconfirmed_messages(&self, id: &MessageId) -> Result<()> {
        let mut messages_ids = self.load_unconfirmed_messages().await?;
        messages_ids.messages_ids.push(id.clone());
        self.store_unconfirmed_messages_ids(messages_ids).await
    }

    /// load unconfirmed messages ids from store
    async fn load_unconfirmed_messages(&self) -> Result<MessagesIds> {
        let res = DatabaseService::read(ReadItem {
            key: Bytes::from(UNCONFIRMED_MSGS_IDS_KEY.as_bytes()),
            cf: SYSTEM_COL_FAMILY,
        })
        .await?;

        let msgs = match res {
            None => MessagesIds {
                messages_ids: vec![],
            },
            Some(data) => {
                use prost::Message;
                let msgs = MessagesIds::decode(data.0.deref())?;
                msgs
            }
        };

        info!(
            "loaded {} messages with unconfirmed deposits from db",
            msgs.messages_ids.len()
        );

        Ok(msgs)
    }

    /// Store unconfirmed messages ids in the db
    async fn store_unconfirmed_messages_ids(&self, messages_ids: MessagesIds) -> Result<()> {
        use prost::Message;
        let mut buf = Vec::with_capacity(messages_ids.encoded_len());
        messages_ids.encode(&mut buf)?;

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(UNCONFIRMED_MSGS_IDS_KEY.as_bytes()),
                value: Bytes::from(buf),
            },
            cf: SYSTEM_COL_FAMILY,
            ttl: 0,
        })
        .await
    }
}
