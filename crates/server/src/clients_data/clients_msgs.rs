// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::clients_data::service::ClientsDataService;
use anyhow::Result;
use base::snp::snp_server_api::{ClientMessageMetadata, DrMessage};
use bytes::{BufMut, Bytes, BytesMut};
use chrono::prelude::*;
use db::db_service;
use db::db_service::{DataItem, DatabaseService, DeleteItem, ReadItem, WriteItem};
use ed25519_dalek::PublicKey;
use rand_core::{OsRng, RngCore};
use std::convert::From;
use xactor::*;

/// Client messages data store
/// Implementation notes. DB Store Layout
///
///   [ client_id || "cms" ] => ClientMessagesMetadata
///   [ msg_id || "cm" ] => DrMessage
///

// suffix for a client message
const MSG_KEY_SUFFIX: &str = "cm"; // key := msg_id.string().bytes() || cm

///////////////////////////

#[message(result = "Result<()>")]
pub(crate) struct DeleteMessages {
    pub(crate) client_id: PublicKey,
    pub(crate) ids: Vec<u64>,
}

/// Delete message and metadata from store for caller provided messages ids
#[async_trait::async_trait]
impl Handler<DeleteMessages> for ClientsDataService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: DeleteMessages) -> Result<()> {
        let mut meta_data = ClientsDataService::get_client_pending_messages(&msg.client_id).await?;

        // todo: should be 2 batch db operations - delete ids... should be added to the db if rocks supports this
        let mut msg_key = BytesMut::with_capacity(1024);
        for id in msg.ids {
            debug!("deleting message {} from db", id);

            msg_key.clear();
            msg_key.put(id.to_string().as_bytes());
            msg_key.put(MSG_KEY_SUFFIX.as_bytes());

            DatabaseService::delete(DeleteItem {
                key: msg_key.clone().freeze(),
                cf: db_service::PROVIDER_COL_FAMILY,
            })
            .await?;

            // remove msg metadata from meta_data store
            if let Some(idx) = meta_data.messages_metadata.iter().position(|m| m.id == id) {
                meta_data.messages_metadata.remove(idx);
            }
        }

        // store updated client metadata
        ClientsDataService::write_client_pending_messages(&msg.client_id, meta_data).await
    }
}

///////////////////////////

#[message(result = "Result<Vec<DrMessage>>")]
pub(crate) struct LoadMessagesFromStore {
    pub(crate) ids: Vec<u64>,
}

/// Load client messages previously stored
#[async_trait::async_trait]
impl Handler<LoadMessagesFromStore> for ClientsDataService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: LoadMessagesFromStore,
    ) -> Result<Vec<DrMessage>> {
        let mut msg_key = BytesMut::with_capacity(1024);

        let mut res: Vec<DrMessage> = vec![];

        for id in msg.ids {
            // generate message db key
            msg_key.clear();
            msg_key.put(id.to_string().as_bytes());
            msg_key.put(MSG_KEY_SUFFIX.as_bytes());

            let read_item = ReadItem {
                key: msg_key.clone().freeze(),
                cf: db_service::PROVIDER_COL_FAMILY,
            };

            if let Some(data) = DatabaseService::read(read_item).await? {
                debug!("Decoding DrMessage");
                use prost::Message;
                let message: DrMessage = DrMessage::decode(data.0.to_vec().as_slice())?;

                res.push(message)
            } else {
                debug!("DrMessage not found in store");
            }
        }

        Ok(res)
    }
}

////////////////////////////
#[message(result = "Result<ClientMessageMetadata>")]
pub(crate) struct StoreMessageForClient {
    pub(crate) id: PublicKey,
    pub(crate) message: DrMessage,
}

/// Store message and meta-data that should be forwarded to a client
#[async_trait::async_trait]
impl Handler<StoreMessageForClient> for ClientsDataService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: StoreMessageForClient,
    ) -> Result<ClientMessageMetadata> {
        // Add ClientMessageMetadata to data structure of all pending client messages
        let mut client_msgs_metadata =
            ClientsDataService::get_client_pending_messages(&msg.id).await?;

        // Create ClientMessageMetadata for the message with unique id
        let meta_data_id = OsRng.next_u64();
        let meta_data = ClientMessageMetadata {
            id: meta_data_id, // this allow client to request the message indexed by provider by id
            received_date: Utc::now().timestamp_nanos() as u64,
            price: 1, // todo: compute this based on pricing policy in terms and message size
            size: 10, // todo: compute this based on message size
            ttl: 0,   // todo: expire this per service terms - e.g. 2 months...
        };

        // update meta data and store to db
        client_msgs_metadata
            .messages_metadata
            .push(meta_data.clone());
        ClientsDataService::write_client_pending_messages(&msg.id, client_msgs_metadata).await?;

        // Store the message by its meta-data id in the db

        // generate message db key
        let mut msg_key = BytesMut::with_capacity(1024);
        msg_key.put(meta_data_id.to_string().as_bytes());
        msg_key.put(MSG_KEY_SUFFIX.as_bytes());

        use prost::Message;
        let mut buff = Vec::with_capacity(msg.message.encoded_len());
        msg.message.encode(&mut buff)?;
        let data = DataItem {
            key: msg_key.freeze(),
            value: Bytes::from(buff),
        };

        let write_item = WriteItem {
            data,
            cf: db_service::PROVIDER_COL_FAMILY,
            ttl: 0, // todo: this should expire per service terms. e.g. 2 months...
        };

        DatabaseService::write(write_item).await?;

        Ok(meta_data)
    }
}
///////////////////////////
