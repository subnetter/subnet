// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use super::server_to_server_service::{SendMessageToServer, ServerToServerService};
use crate::services::messaging::new_outgoing_message::new_outgoing_message;
use anyhow::Result;
use base::snp::snp_core_types::PrivateProviderIdentityBundle;
use base::snp::snp_server_api::provider_core_service_client::ProviderCoreServiceClient;
use base::snp::snp_server_api::{MessageRequest, MessageResponse, TypedMessage};
use double_ratchet::dr::DoubleRatchet;
use tonic::transport::Channel;

/// ServerToServerService facilitates p2p communications with other servers
impl ServerToServerService {
    /// Sends a message to another server using an existing DR session
    pub async fn send_message_in_dr_session(
        &mut self,
        dr_session: &mut DoubleRatchet, // double ratchet session between this provider and remote one
        context: &SendMessageToServer,  // data about the message to send to the provider
        receiver_id: ed25519_dalek::PublicKey,
        _alice_bundle: PrivateProviderIdentityBundle, // our current provider bundle
        receiver_api: &mut ProviderCoreServiceClient<Channel>,
    ) -> Result<TypedMessage> {
        //
        // In this flow, this provider is alice and the other remote provider is bob.
        //

        let message = new_outgoing_message(
            context.message_type,
            context.message.clone(),
            dr_session,
            receiver_id,
        )
        .await?;

        // send the message
        let response: MessageResponse = receiver_api
            .message(MessageRequest {
                message: Some(message),
            })
            .await?
            .into_inner();

        let message = response.message.unwrap();

        let resp_msg = self
            .handle_server_response_message(message, dr_session, receiver_id)
            .await?;

        Ok(resp_msg)
    }
}
