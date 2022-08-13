//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::clients_data::service::ClientsDataService;
use crate::services::blockchain_service::{BlockchainService, PublishClientBundleMessage};
use crate::services::provider_id::ProviderIdService;
use crate::services::provider_id_service::GetCurrentIdentityBundle;
use anyhow::{anyhow, Result};
use base::api_types_extensions::Signed;
use base::snp::snp_core_types::{
    ClientServiceData, PrivateProviderIdentityBundle, ProviderSignedClientIdentityBundle,
};
use base::snp::snp_server_api::{
    MessageType, StartServiceRequest, StartServiceResponse, TypedMessage,
};
use base::typed_msgs_dispatcher::{
    Subscribe, TypedMessageHandler, TypedMessagesDispatcher, Unsubscribe,
};
use chrono::prelude::*;
use xactor::*;

/// PublicService is an app-level networking protocol handler that is responsible
/// for handling public service requests sent to this provider.
/// Public service requests are requests sent by any entity on the network.
/// see provider_public_service.proto for the service definitions.
#[derive(Debug, Default)]
pub(crate) struct PublicService {}
impl Service for PublicService {}

#[async_trait::async_trait]
impl Actor for PublicService {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        // subscribe to terms of service messages
        let subscribe_msg = Subscribe {
            message_type: MessageType::StartServiceRequest as i32,
            subscriber: ctx.address().caller(),
        };

        TypedMessagesDispatcher::from_registry()
            .await?
            .call(subscribe_msg)
            .await??;

        debug!("PublicService started and subscribed to handle StartServiceRequest");
        Ok(())
    }

    async fn stopped(&mut self, _ctx: &mut Context<Self>) {
        // Unsubscribe from the dispatcher
        let dispatcher = TypedMessagesDispatcher::from_registry().await.unwrap();
        let _res = dispatcher
            .call(Unsubscribe {
                id: MessageType::StartServiceRequest as i32,
            })
            .await;
    }
}

/// Handle an incoming StartServiceRequest request from a client
#[async_trait::async_trait]
impl Handler<TypedMessageHandler> for PublicService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: TypedMessageHandler,
    ) -> Result<TypedMessage> {
        info!("start service request...");

        // step 1 - verify we know how to handle the message
        if msg.0.msg_type != (MessageType::StartServiceRequest as i32) {
            return Err(anyhow!("Unexpected message type {}", msg.0.msg_type));
        };

        use prost::Message;

        // step 2 - decode the request into the expected object
        let req: StartServiceRequest = StartServiceRequest::decode(msg.0.message.as_slice())
            .map_err(|e| anyhow!("failed to decode client start service request: {:?}", e))?;

        let client_bundle = req
            .bundle
            .ok_or_else(|| anyhow!("missing bundle for req"))?;

        let client_id = client_bundle.get_client_id_ed25519_public_key()?;

        if ClientsDataService::get_client_service_data(&client_id)
            .await?
            .is_some()
        {
            return Err(anyhow!("client is already serviced by this provider"));
        }

        let mut signed_client_bundle = client_bundle.clone();

        let client_data = ClientServiceData {
            service_started: 0,
            service_ended: 0,
            client_identity_bundle: Some(client_bundle),
        };

        // todo: save the signed client service request data in client data - evidence client agreed to terms of service plus how to charge him - fixed monthly, or pay per use?

        info!("saving client data...");

        // update client data and clients list
        ClientsDataService::upsert_client_data(client_data).await?;

        // provider gets his current bundle, add its to the client bundle, signs it and
        // sends back a signed client bundle

        let provider = ProviderIdService::from_registry().await.unwrap();
        let bundle: PrivateProviderIdentityBundle =
            provider.call(GetCurrentIdentityBundle {}).await??;

        signed_client_bundle.provider_bundle = Some(bundle.public_bundle.unwrap());

        let mut signed_bundle = ProviderSignedClientIdentityBundle {
            client_bundle: Some(signed_client_bundle),
            signature: None,
        };

        let key_pair = bundle
            .provider_id_keypair
            .as_ref()
            .unwrap()
            .to_ed2559_kaypair();

        signed_bundle.sign(&key_pair)?;

        info!("publishing client bundle to the blockchain...");

        BlockchainService::publish_client_bundle(PublishClientBundleMessage {
            client_bundle: signed_bundle.clone(),
        })
        .await?;

        let resp = StartServiceResponse {
            bundle: Some(signed_bundle),
        };

        // step 3 - create result typed message and return it
        let mut buff = Vec::with_capacity(resp.encoded_len());
        resp.encode(&mut buff)?;

        Ok(TypedMessage {
            time_stamp: Utc::now().timestamp_nanos() as u64,
            msg_type: MessageType::StartServiceResponse as i32,
            message: buff,
            receiver: None,
            sender: None,
            signature: None,
        })
    }
}
