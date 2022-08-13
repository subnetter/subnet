// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::channels::channel_creator::CreateNewChannel;
use crate::channels::channel_msg_publisher::PublishNewChannelMessage;
use crate::channels::channel_subscriber::SubscribeToChannel;
use crate::channels::channel_unsubscriber::UnsubscribeFromChannel;

use crate::paid_content::item_buyer::BuyItem;
use crate::paid_content::item_creator::CreatePaidItem;
use crate::paid_content::list_items_sender::ListItems;
use crate::services::add_other_client::AddOtherClientBundle;
use crate::services::set_blockchain_service::SetBlockchainService;
use crate::services::set_provider::SetProvider;
use crate::simple_client::SimpleClient;
use crate::user_to_user_messaging::text_msg_sender::SendTextMessage;
use anyhow::Result;
use base::snp::snp_core_types::*;
use base::snp::upsetter_simple_client::simple_client_user_service_server::SimpleClientUserService;
use base::snp::upsetter_simple_client::*;
use bytes::Bytes;
use tonic::{Request, Response, Status};
use xactor::*;

/// SimpleClientGrpcService is a network service which provides a client grpc api
/// We use it to simulate user actions with a client for use cases such as setting service provider and
/// sending a text message to another client.
#[derive(Debug)]
pub(crate) struct SimpleClientGrpcService {}

impl Default for SimpleClientGrpcService {
    fn default() -> Self {
        debug!("SimpleClientGrpcService started");
        SimpleClientGrpcService {}
    }
}

impl SimpleClientGrpcService {
    /// Subscribe the client on behalf of its user to a channel
    /// Helper method, used to join groups and subscribe to a status updates channels
    async fn user_subscribe_to_channel(&self, channel: ChannelBundle) -> Result<(), Status> {
        let client = SimpleClient::from_registry()
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?;

        match client
            .call(SubscribeToChannel { channel })
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?
        {
            Ok(()) => Ok(()),
            Err(e) => {
                error!("{}", e);
                Err(Status::internal(format!(">>> internal error: {:?}", e)))
            }
        }
    }

    async fn user_unsubscribe_from_channel(&self, channel: ChannelBundle) -> Result<(), Status> {
        let client = SimpleClient::from_registry()
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?;

        match client
            .call(UnsubscribeFromChannel { channel })
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?
        {
            Ok(()) => Ok(()),
            Err(e) => {
                error!("{}", e);
                Err(Status::internal(format!(">>> internal error: {:?}", e)))
            }
        }
    }
}

/// SimpleClientGrpcService implements the SimpleClientService trait which defines the grpc
/// methods in the client's public api.
#[tonic::async_trait]
impl SimpleClientUserService for SimpleClientGrpcService {
    /// Set the provider for this client.
    /// Instructs the client to connect to this provider, set it as its service provider and maintain a DR session with it.
    async fn user_set_provider(
        &self,
        request: Request<UserSetProviderRequest>,
    ) -> Result<Response<UserSetProviderResponse>, Status> {
        let client = SimpleClient::from_registry()
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?;

        let dialup_info = request
            .into_inner()
            .dialup_info
            .ok_or_else(|| Status::invalid_argument("missing dialup info"))?;

        match client
            .call(SetProvider { dialup_info })
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?
        {
            Ok(bundle) => Ok(Response::new(UserSetProviderResponse {
                client_bundle: Some(bundle),
            })),
            Err(e) => Err(Status::internal(format!("internal error: {:?}", e))),
        }
    }

