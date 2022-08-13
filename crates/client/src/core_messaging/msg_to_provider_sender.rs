// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, bail, Result};
use base::snp::snp_server_api::{MessageRequest, MessageType, TypedMessage};

impl SimpleClient {
    /// Send a message to client's provider using an existing DR session, and return the response.
    /// Preconditions: we have our provider's api service client (net connection)
    ///
    pub(crate) async fn send_message_to_provider(
        &mut self,
        msg_type: MessageType,
        msg_data: Vec<u8>,
    ) -> Result<TypedMessage> {
        let provider_bundle = self
            .provider_bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing provider bundle"))?;

        let ikb = provider_bundle.get_provider_id_ed25519_public_key()?;
        let typed_msg = self.create_typed_message(msg_type, msg_data, ikb)?;
        let message = self.create_message_to_receiver(ikb, typed_msg).await?;

        let provider_api_service = self
            .provider_net_client
            .as_mut()
            .ok_or_else(|| anyhow!("missing provider net client"))?;

        debug!("sending message to provider...");

        let response = provider_api_service
            .message(tonic::Request::new(MessageRequest {
                message: Some(message),
            }))
            .await
            .map_err(|e| anyhow!("got an error response: {:?}", e))?
            .into_inner();

        debug!("got provider response...");

        match response.message {
            Some(response_msg) => {
                let typed_message = SimpleClient::decode_incoming_dr_message(response_msg).await?;
                Ok(typed_message)
            }
            None => bail!("missing response message from provider"),
        }
    }
}
