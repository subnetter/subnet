// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, Result};
use base::snp::snp_server_api::{
    MessageType, SubscribeToClientMessagesRequest, SubscribeToClientMessagesRequestPayload,
};

impl SimpleClient {
    /// Subscribe to messages this client provider has for us
    pub(crate) async fn subscribe_to_provider_messages(&mut self) -> Result<()> {
        // 1. prepare the request for the provider
        let msg = SubscribeToClientMessagesRequestPayload {};
        use prost::Message;
        let mut msg_data = Vec::with_capacity(msg.encoded_len());
        msg.encode(&mut msg_data).unwrap();

        // 2. create the dr message

        let provider_bundle = self
            .provider_bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing provider bundle"))?;

        let ikb = provider_bundle.get_provider_id_ed25519_public_key()?;

        let typed_msg =
            self.create_typed_message(MessageType::SubscribeClientMessages, msg_data, ikb)?;

        let dr_message = self.create_message_to_receiver(ikb, typed_msg).await?;

        // 3. send it via provider subscription public api
        let provider_api_service = self
            .provider_net_client
            .as_mut()
            .ok_or_else(|| anyhow!("missing provider net client"))?;

        debug!("Sending subscription request to provider...");

        let req = SubscribeToClientMessagesRequest {
            dr_message: Some(dr_message),
        };

        let response = provider_api_service
            .subscribe_to_client_messages(tonic::Request::new(req))
            .await
            .map_err(|e| anyhow!("got an error response: {:?}", e))?
            .into_inner();

        debug!("got provider response to subscription request - subscribing to messages...");

        // spawn new task to handle messages incoming on the stream
        tokio::spawn(SimpleClient::provider_messages_handler(response));

        Ok(())
    }
}
