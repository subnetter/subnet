//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::model;
use crate::model::api::{ReplyRequest, ReplyResponse};
use crate::model::extensions::{Signed, Validatable};
use crate::model::types::{MessageId, MessageUserdata, Thread, ThreadBox, ThreadBoxType};
use anyhow::{anyhow, bail, Result};
use base::hex_utils::hex_string;
use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::Verifier;
use xactor::*;

impl CryptoMailService {
    // actor handler call wrapper
    pub(crate) async fn reply(request: ReplyRequest) -> Result<ReplyResponse> {
        let service = CryptoMailService::from_registry().await?;
        service.call(ReplyMessage { request }).await?
    }
}

#[message(result = "Result<ReplyResponse>")]
pub(crate) struct ReplyMessage {
    pub(crate) request: ReplyRequest,
}

/// User replies to another message sent to him, in an existing thread
#[async_trait::async_trait]
impl Handler<ReplyMessage> for CryptoMailService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: ReplyMessage,
    ) -> Result<ReplyResponse> {
        info!("Handling user reply request...");

        // validate all input and data

        let request = msg.request;
        request.validate()?;
        request.verify_signature()?;

        // verify sender account exists
        let sender_pub_key = request.public_key.unwrap();
        let sender_account_opt =
            CryptoMailService::load_account_from_store(&sender_pub_key).await?;
        if sender_account_opt.is_none() {
            bail!("unrecognized sender account")
        }

        // decode user input to expected typed data
        use prost::Message;
        let message_user_data: MessageUserdata =
            MessageUserdata::decode(request.message_user_data.as_slice())?;

        message_user_data.validate()?;

        // verify message user data author signature
        let signature =
            ed25519_dalek::Signature::from_bytes(request.message_user_data_signature.as_slice())?;

        let signer_pub_key = ed25519_dalek::PublicKey::from_bytes(
            message_user_data
                .sender_public_key
                .as_ref()
                .unwrap()
                .key
                .as_ref(),
        )?;

        signer_pub_key.verify(&request.message_user_data, &signature)?;

        if sender_pub_key.key != signer_pub_key.as_ref() {
            bail!("reply author id must be same as this method caller id")
        }

        let message_id = message_user_data.message_id.as_ref().unwrap();

        info!("Reply message id: {}", message_id);

        let thread_opt = Thread::read_from_store(message_id.thread_id.as_ref()).await?;
        if thread_opt.is_none() {
            bail!("invalid thread. Reply must be to a message in an existing thread")
        }

        let mut thread = thread_opt.unwrap();
        if thread.msgs_ids.contains(&message_id.message_thread_id) {
            bail!("message with the same id already exists in its thread. Please use a unique message id")
        }

        if !thread
            .msgs_ids
            .contains(message_user_data.reply_to.as_ref())
        {
            bail!("invalid reply-to message id - reply message must be a reply to an existing message in its thread")
        }

        // load the message that this reply is a reply for and validate that reply author is that message receiver
        let reply_to_message_id = MessageId {
            thread_id: message_id.thread_id.clone(),
            message_thread_id: message_user_data.reply_to.clone(),
        };

        info!("Reply to message: {}", reply_to_message_id);

        let mut replied_to_message =
            crate::model::types::Message::load_message(&reply_to_message_id)
                .await?
                .ok_or_else(|| anyhow!("unrecognized replied to message"))?;

        let replied_to_message_recipient_pub_key = replied_to_message
            .get_message_user_data()?
            .get_recipient_pub_key()?
            .clone();

        if sender_pub_key.key != replied_to_message_recipient_pub_key.key {
            bail!("reply sender must be the recipient of the message that this reply replies to")
        };

        let replied_to_message_author_pub_key = replied_to_message
            .get_message_user_data()?
            .get_author_public_key()?
            .clone();

        if replied_to_message_author_pub_key.key
            != message_user_data.recipient_public_key.as_ref().unwrap().key
        {
            bail!("reply receiver must be the original message author")
        }

        // verify reply's recipient account exists
        let opt_recipient_account = CryptoMailService::load_account_from_store(
            message_user_data.recipient_public_key.as_ref().unwrap(),
        )
        .await?;

        if opt_recipient_account.is_none() {
            bail!("unknown reply recipient account. Reply recipient must have an active account")
        }

        //
        // Reply validation is done - process the message and store data bellow...
        //

        // update source message reply field and store it
        replied_to_message.server_data.as_mut().unwrap().replied = true;
        replied_to_message.store_message().await?;

        // Store the reply message
        let message =
            model::types::Message::new(message_id, &request.message_user_data, signature.as_ref());
        message.store_message().await?;

        // Add the message id to its thread
        info!("Adding reply to thread {}...", hex_string(&thread.id));
        thread.msgs_ids.push(message_id.message_thread_id.clone());
        thread.store().await?;

        if message_user_data.payment.is_some() {
            // reply message includes a payment - add to unconfirmed messages set so deposits verifier
            // will check the deposit transaction
            info!("added paid reply to unconfirmed messages...");
            self.add_to_unconfirmed_messages(message_user_data.get_message_id()?)
                .await?;
        } else {
            info!("a non-paid reply");

            // add thread to the reply receiver inbox w/o waiting for deposit confirmation
            let recipient_account = opt_recipient_account.unwrap();
            let inbox_data = recipient_account
                .load_thread_box(ThreadBoxType::Inbox)
                .await?;
            let mut inbox = match inbox_data {
                Some(ibx) => ibx,
                None => ThreadBox::new(ThreadBoxType::Inbox),
            };
            if !inbox.has_thread(&thread.id) {
                inbox.thread_ids.insert(0, thread.id.clone());
                recipient_account.save_thread_box(inbox).await?;
                info!("Added thread to recipient inbox")
            } else {
                info!("Thread was already in recipient inbox")
            }
        }

        // add thread to the replier sent items box!
        let sender_account = sender_account_opt.unwrap();

        let sent_box_data = sender_account.load_thread_box(ThreadBoxType::Sent).await?;
        let mut sent_box = match sent_box_data {
            Some(ibx) => ibx,
            None => ThreadBox::new(ThreadBoxType::Sent),
        };
        if !sent_box.has_thread(&thread.id) {
            sent_box.thread_ids.insert(0, thread.id);
            sender_account.save_thread_box(sent_box).await?;
            info!("Added thread to sender's sent items box")
        } else {
            info!("Thread was already in sender's sent items box")
        }

        info!("Reply processing successfully finished - returning to caller");

        Ok(ReplyResponse {
            message_id: Some(message_id.clone()),
        })
    }
}