    // Set other client bundle on behalf of the client's user
    async fn user_add_other_client_bundle(
        &self,
        request: tonic::Request<ProviderSignedClientIdentityBundle>,
    ) -> Result<tonic::Response<UserAddOtherClientBundleResponse>, tonic::Status> {
        let client = SimpleClient::from_registry()
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?;

        match client
            .call(AddOtherClientBundle(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?
        {
            Ok(()) => Ok(Response::new(UserAddOtherClientBundleResponse {})),
            Err(e) => Err(Status::internal(format!("Internal error: {:?}", e))),
        }
    }

    /// A Remote API client (user, script, test, etc...) requests to trigger flow for this client to send a text message to another client over SNP.
    /// Assumes client is already provided by a provider. This is used for testing instead of client's user input.
    async fn user_send_text_message(
        &self,
        request: Request<UserSendTextMessageRequest>,
    ) -> Result<Response<UserSendTextMessageResponse>, Status> {
        let client = SimpleClient::from_registry()
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?;

        let inner_req = request.into_inner();

        let receiver_id: Vec<u8> = inner_req
            .other_client_id
            .ok_or_else(|| Status::invalid_argument("missing receiver id"))?
            .get_id()
            .map_err(|_| Status::invalid_argument("missing pub key"))?
            .clone();

        match client
            .call(SendTextMessage {
                message: inner_req.user_text,
                receiver_id: Bytes::from(receiver_id),
                reply_to: inner_req.reply_to,
            })
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?
        {
            Ok(message_id) => Ok(Response::new(UserSendTextMessageResponse { message_id })),
            Err(_) => Err(Status::internal("internal error")),
        }
    }

    // Create a new status update channel on behalf of the user
    async fn user_create_status_update_channel(
        &self,
        request: Request<UserCreateStatusUpdateChannelRequest>,
    ) -> Result<Response<UserCreateStatusUpdateChannelResponse>, Status> {
        let client = SimpleClient::from_registry()
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?;

        match client
            .call(CreateNewChannel {
                name: request.into_inner().channel_name,
                channel_type: ChannelType::StatusFeed,
                description: "My Upsetter Status Updates".to_string(),
            })
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?
        {
            Ok(bundle) => Ok(Response::new(UserCreateStatusUpdateChannelResponse {
                channel_bundle: Some(bundle),
            })),
            Err(e) => Err(Status::internal(format!("Internal error: {:?}", e))),
        }
    }

    async fn user_subscribe_to_status_updates(
        &self,
        request: Request<UserSubscribeRequest>,
    ) -> Result<Response<UserSubscribeResponse>, Status> {
        let channel = request
            .into_inner()
            .channel_bundle
            .ok_or_else(|| Status::invalid_argument("missing channel bundle"))?;

        self.user_subscribe_to_channel(channel).await?;

        Ok(Response::new(UserSubscribeResponse {}))
    }

    async fn user_unsubscribe_from_status_updates(
        &self,
        request: Request<UserUnsubscribeRequest>,
    ) -> Result<Response<UserUnsubscribeResponse>, Status> {
        let channel = request
            .into_inner()
            .channel_bundle
            .ok_or_else(|| Status::invalid_argument("missing channel bundle"))?;

        self.user_unsubscribe_from_channel(channel).await?;

        Ok(Response::new(UserUnsubscribeResponse {}))
    }

    /// Publish a new simple text status update in a status update or group
    /// By channel/group creator, or by channel subscriber reply or group member
    async fn user_new_post(
        &self,
        request: Request<UserNewPostRequest>,
    ) -> Result<Response<UserNewPostResponse>, Status> {
        let client = SimpleClient::from_registry()
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?;
        let req = request.into_inner();
        let channel_id = req
            .channel_id
            .ok_or_else(|| Status::invalid_argument("missing channel id"))?;

        match client
            .call(PublishNewChannelMessage {
                channel_id,
                reply_to: req.reply_to,
                text: req.text,
            })
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?
        {
            Ok(post_id) => Ok(Response::new(UserNewPostResponse { post_id })),
            Err(e) => Err(Status::internal(format!("Internal error: {:?}", e))),
        }
    }

    /// Create a new group on behalf of this client's user
    async fn user_create_group(
        &self,
        request: Request<UserCreateGroupRequest>,
    ) -> Result<Response<UserCreateGroupResponse>, Status> {
        let client = SimpleClient::from_registry()
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?;

        match client
            .call(CreateNewChannel {
                name: request.into_inner().group_name,
                channel_type: ChannelType::Group,
                description: "My Upsetter Group".to_string(),
            })
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?
        {
            Ok(bundle) => Ok(Response::new(UserCreateGroupResponse {
                channel_bundle: Some(bundle),
            })),
            Err(e) => Err(Status::internal(format!("Internal error: {:?}", e))),
        }
    }

    async fn user_join_group(
        &self,
        request: Request<UserJoinGroupRequest>,
    ) -> Result<Response<UserJoinGroupResponse>, Status> {
        let channel = request
            .into_inner()
            .channel_bundle
            .ok_or_else(|| Status::invalid_argument("missing channel bundle"))?;
        self.user_subscribe_to_channel(channel).await?;
        Ok(Response::new(UserJoinGroupResponse {}))
    }

    async fn user_leave_group(
        &self,
        request: Request<UserLeaveGroupRequest>,
    ) -> Result<Response<UserLeaveGroupResponse>, Status> {
        let channel = request
            .into_inner()
            .channel_bundle
            .ok_or_else(|| Status::invalid_argument("missing channel bundle"))?;
        self.user_unsubscribe_from_channel(channel).await?;
        Ok(Response::new(UserLeaveGroupResponse {}))
    }

    async fn user_create_paid_item(
        &self,
        request: Request<UserCreatePaidItemRequest>,
    ) -> Result<Response<UserCreatePaidItemResponse>, Status> {
        let client = SimpleClient::from_registry()
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?;

        match client
            .call(CreatePaidItem(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?
        {
            Ok(item_id) => Ok(Response::new(UserCreatePaidItemResponse { item_id })),
            Err(e) => Err(Status::internal(format!("Internal error: {:?}", e))),
        }
    }

    // a request from client's user to buy a paid content item from another client
    async fn user_buy_paid_item(
        &self,
        request: Request<UserBuyPaidItemRequest>,
    ) -> Result<Response<UserBuyPaidItemResponse>, Status> {
        let client = SimpleClient::from_registry()
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?;

        let msg = request.into_inner();
        let seller_id = msg
            .seller_client_id
            .ok_or_else(|| Status::invalid_argument("missing seller id"))?;
        match client
            .call(BuyItem {
                seller_id,
                item_id: msg.item_id,
                price: msg.price,
            })
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?
        {
            Ok(_item_id) => Ok(Response::new(UserBuyPaidItemResponse {})),
            Err(e) => Err(Status::internal(format!("Internal error: {:?}", e))),
        }
    }

    async fn user_list_paid_content_items(
        &self,
        request: Request<UserListPaidContentItemsRequest>,
    ) -> Result<Response<UserListPaidContentItemsResponse>, Status> {
        let client = SimpleClient::from_registry()
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?;

        let msg = request.into_inner();
        let seller_id = msg
            .seller_client_id
            .ok_or_else(|| Status::invalid_argument("missing seller id"))?;

        match client
            .call(ListItems { seller_id })
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?
        {
            Ok(_item_id) => Ok(Response::new(UserListPaidContentItemsResponse {})),
            Err(e) => Err(Status::internal(format!("Internal error: {:?}", e))),
        }
    }

    /// Set the blockchain service for this client
    async fn set_blockchain_service(
        &self,
        request: Request<SetBlockchainServiceRequest>,
    ) -> Result<Response<()>, Status> {
        let client = SimpleClient::from_registry()
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?;

        let info = request
            .into_inner()
            .dialup_info
            .ok_or_else(|| Status::invalid_argument("missing dialup info"))?;

        match client
            .call(SetBlockchainService { info })
            .await
            .map_err(|e| Status::internal(format!("{:?}", e)))?
        {
            Ok(()) => Ok(Response::new(())),
            Err(e) => Err(Status::internal(format!("Internal error: {:?}", e))),
        }
    }
}
