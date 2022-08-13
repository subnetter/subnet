//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::model::api::{GetThreadBoxesRequest, GetThreadBoxesResponse};
use crate::model::extensions::{Signed, Validatable};
use crate::model::types::{Account, MessageId, MessageUserdata, PublicKey, Thread, ThreadBoxType};
use anyhow::{anyhow, bail, Result};
use base::hex_utils::hex_string;
use chrono::Utc;
use std::collections::HashSet;
use xactor::*;

impl CryptoMailService {
    // actor handler call wrapper
    pub(crate) async fn get_thread_boxes(
        request: GetThreadBoxesRequest,
    ) -> Result<GetThreadBoxesResponse> {
        let service = CryptoMailService::from_registry().await?;
        service.call(GetThreadBoxesMessage { request }).await?
    }
}

#[message(result = "Result<GetThreadBoxesResponse>")]
pub(crate) struct GetThreadBoxesMessage {
    pub(crate) request: GetThreadBoxesRequest,
}

#[async_trait::async_trait]
impl Handler<GetThreadBoxesMessage> for CryptoMailService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetThreadBoxesMessage,
    ) -> Result<GetThreadBoxesResponse> {
        let request = msg.request;

        info!("verifying GetThreadBoxes() request...");
        request.validate()?;
        request.verify_signature()?;
        info!("request signature verified");

        let pub_key = request.public_key.as_ref().unwrap();

        let opt_account = CryptoMailService::load_account_from_store(pub_key).await?;

        if opt_account.is_none() {
            info!("unknown account w pub key {}", pub_key);
            bail!("unknown account w pub key {}", pub_key)
        }

        // update last login time for account
        let mut account = opt_account.unwrap();
        account.time_last_login = Utc::now().timestamp_nanos() as u64;
        CryptoMailService::store_account(&account).await?;

        let mut threads_boxes = vec![];

        // Note that these 3 thread boxes exist for every account and can't be removed
        if request.thread_boxes & ThreadBoxType::Inbox as u32 != 0 {
            threads_boxes.push(
                account
                    .load_thread_box(ThreadBoxType::Inbox)
                    .await?
                    .unwrap(),
            );
        }

        if request.thread_boxes & ThreadBoxType::Sent as u32 != 0 {
            threads_boxes.push(account.load_thread_box(ThreadBoxType::Sent).await?.unwrap());
        }

        if request.thread_boxes & ThreadBoxType::Archive as u32 != 0 {
            threads_boxes.push(
                account
                    .load_thread_box(ThreadBoxType::Archive)
                    .await?
                    .unwrap(),
            );
        }

        // note: in the future - load additional user's boxes here...

        // collect unique thread ids across all boxes
        let mut thread_ids = HashSet::<&[u8]>::new();
        for thread_box in threads_boxes.iter() {
            info!("Thread box: {}", thread_box);
            for id in thread_box.thread_ids.iter() {
                info!(" adding thread id: {}", hex_string(id));
                thread_ids.insert(id);
            }
        }

        use crate::model::types::Message;

        // load all threads from store
        let mut threads = Vec::<Thread>::new();

        // collect all message ids of all messages in all threads
        let mut messages_ids = HashSet::<MessageId>::new();

        for id in thread_ids.into_iter() {
            // load thread by id
            if let Some(thread) = Thread::read_from_store(&id).await? {
                info!(
                    "GetThreadBoxes(): adding thread to response with id: {}",
                    hex_string(id)
                );

                thread.msgs_ids.iter().for_each(|msg_id| {
                    messages_ids.insert(MessageId {
                        message_thread_id: msg_id.clone(),
                        thread_id: thread.id.clone(),
                    });
                });

                threads.push(thread);
            } else {
                error!("message not found by id in store...");
            }
        }

        // load all message from store
        let mut messages = Vec::<Message>::new();
        for id in messages_ids.into_iter() {
            // load message by id
            if let Some(message) = Message::load_message(&id).await? {
                info!(
                    "GetThreadBoxes(): adding message to response with id: {}",
                    id
                );

                use prost::Message;
                let message_user_data: MessageUserdata =
                    MessageUserdata::decode(message.author_data.as_slice())
                        .map_err(|e| anyhow!("failed to decode message {:?}", e))?;

                let mut buf = Vec::with_capacity(message_user_data.encoded_len());
                message_user_data.encode(&mut buf)?;
                // use base::hex_utils::hex_string;
                // info!(">> message binary hex data: {}", hex_string(buf.as_ref()));
                // info!(">> message binary data: {:?}", buf);

                messages.push(message);
            } else {
                error!("message not found by id in store...");
            }
        }

        // load unique senders and receivers public account infos from store for the response but exclude caller info
        let caller_pub_key = account.get_public_key();
        let mut accounts_ids = HashSet::<PublicKey>::new();
        for message in messages.iter() {
            let message_user_data = message.get_message_user_data().unwrap();

            let sender_pub_key = message_user_data.get_author_public_key().unwrap();
            let receiver_pub_key = message_user_data.get_recipient_pub_key().unwrap();

            if sender_pub_key.key != caller_pub_key.key {
                accounts_ids.insert(sender_pub_key.clone());
            }

            if receiver_pub_key.key != caller_pub_key.key {
                accounts_ids.insert(receiver_pub_key.clone());
            }
        }

        // todo: change to accounts so data includes reputation

        let mut accounts = Vec::<Account>::new();
        for account_id in accounts_ids.into_iter() {
            let account = CryptoMailService::load_account_from_store(&account_id)
                .await?
                .unwrap();
            accounts.push(account)
        }

        info!(" returning {} accounts. ", accounts.len());

        // update account last login time
        account.time_last_login = Utc::now().timestamp_nanos() as u64;
        CryptoMailService::store_account(&account).await?;

        info!(
            "📬 returning thread boxes for {} : {}",
            account.get_name(),
            pub_key,
        );

        Ok(GetThreadBoxesResponse {
            account: Some(account),
            threads_boxes,
            messages,
            accounts,
            threads,
        })
    }
}
