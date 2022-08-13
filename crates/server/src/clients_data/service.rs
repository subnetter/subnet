// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::clients_data::clients::{
    AddClientId, GetAllClientIds, GetClientServiceData, GetClientsServiceData,
    UpsertClientServiceData,
};
use crate::clients_data::clients_msgs::{
    DeleteMessages, LoadMessagesFromStore, StoreMessageForClient,
};
use anyhow::{anyhow, Result};
use base::snp::snp_core_types::ClientServiceData;
use base::snp::snp_server_api::{ClientMessageMetadata, ClientMessagesMetadata, DrMessage};
use bytes::{BufMut, Bytes, BytesMut};
use db::db_service;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use ed25519_dalek::PublicKey;
use std::convert::From;
use xactor::*;

// suffix for list of client messages pending for delivery
pub(crate) const MSGS_METADATA_KEY_SUFFIX: &str = "cms"; // key := client_id || cms

/// Clients data service is responsible for handling all provided clients persisted data
#[derive(Debug, Default)]
pub struct ClientsDataService {}

/// ClientsDataService public API
impl ClientsDataService {
    /// Returns serviced client service data
    pub(crate) async fn get_client_service_data(
        client_id: &PublicKey,
    ) -> Result<Option<ClientServiceData>> {
        let service = ClientsDataService::from_registry().await?;
        service.call(GetClientServiceData(*client_id)).await?
    }

    /// Returns the service data for all clients serviced by this provider
    pub(crate) async fn _get_all_clients_service_data() -> Result<Vec<ClientServiceData>> {
        let service = ClientsDataService::from_registry().await?;
        let all_ids: Vec<PublicKey> = service.call(GetAllClientIds {}).await??;
        service.call(GetClientsServiceData(all_ids)).await?
    }

    /// Get all serviced clients ids
    pub(crate) async fn _get_all_client_ids() -> Result<Vec<PublicKey>> {
        let service = ClientsDataService::from_registry().await?;
        service.call(GetAllClientIds {}).await?
    }

    /// Store serviced client data.
    /// This will also update serviced clients ids store.
    /// When a client stops being serviced server should update its service data to indicate
    /// service end date.
    pub(crate) async fn upsert_client_data(data: ClientServiceData) -> Result<()> {
        let id = data
            .client_identity_bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing client data"))?
            .get_client_id_ed25519_public_key()?;

        let service = ClientsDataService::from_registry().await?;
        let _ = service.call(UpsertClientServiceData(data)).await?;
        service.call(AddClientId(id)).await?
    }

    /// Store a new message that should be delivered to a client
    /// This will create an indexed message metadata that can be sent to client
    /// Returns the message's metadata
    pub(crate) async fn store_new_message_for_client(
        id: PublicKey,
        message: DrMessage,
    ) -> Result<ClientMessageMetadata> {
        let service = ClientsDataService::from_registry().await?;
        service.call(StoreMessageForClient { id, message }).await?
    }

    // Load client messages from store based on id
    pub(crate) async fn load_client_messages(ids: Vec<u64>) -> Result<Vec<DrMessage>> {
        let service = ClientsDataService::from_registry().await?;
        service.call(LoadMessagesFromStore { ids }).await?
    }

    /// Delete from storage meta-data and message for caller provider messages ids
    pub(crate) async fn delete_client_messages(client_id: &PublicKey, ids: Vec<u64>) -> Result<()> {
        let service = ClientsDataService::from_registry().await?;
        service
            .call(DeleteMessages {
                client_id: *client_id,
                ids,
            })
            .await?
    }

    /// Returns metadata about all messages pending delivery to a client
    pub(crate) async fn get_client_pending_messages(
        id: &PublicKey,
    ) -> Result<ClientMessagesMetadata> {
        let mut key = BytesMut::with_capacity(1024);
        key.put(id.as_ref());
        key.put(MSGS_METADATA_KEY_SUFFIX.as_bytes());

        let read_item = ReadItem {
            key: key.clone().freeze(),
            cf: db_service::PROVIDER_COL_FAMILY,
        };

        use prost::Message;
        match DatabaseService::read(read_item).await? {
            Some(data) => Ok(ClientMessagesMetadata::decode(data.0.as_ref())?),
            None => Ok(ClientMessagesMetadata {
                messages_metadata: vec![],
            }),
        }
    }

    /// Store indexed meta-data about a client message that is pending delivery to the client
    pub(crate) async fn write_client_pending_messages(
        id: &PublicKey,
        data: ClientMessagesMetadata,
    ) -> Result<()> {
        let mut key = BytesMut::with_capacity(1024);
        key.put(id.as_ref());
        key.put(MSGS_METADATA_KEY_SUFFIX.as_bytes());

        use prost::Message;
        let mut buff = Vec::with_capacity(data.encoded_len());
        data.encode(&mut buff)?;
        let data = DataItem {
            key: key.freeze(),
            value: Bytes::from(buff),
        };

        let write_item = WriteItem {
            data,
            cf: db_service::PROVIDER_COL_FAMILY,
            ttl: 0, // todo: this should expire per service terms. e.g. 2 months...
        };

        DatabaseService::write(write_item).await
    }
}

impl Service for ClientsDataService {}

#[async_trait::async_trait]
impl Actor for ClientsDataService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("ClientsDataService started");
        Ok(())
    }
}
