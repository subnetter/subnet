//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::model;
use crate::model::api::{NewThreadRequest, NewThreadResult};
use crate::model::extensions::{Signed, Validatable};
use crate::model::types::*;
use anyhow::Result;
use base::hex_utils::hex_string;
use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::Verifier;
use xactor::*;

impl CryptoMailService {
    // actor handler call wrapper
    pub(crate) async fn new_thread(msg: NewThreadMessage) -> Result<NewThreadResult> {
        let service = CryptoMailService::from_registry().await?;
        service.call(msg).await?
    }
}

#[message(result = "Result<NewThreadResult>")]
pub(crate) struct NewThreadMessage {
    pub(crate) request: NewThreadRequest,
}

/// Process a user's request to start a new thread with a first paid messages with another user
#[async_trait::async_trait]
impl Handler<NewThreadMessage> for CryptoMailService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: NewThreadMessage,
    ) -> Result<NewThreadResult> {
        // validation

        let request = msg.request;

        if request.validate().is_err() {
            return Ok(NewThreadResult::MissingData);
        }

        if request.verify_signature().is_err() {
            return Ok(NewThreadResult::InvalidSig);
        }

        // decode user input to expected typed data
        use prost::Message;
        let message_user_data: MessageUserdata =
            MessageUserdata::decode(request.message_user_data.clone().as_slice())?;

        if message_user_data.validate().is_err() {
            return Ok(NewThreadResult::MissingData);
        }

        // verify sender signature
        let signature =
            ed25519_dalek::Signature::from_bytes(request.message_user_data_signature.as_slice())?;

        let message_author_pub_key = ed25519_dalek::PublicKey::from_bytes(
            message_user_data
                .sender_public_key
                .as_ref()
                .unwrap()
                .key
                .as_ref(),
        )?;

        // check that request signer and message author are the same
        let signer_pub_key = message_user_data.sender_public_key.as_ref().unwrap();
        if signer_pub_key.key != message_author_pub_key.as_ref() {
            return Ok(NewThreadResult::InvalidSig);
        }

        if message_author_pub_key
            .verify(&request.message_user_data, &signature)
            .is_err()
        {
            return Ok(NewThreadResult::InvalidSig);
        }

        if message_user_data.has_payment().is_err() {
            info!("new thread message user data must include a payment");
            return Ok(NewThreadResult::MissingData);
        }

        let sender_account_opt = CryptoMailService::load_account_from_store(signer_pub_key).await?;
        if sender_account_opt.is_none() {
            return Ok(NewThreadResult::InvalidSenderAccount);
        }
        let sender_account = sender_account_opt.unwrap();

        let message_id = message_user_data.message_id.as_ref().unwrap();

        if Thread::is_in_store(message_id.thread_id.as_ref()).await? {
            return Ok(NewThreadResult::InvalidThreadId);
        }

        // verify recipient account exists
        let opt_recipient_account = CryptoMailService::load_account_from_store(
            message_user_data.recipient_public_key.as_ref().unwrap(),
        )
        .await?;

        if opt_recipient_account.is_none() {
            return Ok(NewThreadResult::InvalidReceiverAccount);
        }

        let mut recipient_account = opt_recipient_account.unwrap();
        // let _payment = message_user_data.payment.as_ref().unwrap();

        let new_thread_id = message_id.thread_id.clone();
        let new_message_id = message_id.message_thread_id.clone();

        let inbox_data = recipient_account
            .load_thread_box(ThreadBoxType::Inbox)
            .await?;

        let inbox = match inbox_data {
            Some(ibx) => ibx,
            None => ThreadBox::new(ThreadBoxType::Inbox),
        };

        for inbox_thread_id in inbox.thread_ids.iter() {
            if *inbox_thread_id == new_thread_id {
                info!("recipient inbox already got a thread with this id");
                return Ok(NewThreadResult::InvalidThreadId);
            }
        }

        // end of validation - process the valid request:

        // create the message
        let m =
            model::types::Message::new(message_id, &request.message_user_data, signature.as_ref());

        // store the message
        m.store_message().await?;

        self.add_to_unconfirmed_messages(message_id).await?;

        // create the new thread with the message and store it
        let new_thread = Thread {
            id: new_thread_id.clone(),
            msgs_ids: vec![new_message_id],
        };

        // store the thread
        new_thread.store().await?;

        // add the thread to the sender's sent items box
        let sent_items_box_data = sender_account.load_thread_box(ThreadBoxType::Sent).await?;

        // create sent items on demand if it doesn't exist for sender
        let mut sent_items = match sent_items_box_data {
            Some(items_box) => items_box,
            None => ThreadBox::new(ThreadBoxType::Sent),
        };

        // add the new thread to the sender's sent items box
        sent_items.thread_ids.insert(0, new_thread.id);
        sender_account.save_thread_box(sent_items).await?;

        let paid_action_type = message_user_data.payment.unwrap().paid_action_type;

        if paid_action_type == PaidActionType::Open as i32 {
            recipient_account
                .reputation
                .as_mut()
                .unwrap()
                .open_paid_messages_received += 1
        } else if paid_action_type == PaidActionType::Reply as i32 {
            recipient_account
                .reputation
                .as_mut()
                .unwrap()
                .messages_reply_paid_received += 1
        }

        info!(
            "Create new thread from pub_key {} to pub_key {}. Thread id: {}",
            sender_account.get_public_key(),
            recipient_account.get_public_key(),
            hex_string(message_id.thread_id.as_ref())
        );

        Ok(NewThreadResult::Created)
    }
}
