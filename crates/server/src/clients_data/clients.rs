// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::clients_data::service::ClientsDataService;
use anyhow::{anyhow, Result};
use base::hex_utils::short_hex_string;
use base::snp::snp_core_types::ClientServiceData;
use bytes::{BufMut, Bytes, BytesMut};
use db::db_service;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use ed25519_dalek::PublicKey;
use serde::{Deserialize, Serialize};
use std::convert::From;
use xactor::*;

/// Clients data store
/// Implementation notes. DB Store Layout.
/// [ client_id || "cd" ] => ClientServiceData
/// [ ALL_CLIENT_IDS_KEY ] => ClientsIds
///

/// suffix for ClientServiceData key. key := client_id || KEY_SUFFIX
const CD_KEY_SUFFIX: &str = "cd";

/// key for storage of all serviced client ids
const ALL_CLIENT_IDS_KEY: &str = "serviced_client_ids";

#[message(result = "Result<()>")]
pub struct AddClientId(pub(crate) PublicKey);
/// Add a client id to the store
#[async_trait::async_trait]
impl Handler<AddClientId> for ClientsDataService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: AddClientId) -> Result<()> {
        let read_item = ReadItem {
            key: ALL_CLIENT_IDS_KEY.into(),
            cf: db_service::PROVIDER_COL_FAMILY,
        };

        let mut ids: Vec<PublicKey> = Vec::new();
        if let Some(data) = DatabaseService::read(read_item).await? {
            let data: ClientsIds = bincode::deserialize(&data.0)?;
            for id in data.ids {
                ids.push(id)
            }
        }

        if ids.iter().any(|&x| x == msg.0) {
            debug!("we already know about this client");
            return Ok(());
        }

        // add client id to ids and write to db
        ids.push(msg.0);
        let persisted_data = ClientsIds { ids };
        let data: Bytes = Bytes::from(bincode::serialize(&persisted_data)?);
        let write_item = WriteItem {
            data: DataItem {
                key: ALL_CLIENT_IDS_KEY.into(),
                value: data,
            },
            cf: db_service::PROVIDER_COL_FAMILY,
            ttl: 0, // this data should never expire
        };

        DatabaseService::write(write_item).await
    }
}

///////////////

#[message(result = "Result<Vec<PublicKey>>")]
pub(crate) struct GetAllClientIds;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ClientsIds {
    pub(crate) ids: Vec<PublicKey>,
}
/// Gets all serviced client ids from the db
#[async_trait::async_trait]
impl Handler<GetAllClientIds> for ClientsDataService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetAllClientIds,
    ) -> Result<Vec<PublicKey>> {
        let read_item = ReadItem {
            key: ALL_CLIENT_IDS_KEY.into(),
            cf: db_service::PROVIDER_COL_FAMILY,
        };

        if let Some(data) = DatabaseService::read(read_item).await? {
            let data: ClientsIds = bincode::deserialize(&data.0)?;
            Ok(data.ids)
        } else {
            Ok(Vec::new())
        }
    }
}

#[message(result = "Result<Vec<ClientServiceData>>")]
pub(crate) struct GetClientsServiceData(pub(crate) Vec<PublicKey>);
/// Get service data for one or more clients
#[async_trait::async_trait]
impl Handler<GetClientsServiceData> for ClientsDataService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetClientsServiceData,
    ) -> Result<Vec<ClientServiceData>> {
        unimplemented!()
    }
}

/////////////////

/// Get client bundle based on client id. Returns None if client is unknown
/// to this provider.
/// todo: change this to fuller client info when we have such info such as L2 payment info, balance, etc...
#[message(result = "Result<Option<ClientServiceData>>")]
pub(crate) struct GetClientServiceData(pub(crate) ed25519_dalek::PublicKey);

/// GetClientData handler
#[async_trait::async_trait]
impl Handler<GetClientServiceData> for ClientsDataService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetClientServiceData,
    ) -> Result<Option<ClientServiceData>> {
        debug!("Get client data for: {}", short_hex_string(msg.0.as_ref()));

        // we need to create a key unique to client data to avoid conflicts with other code
        // that uses user ids as keys
        let mut key = BytesMut::with_capacity(1024);
        key.put(CD_KEY_SUFFIX.as_bytes());
        key.put(msg.0.as_ref().to_vec().as_slice());

        let read_item = ReadItem {
            key: key.freeze(),
            cf: db_service::PROVIDER_COL_FAMILY,
        };

        if let Some(res) = DatabaseService::read(read_item).await? {
            debug!("Decoding client data...");
            use prost::Message;
            let client_data = ClientServiceData::decode(res.0.to_vec().as_slice())?;
            debug!("Returning client data");
            Ok(Some(client_data))
        } else {
            debug!("No client data found");
            Ok(None)
        }
    }
}

/////////////////

#[message(result = "Result<()>")]
pub(crate) struct UpsertClientServiceData(pub(crate) ClientServiceData);

/// AddNewClient handler
/// Like other handlers, we assume that message was authenticated
#[async_trait::async_trait]
impl Handler<UpsertClientServiceData> for ClientsDataService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: UpsertClientServiceData,
    ) -> Result<()> {
        let client_data = msg.0;
        let client_bundle = client_data
            .client_identity_bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing id bundle"))?;

        let client_id = client_bundle
            .client_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing client id"))?;

        let client_pub_key = client_id
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing client pub key"))?;

        use prost::Message;
        let mut buff = Vec::with_capacity(client_data.encoded_len());
        client_data.encode(&mut buff)?;

        let mut key = BytesMut::with_capacity(1024);
        key.put(CD_KEY_SUFFIX.as_bytes());
        key.put(client_pub_key.key.to_vec().as_slice());

        let data = DataItem {
            key: key.freeze(),
            value: Bytes::from(buff),
        };

        let write_item = WriteItem {
            data,
            cf: db_service::PROVIDER_COL_FAMILY,
            ttl: 0, // we store this forever
        };

        debug!(
            "Saving data for client id: {}",
            short_hex_string(client_pub_key.key.as_ref())
        );

        DatabaseService::write(write_item).await
    }
}

////////////////////
