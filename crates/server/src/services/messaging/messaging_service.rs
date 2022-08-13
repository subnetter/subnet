//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::terms_service::{GetCurrentTerms, TermsService};
use anyhow::Result;
use base::snp::snp_server_api::provider_core_service_server::ProviderCoreService;
use base::snp::snp_server_api::{
    DrMessage, GetIdentityBundleRequest, GetIdentityBundleResponse, GetTermsOfServiceRequest,
    GetTermsOfServiceResponse, MessageRequest, MessageResponse, NewSessionRequest,
    NewSessionResponse, SubscribeToClientMessagesRequest,
};
use tonic::{Request, Response, Status};
use xactor::Service;

/// ServerMessagingService is a network service which is responsible for handling incoming network requests
/// to this provider MessagingService api
#[derive(Debug, Clone, Copy)]
pub struct ServerMessagingService {}

impl Default for ServerMessagingService {
    fn default() -> Self {
        debug!("MyMessagingService started");
        ServerMessagingService {}
    }
}

/// ServerMessagingService implements the MessagingService trait which defines the grpc methods
/// it provides for clients over the network
#[tonic::async_trait]
impl ProviderCoreService for ServerMessagingService {
    /// A remote node requests to start a new encrypted session with this provider
    /// and sends a first message dr encrypted in the session
    async fn new_session(
        &self,
        request: Request<NewSessionRequest>,
    ) -> Result<Response<NewSessionResponse>, Status> {
        let res = self.new_session_handler(request).await;
        if res.is_err() {
            // let's log the error before returning it to remote host
            error!(
                "Error processing new_session request: {:}",
                res.as_ref().err().unwrap()
            );
        }

        res
    }

    /// A remote client (or another remote Provider) requests to process a new message in an existing DR session
    /// with this server. All higher-level protocol messages are sent using this method.
    async fn message(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        // validate and decode the incoming message

        // parse input
        let msg_req = request.into_inner();
        let message = msg_req
            .message
            .ok_or_else(|| Status::invalid_argument("missing message"))?;

        let res = self.new_message_handler(message).await;
        if res.is_err() {
            let err = res.err().unwrap();
            error!("error handling new message: {:?}", err);
            return Err(err);
        }

        // process the inner typed message by higher-level components
        // and return response to caller or status error
        let resp_msg = ServerMessagingService::process_incoming_msg(res.unwrap())
            .await
            .map_err(|e| {
                Status::internal(format!("failed to process incoming message: {:?}", e))
            })?;

        info!("got message response from component processing the incoming message");
        Ok(Response::new(MessageResponse {
            message: Some(resp_msg),
        }))
    }

    ///////////////////////////

    type SubscribeToClientMessagesStream =
        tokio_stream::wrappers::ReceiverStream<Result<DrMessage, Status>>;

    /// A remote client is requesting to subscribe to a stream of message designated to him.
    /// Note that in the full implementation, this stream will contain meta-data about available messages
    /// to the client, including message size and price and client needs to make non-payment before he can get the message.
    async fn subscribe_to_client_messages(
        &self,
        request: Request<SubscribeToClientMessagesRequest>,
    ) -> Result<Response<Self::SubscribeToClientMessagesStream>, Status> {
        match self
            .handle_client_messages_subscription_request(request.into_inner())
            .await
        {
            Err(e) => Err(Status::internal(format!("internal error: {:?}", e))),
            Ok(resp) => Ok(Response::new(resp)),
        }
    }

    // Returns self's identity bundle for callers that dialed up directly. a public service.
    // This is useful for bootstrapping a network from a list of known providers addresses.
    // rpc GetIdentityBundle(google.protobuf.Empty) returns (GetIdentityBundleResponse);
    async fn get_identity_bundle(
        &self,
        _request: Request<GetIdentityBundleRequest>,
    ) -> Result<Response<GetIdentityBundleResponse>, Status> {
        debug!("Got GetIdentityBundle request",);

        let bundle = ServerMessagingService::get_curr_provider_id_bundle().await?;

        debug!(
            "Returning bundle id: {}",
            bundle.public_bundle.as_ref().unwrap().time_stamp
        );

        Ok(Response::new(GetIdentityBundleResponse {
            bundle: Some(bundle.public_bundle.unwrap()),
        }))
    }

    // Returns provider terms of service for new users - a public service
    async fn get_terms_of_service(
        &self,
        _request: Request<GetTermsOfServiceRequest>,
    ) -> Result<Response<GetTermsOfServiceResponse>, Status> {
        let terms_service = TermsService::from_registry()
            .await
            .map_err(|_| Status::internal("failed to call terms service"))?;

        let terms = terms_service
            .call(GetCurrentTerms)
            .await
            .map_err(|e| Status::internal(format!("failed to call terms: {:?}", e)))?
            .map_err(|_| Status::internal("failed to get terms"))?;

        let response = GetTermsOfServiceResponse { terms: Some(terms) };
        Ok(Response::new(response))
    }
}
