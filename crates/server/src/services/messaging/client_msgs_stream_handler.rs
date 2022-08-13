//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::clients_data::service::ClientsDataService;
use crate::services::clients_service::{ClientsService, SetClientMessagesSender};
use crate::services::messaging::messaging_service::ServerMessagingService;
use anyhow::{anyhow, bail, Result};
use base::api_types_extensions::Signed;
use base::snp::snp_server_api::provider_core_service_server::ProviderCoreService;
use base::snp::snp_server_api::SubscribeToClientMessagesRequest;
use tokio::sync::mpsc;

impl ServerMessagingService {
    /// Handles remote client is requesting to subscribe to messages designated
    /// to it by this provider over a network connection
    pub(crate) async fn handle_client_messages_subscription_request(
        &self,
        request: SubscribeToClientMessagesRequest,
    ) -> Result<<Self as ProviderCoreService>::SubscribeToClientMessagesStream> {
        let message = request
            .dr_message
            .ok_or_else(|| anyhow!("missing message from request"))?;

        // validate and decode the request from a DR message
        // for now we assume caller has sent an empty DR message
        let context = self
            .new_message_handler(message)
            .await
            .map_err(|e| anyhow!(format!("error: {:?}", e)))?;

        // verify that client signed on this request before serving
        context.msg.verify_signature()?;

        // so the typed message is a GetClientMessagesRequestPayload but we don't decode it as it is currently empty
        // we used DR to hide the identity of the client over the network for this use case....

        // step 5 - verify that this provider is serving the designated receiver
        let ika = context.msg.get_ika()?;
        if ClientsDataService::get_client_service_data(&ika)
            .await?
            .is_none()
        {
            bail!("unrecognized client - not served by this provider")
        }

        // todo: check that service didn't expire past its grace period for client who asked to stop being served

        let (tx, rx) = mpsc::channel(32);

        let msg = SetClientMessagesSender {
            client_id: context.ika,
            sender: tx,
        };

        ClientsService::set_client_message_sender(msg).await?;

        let res = tokio_stream::wrappers::ReceiverStream::new(rx);

        // return the Receiver
        Ok(res)
    }
}
