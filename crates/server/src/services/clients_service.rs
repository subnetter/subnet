//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::clients_data::service::ClientsDataService;
use crate::services::messaging::new_outgoing_message::new_outgoing_message;
use anyhow::{anyhow, Result};
use base::snp::snp_server_api::*;
use bytes::Bytes;
use common::dr_service::DRService;
use std::collections::HashMap;
use std::convert::From;
use tokio::sync::mpsc;
use tonic::Status;
use xactor::*;

/// ClientsService is a system service that manages the provider's clients
/// It maintains a list of all current provider's clients and knows how to return
/// client data based on a public client id.
/// This includes the current ClientIdentityBundle provided by the client and additional info such as balance, L2
#[derive(Debug)]
pub struct ClientsService {
    /// map from client id to Sender used for sending remote clients messages from other entities
    client_messages_streams: HashMap<Bytes, mpsc::Sender<Result<DrMessage, Status>>>,
}

impl Default for ClientsService {
    fn default() -> Self {
        ClientsService {
            client_messages_streams: HashMap::new(),
        }
    }
}

impl Service for ClientsService {}

#[async_trait::async_trait]
impl Actor for ClientsService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("ClientsService started");
        Ok(())
    }
}

// public api
impl ClientsService {
    pub async fn set_client_message_sender(sender: SetClientMessagesSender) -> Result<()> {
        let service = ClientsService::from_registry().await?;
        service.call(sender).await?
    }

    pub async fn _get_client_message_sender(
        client_id: ed25519_dalek::PublicKey,
    ) -> Result<Option<mpsc::Sender<Result<DrMessage, Status>>>> {
        let service = ClientsService::from_registry().await?;
        service.call(GetClientMessagesSender { client_id }).await?
    }

    pub async fn _remove_client_message_sender(client_id: ed25519_dalek::PublicKey) -> Result<()> {
        let service = ClientsService::from_registry().await?;
        service
            .call(RemoveClientMessagesSender { client_id })
            .await?
    }

    pub async fn send_message_to_client(data: SendMessageToClient) -> Result<()> {
        let service = ClientsService::from_registry().await?;
        service.call(data).await?
    }
}

#[message(result = "Result<()>")]
pub struct SetClientMessagesSender {
    pub client_id: ed25519_dalek::PublicKey,
    pub sender: mpsc::Sender<Result<DrMessage, Status>>,
}

/// SetClientMessagesSender sets a Sender that is able to send messages designated to a client over a stream from this provider
#[async_trait::async_trait]
impl Handler<SetClientMessagesSender> for ClientsService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: SetClientMessagesSender,
    ) -> Result<()> {
        self.client_messages_streams
            .insert(Bytes::from(msg.client_id.as_bytes().to_vec()), msg.sender);

        // Send any pending messages metadata over the stream to the client
        let msgs_metadata = ClientsDataService::get_client_pending_messages(&msg.client_id).await?;

        if !msgs_metadata.messages_metadata.is_empty() {
            use prost::Message;
            let mut buff: Vec<u8> = Vec::with_capacity(msgs_metadata.encoded_len());
            msgs_metadata.encode(&mut buff)?;
            self.send_message_to_client_over_stream(
                &msg.client_id,
                Bytes::from(buff),
                MessageType::ClientMessagesMetadata,
            )
            .await?;
        }

        Ok(())
    }
}

impl ClientsService {
    async fn send_message_to_client_over_stream(
        &mut self,
        client_id: &ed25519_dalek::PublicKey,
        message: Bytes,
        message_type: MessageType,
    ) -> Result<()> {
        if let Some(sender) = self.client_messages_streams.get_mut(client_id.as_ref()) {
            let mut dr = DRService::get_dr_session(*client_id)
                .await?
                .ok_or_else(|| anyhow!("no dr session with client"))?;

            debug!("preparing new message to client...");
            let out_msg = new_outgoing_message(message_type, message, &mut dr, *client_id).await?;

            debug!("sending new message to client over stream...");
            sender
                .send(Ok(DrMessage {
                    data: Some(dr_message::Data::Message(out_msg)),
                }))
                .await?;
        } else {
            warn!("didn't find a remote client subscription on message stream");
        }

        Ok(())
    }
}

#[message(result = "Result<()>")]
pub struct RemoveClientMessagesSender {
    pub client_id: ed25519_dalek::PublicKey,
}

/// RemoveClientMessagesSender removes a message sender for a client.
/// This should be called when client disconnects streaming connection with this server
#[async_trait::async_trait]
impl Handler<RemoveClientMessagesSender> for ClientsService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: RemoveClientMessagesSender,
    ) -> Result<()> {
        self.client_messages_streams.remove(msg.client_id.as_ref());
        Ok(())
    }
}

/// Request service to send a typed message to a client
/// Message is a typed message to be sent
#[message(result = "Result<()>")]
pub struct SendMessageToClient {
    pub client_id: ed25519_dalek::PublicKey,
    pub message_type: MessageType, // Protobuf message type
    pub message: Bytes,            // Protobuf serialized data
}

/// SendMessageToClient is used for sending a typed message to a client.
/// Currently, the message is sent if this service has a Sender for a streaming connection of GetClientMessageResponses with the client. The message will be encrypted in the current dr session between the provider and the client
#[async_trait::async_trait]
impl Handler<SendMessageToClient> for ClientsService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SendMessageToClient) -> Result<()> {
        // use the help method tha tis used by other flows
        self.send_message_to_client_over_stream(&msg.client_id, msg.message, msg.message_type)
            .await
    }
}

#[message(result = "Result<Option<mpsc::Sender<Result<DrMessage, Status>>>>")]
pub struct GetClientMessagesSender {
    pub client_id: ed25519_dalek::PublicKey,
}

/// GetClientMessagesSender gets a message sender for a client. Sender is able to send client messages
/// over a stream. This should be called when client disconnects a streaming connection with this server
/// Returns a Sender for this client or None if no Sender is currently available for the client
#[async_trait::async_trait]
impl Handler<GetClientMessagesSender> for ClientsService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetClientMessagesSender,
    ) -> Result<Option<mpsc::Sender<Result<DrMessage, Status>>>> {
        match self
            .client_messages_streams
            .get(msg.client_id.as_bytes().to_vec().as_slice())
        {
            Some(res) => Ok(Some(res.clone())),
            None => Ok(None),
        }
    }
}
