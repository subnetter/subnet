//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::consts::BOXES_COL_FAMILY;
use crate::model::types::*;
use anyhow::{anyhow, Result};
use bytes::{BufMut, Bytes, BytesMut};
use db::db_service::{DataItem, DatabaseService, DeleteItem, ReadItem, WriteItem};
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for Account {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "Public info: {}",
            self.public_account_info.as_ref().unwrap()
        )?;
        writeln!(f, "Public key: {}", self.get_public_key())?;
        writeln!(f, "Settings: {}", self.settings.as_ref().unwrap())?;

        Ok(())
    }
}

impl Account {
    /// Update the public listing for an account. This will update the public listing store
    pub(crate) async fn update_public_listing(&self) -> Result<()> {
        // update public listing
        let name = self.get_name();
        if self.settings.as_ref().unwrap().public_list_account {
            info!("Publicly listing account.");
            CryptoMailService::public_list_account(name.as_str(), self.get_public_key()).await
        } else {
            info!("De-listing publicly listed account.");
            CryptoMailService::remove_account_from_public_listing(name.as_str()).await
        }
    }

    // Add cmail tokens to the account
    pub(crate) fn add_cmail_tokens(&mut self, amount: u64) -> Result<()> {
        let rep = self
            .reputation
            .as_mut()
            .ok_or_else(|| anyhow!("missing rep"))?;
        rep.add_cmail_tokens(amount);
        Ok(())
    }

    pub(crate) async fn get_thread_boxes(&self) -> Result<Vec<ThreadBox>> {
        let mut res = vec![];

        let inbox = self
            .load_thread_box(ThreadBoxType::Inbox)
            .await?
            .ok_or_else(|| anyhow!("missing inbox"))?;

        res.push(inbox);

        let archive = self
            .load_thread_box(ThreadBoxType::Archive)
            .await?
            .ok_or_else(|| anyhow!("missing archive-box"))?;

        res.push(archive);

        let sent = self
            .load_thread_box(ThreadBoxType::Sent)
            .await?
            .ok_or_else(|| anyhow!("missing sent-box"))?;

        res.push(sent);

        Ok(res)
    }

    /// Load a thread box (inbox, archive, sent, etc...) from store, returns none if it doesn't exist in store
    pub(crate) async fn load_thread_box(
        &self,
        box_type: ThreadBoxType,
    ) -> Result<Option<ThreadBox>> {
        let res = DatabaseService::read(ReadItem {
            key: self.get_thread_box_store_key(box_type as i32),
            cf: BOXES_COL_FAMILY,
        })
        .await
        .map_err(|_| anyhow!("internal error"))?;

        match res {
            Some(res) => {
                use prost::Message;
                let tb = ThreadBox::decode(res.0.to_vec().as_slice())?;
                Ok(Some(tb))
            }
            None => Ok(None),
        }
    }

    fn get_thread_box_store_key(&self, box_type: i32) -> Bytes {
        let mut buf = BytesMut::with_capacity(12);
        buf.put(self.get_public_key().key.as_slice());
        buf.put_i32(box_type);
        buf.freeze()
    }

    /// Save an account's thread box to store
    pub(crate) async fn save_thread_box(&self, thread_box: ThreadBox) -> Result<()> {
        use prost::Message;
        let mut buf = Vec::with_capacity(thread_box.encoded_len());
        thread_box.encode(&mut buf)?;

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: self.get_thread_box_store_key(thread_box.thread_box_type),
                value: Bytes::from(buf),
            },
            cf: BOXES_COL_FAMILY,
            ttl: 0,
        })
        .await
        .map_err(|_| anyhow!("internal server error - failed to save account"))?;

        Ok(())
    }

    /// Delete thread box from storage. Used when deleting an accounts
    pub(crate) async fn delete_all_thread_boxes(&self) -> Result<()> {
        self.delete_thread_box(ThreadBoxType::Inbox).await?;
        self.delete_thread_box(ThreadBoxType::Sent).await?;
        self.delete_thread_box(ThreadBoxType::Archive).await
    }

    /// Delete an account thread box from storage. Used when deleting an accounts
    async fn delete_thread_box(&self, thread_box_type: ThreadBoxType) -> Result<()> {
        DatabaseService::delete(DeleteItem {
            key: self.get_thread_box_store_key(thread_box_type as i32),
            cf: BOXES_COL_FAMILY,
        })
        .await
    }

    /// Returns the account address
    pub fn get_public_key(&self) -> &PublicKey {
        self.id_pub_key.as_ref().unwrap()
    }

    /// Returns the account mame
    pub fn get_name(&self) -> String {
        self.public_account_info.as_ref().unwrap().name.clone()
    }

    /// Account's public listing status
    pub fn get_public_listing(&self) -> bool {
        self.settings.as_ref().unwrap().public_list_account
    }

    /// Get the currently known prekey for this account
    pub fn get_pre_key(&self) -> Result<&PreKey> {
        self.public_account_info
            .as_ref()
            .ok_or_else(|| anyhow!("missing account info"))?
            .pre_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing prekey"))
    }
}
