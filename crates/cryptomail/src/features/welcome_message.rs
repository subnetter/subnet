//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::model;
use crate::model::types::{
    Account, Compression, ContentItem, MessageContent, MessageId, MessageUserdata, MimeType,
    PublicKey, Thread, ThreadBox, ThreadBoxType,
};
use anyhow::Result;
use byteorder::{ByteOrder, LittleEndian};
use chrono::Utc;
use ed25519_dalek::ed25519::signature::rand_core::{OsRng, RngCore};
use ed25519_dalek::Signer;

impl CryptoMailService {
    /// Send welcome message to a recipient
    pub(crate) async fn create_inbox_with_welcome_message(recipient: &Account) -> Result<()> {
        info!("preparing welcome message...");

        let mut thread_id = [0u8; 8];
        OsRng.fill_bytes(&mut thread_id);

        let id = MessageId {
            message_thread_id: [0u8; 8].to_vec(),
            thread_id: thread_id.to_vec(),
        };

        // let admin = CryptoMailService::admin_account();
        let admin_key_pair = CryptoMailService::admin_account_key_pair();
        let admin_pub_key = PublicKey {
            key: admin_key_pair.public.as_ref().to_vec(),
        };

        let recipient_pre_key = recipient.get_pre_key()?;
        let simple_message = CryptoMailService::get_welcome_message_content(recipient);
        let (x_eph_pub_key, enc_message) = simple_message.encrypt_message(recipient_pre_key)?;

        let eph_pub_key = PublicKey {
            key: x_eph_pub_key.to_bytes().as_ref().to_vec(),
        };

        info!(
            "welcome message thread id: {}",
            LittleEndian::read_u64(id.message_thread_id.as_ref())
        );

        info!(
            "welcome thread id: {}",
            LittleEndian::read_u64(id.thread_id.as_ref())
        );

        let message_user_data = MessageUserdata {
            message_id: Some(id.clone()),
            sender_public_key: Some(admin_pub_key.clone()),
            created: Utc::now().timestamp_nanos() as u64,
            payment: None,
            reply_to: [0u8; 8].to_vec(),
            recipient_public_key: Some(recipient.get_public_key().clone()),
            recipient_pre_key_id: recipient_pre_key.id.clone(),
            eph_pub_key: Some(eph_pub_key),
            content: enc_message,
        };

        use prost::Message;
        let mut buf = Vec::with_capacity(message_user_data.encoded_len());
        message_user_data.encode(&mut buf)?;
        let signature = admin_key_pair.sign(&buf);

        // create Message and store it
        let message = model::types::Message::new(&id, &buf, &signature.as_ref());
        message.store_message().await?;

        // create thread and store it
        let new_thread = Thread {
            id: thread_id.to_vec(),
            msgs_ids: vec![id.message_thread_id.to_vec()],
        };
        new_thread.store().await?;

        // load inbox or create new one on demand
        let mut inbox = match recipient.load_thread_box(ThreadBoxType::Inbox).await? {
            Some(inbox) => inbox,
            None => ThreadBox {
                thread_box_type: ThreadBoxType::Inbox as i32,
                thread_ids: vec![],
            },
        };

        // add the thread to recipient's inbox and store it
        inbox.thread_ids.push(new_thread.id.clone());
        recipient.save_thread_box(inbox).await?;

        info!(
            "sent welcome message from admin to {}",
            recipient.get_name()
        );

        // add thread to admin sent items
        let admin_account = CryptoMailService::load_account_from_store(&admin_pub_key)
            .await?
            .unwrap();

        let mut sent_items = admin_account
            .load_thread_box(ThreadBoxType::Sent)
            .await?
            .unwrap();

        sent_items.thread_ids.push(new_thread.id);
        admin_account.save_thread_box(sent_items).await?;

        Ok(())
    }

    fn get_welcome_message_content(recipient: &Account) -> MessageContent {
        let account_url = format!("/u/{}", recipient.get_name());

        MessageContent {
            subject: Some(ContentItem {
                mime_type: MimeType::TextUtf8 as i32,
                compression: Compression::Uncompressed as i32,
                data: "🖖 Welcome to cmail!".as_bytes().to_vec(),
            }),
            body: Some(ContentItem {
                mime_type: MimeType::TextMd as i32,
                compression: Compression::Uncompressed as i32,
                data: format!(
                    "### 👋 Welcome {} to your new cmail account!\n \
                    I'm really glad to see you here!.\n\n \
                    #### Some tips for getting the most out of cmail...\n\n\
                    - Share your public cmail url so others can send you paid messages. Check [your account permanent friendly url]({}).\n\
                    - Send a paid message to someone you want to read or reply to your message. All you need is their cmail account name.\n\
                    - Find people you may want to send a message to in the [public users directory](/users).\n \
                    - Consider making your profile [more personal](/settings) so others can tell who you are.\n \
                    - Be sure to check the [FAQ](/faq) and [join the cmail community](https://discord.gg/dzEhCHsyX5).\n\n \
                    Best, your friendly cmail admin."
                    ,recipient.get_name(),
                    account_url
                )
                .as_bytes()
                .to_vec(),
            }),
            media_items: vec![],
        }
    }
}
