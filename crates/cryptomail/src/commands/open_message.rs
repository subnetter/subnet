//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::consts::READ_PAID_MESSAGE_TOKENS_AMOUNT;
use crate::model::api::{OpenMessageRequest, OpenMessageResponse};
use crate::model::extensions::{Signed, Validatable};
use crate::model::types::{MessageUserdata, PaidActionType};
use anyhow::{anyhow, bail, Result};
use chrono::Utc;
use xactor::*;

impl CryptoMailService {
    pub(crate) async fn open_message_handler(
        request: OpenMessageRequest,
    ) -> Result<OpenMessageResponse> {
        let service = CryptoMailService::from_registry().await?;
        service.call(OpenMessageMessage { request }).await?
    }
}

#[message(result = "Result<OpenMessageResponse>")]
pub struct OpenMessageMessage {
    pub request: OpenMessageRequest,
}

#[async_trait::async_trait]
impl Handler<OpenMessageMessage> for CryptoMailService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: OpenMessageMessage,
    ) -> Result<OpenMessageResponse> {
        let req = msg.request;
        req.validate()?;
        req.verify_signature()?;

        let msg_id = req.message_id.unwrap();
        let opt_msg = crate::model::types::Message::load_message(&msg_id).await?;
        if opt_msg.is_none() {
            bail!("unknown message")
        }

        let mut msg = opt_msg.unwrap();
        let recipient_pub_key = msg
            .get_message_user_data()?
            .get_recipient_pub_key()?
            .clone();

        let request_sender_pub_key = req
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing public key"))?;

        // note that request_sender_address is signed
        if recipient_pub_key.key != request_sender_pub_key.key {
            bail!("unauthorized request. Requester is not message recipient")
        }

        let mut server_data = msg
            .server_data
            .as_mut()
            .ok_or_else(|| anyhow!("missing message server data"))?;

        if server_data.opened {
            bail!("message was already opened")
        }

        server_data.opened = true;
        msg.store_message().await?;

        info!("stored message opened");

        // todo: load opener account and update his account open stats

        let opt_account =
            CryptoMailService::load_account_from_store(request_sender_pub_key).await?;

        if opt_account.is_none() {
            info!("unknown account w pub key {}", request_sender_pub_key);
            bail!("unknown account w pub key {}", request_sender_pub_key)
        }

        // update last login time for account
        let mut account = opt_account.unwrap();
        account.time_last_login = Utc::now().timestamp_nanos() as u64;

        // decode user input to expected typed data
        use prost::Message;
        let message_user_data: MessageUserdata =
            MessageUserdata::decode(msg.author_data.clone().as_slice())?;

        if let Some(payment) = message_user_data.payment.as_ref() {
            if payment.paid_action_type == PaidActionType::Open as i32 {
                // todo: check if message is paid and update
                account
                    .reputation
                    .as_mut()
                    .unwrap()
                    .open_paid_message_opened += 1;
            } else if payment.paid_action_type == PaidActionType::Reply as i32 {
                // todo: check if message is paid and update
                account
                    .reputation
                    .as_mut()
                    .unwrap()
                    .messages_reply_paid_opened += 1;
            }

            // tokens grant for creating a new thread with a paid message
            account.add_cmail_tokens(READ_PAID_MESSAGE_TOKENS_AMOUNT)?;
        }

        CryptoMailService::store_account(&account).await?;

        Ok(OpenMessageResponse {})
    }
}
