#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBlockchainServiceRequest {
    #[prost(message, optional, tag = "1")]
    pub dialup_info: ::core::option::Option<super::super::snp::core_types::DialupInfo>,
}
/////////// status updates ////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSubscribeRequest {
    /// Specify channel bundle for subscription (in product it should be stored via kad and queryable by channel_id
    #[prost(message, optional, tag = "1")]
    pub channel_bundle: ::core::option::Option<super::super::snp::core_types::ChannelBundle>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSubscribeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserUnsubscribeRequest {
    /// Specify channel bundle to unsubscribe from (in product it should be stored via kad and queryable by channel_id
    #[prost(message, optional, tag = "1")]
    pub channel_bundle: ::core::option::Option<super::super::snp::core_types::ChannelBundle>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserUnsubscribeResponse {}
/// Publish a new simple text status update, a replay to a status update, a group message or a reply to a group message
/// Implementation in SimpleClient should do the correct thing based on whether user is a the channel's creator or not.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserNewPostRequest {
    #[prost(message, optional, tag = "1")]
    pub channel_id: ::core::option::Option<super::super::snp::core_types::EntityId>,
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    /// id of content item this post is a reply to
    #[prost(uint64, tag = "3")]
    pub reply_to: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserNewPostResponse {
    /// the unique generated post id. useful so integration tests can send a reply for the post
    #[prost(uint64, tag = "1")]
    pub post_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCreateStatusUpdateChannelRequest {
    #[prost(string, tag = "1")]
    pub channel_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCreateStatusUpdateChannelResponse {
    #[prost(message, optional, tag = "1")]
    pub channel_bundle: ::core::option::Option<super::super::snp::core_types::ChannelBundle>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserAddOtherClientBundleResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSetProviderRequest {
    #[prost(message, optional, tag = "1")]
    pub dialup_info: ::core::option::Option<super::super::snp::core_types::DialupInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSetProviderResponse {
    /// the provider signed client bundle that was also sent over the p2p network for providers close to client
    #[prost(message, optional, tag = "1")]
    pub client_bundle:
        ::core::option::Option<super::super::snp::core_types::ProviderSignedClientIdentityBundle>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSendTextMessageRequest {
    #[prost(message, optional, tag = "1")]
    pub other_client_id: ::core::option::Option<super::super::snp::core_types::EntityId>,
    #[prost(string, tag = "2")]
    pub user_text: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub reply_to: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSendTextMessageResponse {
    /// the unique generated post id. useful so integration tests can send a reply for the message
    #[prost(uint64, tag = "1")]
    pub message_id: u64,
}
///// Groups

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCreateGroupRequest {
    #[prost(string, tag = "1")]
    pub group_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserJoinGroupResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCreateGroupResponse {
    #[prost(message, optional, tag = "1")]
    pub channel_bundle: ::core::option::Option<super::super::snp::core_types::ChannelBundle>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserJoinGroupRequest {
    #[prost(message, optional, tag = "1")]
    pub channel_bundle: ::core::option::Option<super::super::snp::core_types::ChannelBundle>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserLeaveGroupRequest {
    #[prost(message, optional, tag = "1")]
    pub channel_bundle: ::core::option::Option<super::super::snp::core_types::ChannelBundle>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserLeaveGroupResponse {}
//// Paid content items

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCreatePaidItemRequest {
    #[prost(uint64, tag = "1")]
    pub price: u64,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// string content only for now
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCreatePaidItemResponse {
    #[prost(uint64, tag = "1")]
    pub item_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserBuyPaidItemRequest {
    #[prost(message, optional, tag = "1")]
    pub seller_client_id: ::core::option::Option<super::super::snp::core_types::EntityId>,
    #[prost(uint64, tag = "2")]
    pub item_id: u64,
    #[prost(uint64, tag = "3")]
    pub price: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserBuyPaidItemResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListPaidContentItemsRequest {
    #[prost(message, optional, tag = "1")]
    pub seller_client_id: ::core::option::Option<super::super::snp::core_types::EntityId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListPaidContentItemsResponse {}
#[doc = r" Generated client implementations."]
pub mod simple_client_user_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A simple Upsetter client grpc api simulating a real user interacting with a SNP client"]
    #[doc = " Useful for automated integration testing which involves clients so lots of boilerplate code can be shared between"]
    #[doc = " test scenarios."]
    #[doc = " API usage pattern for instant messaging: 1. set a provider. 2. set other client bundle. 3. send message(s) to other client."]
    #[doc = " Note the User prefix to types used here - this is used to avoid confusion with server api similar types"]
    #[derive(Debug, Clone)]
    pub struct SimpleClientUserServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SimpleClientUserServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SimpleClientUserServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SimpleClientUserServiceClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            SimpleClientUserServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Set a provider for this client"]
        pub async fn user_set_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::UserSetProviderRequest>,
        ) -> Result<tonic::Response<super::UserSetProviderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/upsetter.simple_client.SimpleClientUserService/UserSetProvider",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Set other client bundle (so we can chat with him)"]
        pub async fn user_add_other_client_bundle(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::snp::core_types::ProviderSignedClientIdentityBundle,
            >,
        ) -> Result<tonic::Response<super::UserAddOtherClientBundleResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/upsetter.simple_client.SimpleClientUserService/UserAddOtherClientBundle",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Send a 1:1 text message to another other client on behalf of user"]
        pub async fn user_send_text_message(
            &mut self,
            request: impl tonic::IntoRequest<super::UserSendTextMessageRequest>,
        ) -> Result<tonic::Response<super::UserSendTextMessageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/upsetter.simple_client.SimpleClientUserService/UserSendTextMessage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a new status update channel and return its id and bundle so we can share it with other clients so"]
        #[doc = " they may subscribe"]
        pub async fn user_create_status_update_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::UserCreateStatusUpdateChannelRequest>,
        ) -> Result<tonic::Response<super::UserCreateStatusUpdateChannelResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/upsetter.simple_client.SimpleClientUserService/UserCreateStatusUpdateChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Subscribe client on behalf of user to a status updates channel"]
        pub async fn user_subscribe_to_status_updates(
            &mut self,
            request: impl tonic::IntoRequest<super::UserSubscribeRequest>,
        ) -> Result<tonic::Response<super::UserSubscribeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/upsetter.simple_client.SimpleClientUserService/UserSubscribeToStatusUpdates",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Unsubscribe the client on behalf of user from a channel client is subscribed to"]
        pub async fn user_unsubscribe_from_status_updates(
            &mut self,
            request: impl tonic::IntoRequest<super::UserUnsubscribeRequest>,
        ) -> Result<tonic::Response<super::UserUnsubscribeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/upsetter.simple_client.SimpleClientUserService/UserUnsubscribeFromStatusUpdates",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Publish a new status update, a reply to a status update, a new group message or reply to a group message"]
        pub async fn user_new_post(
            &mut self,
            request: impl tonic::IntoRequest<super::UserNewPostRequest>,
        ) -> Result<tonic::Response<super::UserNewPostResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/upsetter.simple_client.SimpleClientUserService/UserNewPost",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a new group channel and return its id and bundle so we can share it with other clients so"]
        #[doc = " they may subscribe"]
        pub async fn user_create_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UserCreateGroupRequest>,
        ) -> Result<tonic::Response<super::UserCreateGroupResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/upsetter.simple_client.SimpleClientUserService/UserCreateGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A request from a user to group creator to join a groups - for now it is always accepted"]
        pub async fn user_join_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UserJoinGroupRequest>,
        ) -> Result<tonic::Response<super::UserJoinGroupResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/upsetter.simple_client.SimpleClientUserService/UserJoinGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " User asks creator to leave group"]
        pub async fn user_leave_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UserLeaveGroupRequest>,
        ) -> Result<tonic::Response<super::UserLeaveGroupResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/upsetter.simple_client.SimpleClientUserService/UserLeaveGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a new paid content item"]
        pub async fn user_create_paid_item(
            &mut self,
            request: impl tonic::IntoRequest<super::UserCreatePaidItemRequest>,
        ) -> Result<tonic::Response<super::UserCreatePaidItemResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/upsetter.simple_client.SimpleClientUserService/UserCreatePaidItem",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Buy a paid content item published by another client on behalf of user"]
        pub async fn user_buy_paid_item(
            &mut self,
            request: impl tonic::IntoRequest<super::UserBuyPaidItemRequest>,
        ) -> Result<tonic::Response<super::UserBuyPaidItemResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/upsetter.simple_client.SimpleClientUserService/UserBuyPaidItem",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn user_list_paid_content_items(
            &mut self,
            request: impl tonic::IntoRequest<super::UserListPaidContentItemsRequest>,
        ) -> Result<tonic::Response<super::UserListPaidContentItemsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/upsetter.simple_client.SimpleClientUserService/UserListPaidContentItems",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Name Service"]
        pub async fn set_blockchain_service(
            &mut self,
            request: impl tonic::IntoRequest<super::SetBlockchainServiceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/upsetter.simple_client.SimpleClientUserService/SetBlockchainService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod simple_client_user_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SimpleClientUserServiceServer."]
    #[async_trait]
    pub trait SimpleClientUserService: Send + Sync + 'static {
        #[doc = " Set a provider for this client"]
        async fn user_set_provider(
            &self,
            request: tonic::Request<super::UserSetProviderRequest>,
        ) -> Result<tonic::Response<super::UserSetProviderResponse>, tonic::Status>;
        #[doc = " Set other client bundle (so we can chat with him)"]
        async fn user_add_other_client_bundle(
            &self,
            request: tonic::Request<
                super::super::super::snp::core_types::ProviderSignedClientIdentityBundle,
            >,
        ) -> Result<tonic::Response<super::UserAddOtherClientBundleResponse>, tonic::Status>;
        #[doc = " Send a 1:1 text message to another other client on behalf of user"]
        async fn user_send_text_message(
            &self,
            request: tonic::Request<super::UserSendTextMessageRequest>,
        ) -> Result<tonic::Response<super::UserSendTextMessageResponse>, tonic::Status>;
        #[doc = " Create a new status update channel and return its id and bundle so we can share it with other clients so"]
        #[doc = " they may subscribe"]
        async fn user_create_status_update_channel(
            &self,
            request: tonic::Request<super::UserCreateStatusUpdateChannelRequest>,
        ) -> Result<tonic::Response<super::UserCreateStatusUpdateChannelResponse>, tonic::Status>;
        #[doc = " Subscribe client on behalf of user to a status updates channel"]
        async fn user_subscribe_to_status_updates(
            &self,
            request: tonic::Request<super::UserSubscribeRequest>,
        ) -> Result<tonic::Response<super::UserSubscribeResponse>, tonic::Status>;
        #[doc = " Unsubscribe the client on behalf of user from a channel client is subscribed to"]
        async fn user_unsubscribe_from_status_updates(
            &self,
            request: tonic::Request<super::UserUnsubscribeRequest>,
        ) -> Result<tonic::Response<super::UserUnsubscribeResponse>, tonic::Status>;
        #[doc = " Publish a new status update, a reply to a status update, a new group message or reply to a group message"]
        async fn user_new_post(
            &self,
            request: tonic::Request<super::UserNewPostRequest>,
        ) -> Result<tonic::Response<super::UserNewPostResponse>, tonic::Status>;
        #[doc = " Create a new group channel and return its id and bundle so we can share it with other clients so"]
        #[doc = " they may subscribe"]
        async fn user_create_group(
            &self,
            request: tonic::Request<super::UserCreateGroupRequest>,
        ) -> Result<tonic::Response<super::UserCreateGroupResponse>, tonic::Status>;
        #[doc = " A request from a user to group creator to join a groups - for now it is always accepted"]
        async fn user_join_group(
            &self,
            request: tonic::Request<super::UserJoinGroupRequest>,
        ) -> Result<tonic::Response<super::UserJoinGroupResponse>, tonic::Status>;
        #[doc = " User asks creator to leave group"]
        async fn user_leave_group(
            &self,
            request: tonic::Request<super::UserLeaveGroupRequest>,
        ) -> Result<tonic::Response<super::UserLeaveGroupResponse>, tonic::Status>;
        #[doc = " Create a new paid content item"]
        async fn user_create_paid_item(
            &self,
            request: tonic::Request<super::UserCreatePaidItemRequest>,
        ) -> Result<tonic::Response<super::UserCreatePaidItemResponse>, tonic::Status>;
        #[doc = " Buy a paid content item published by another client on behalf of user"]
        async fn user_buy_paid_item(
            &self,
            request: tonic::Request<super::UserBuyPaidItemRequest>,
        ) -> Result<tonic::Response<super::UserBuyPaidItemResponse>, tonic::Status>;
        async fn user_list_paid_content_items(
            &self,
            request: tonic::Request<super::UserListPaidContentItemsRequest>,
        ) -> Result<tonic::Response<super::UserListPaidContentItemsResponse>, tonic::Status>;
        #[doc = " Name Service"]
        async fn set_blockchain_service(
            &self,
            request: tonic::Request<super::SetBlockchainServiceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[doc = " A simple Upsetter client grpc api simulating a real user interacting with a SNP client"]
    #[doc = " Useful for automated integration testing which involves clients so lots of boilerplate code can be shared between"]
    #[doc = " test scenarios."]
    #[doc = " API usage pattern for instant messaging: 1. set a provider. 2. set other client bundle. 3. send message(s) to other client."]
    #[doc = " Note the User prefix to types used here - this is used to avoid confusion with server api similar types"]
    #[derive(Debug)]
    pub struct SimpleClientUserServiceServer<T: SimpleClientUserService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SimpleClientUserService> SimpleClientUserServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        #[doc = r" Enable decompressing requests with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.accept_compression_encodings.enable_gzip();
            self
        }
        #[doc = r" Compress responses with `gzip`, if the client supports it."]
        pub fn send_gzip(mut self) -> Self {
            self.send_compression_encodings.enable_gzip();
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SimpleClientUserServiceServer<T>
    where
        T: SimpleClientUserService,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req . uri () . path () { "/upsetter.simple_client.SimpleClientUserService/UserSetProvider" => { # [allow (non_camel_case_types)] struct UserSetProviderSvc < T : SimpleClientUserService > (pub Arc < T >) ; impl < T : SimpleClientUserService > tonic :: server :: UnaryService < super :: UserSetProviderRequest > for UserSetProviderSvc < T > { type Response = super :: UserSetProviderResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: UserSetProviderRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . user_set_provider (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = UserSetProviderSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/upsetter.simple_client.SimpleClientUserService/UserAddOtherClientBundle" => { # [allow (non_camel_case_types)] struct UserAddOtherClientBundleSvc < T : SimpleClientUserService > (pub Arc < T >) ; impl < T : SimpleClientUserService > tonic :: server :: UnaryService < super :: super :: super :: snp :: core_types :: ProviderSignedClientIdentityBundle > for UserAddOtherClientBundleSvc < T > { type Response = super :: UserAddOtherClientBundleResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: snp :: core_types :: ProviderSignedClientIdentityBundle >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . user_add_other_client_bundle (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = UserAddOtherClientBundleSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/upsetter.simple_client.SimpleClientUserService/UserSendTextMessage" => { # [allow (non_camel_case_types)] struct UserSendTextMessageSvc < T : SimpleClientUserService > (pub Arc < T >) ; impl < T : SimpleClientUserService > tonic :: server :: UnaryService < super :: UserSendTextMessageRequest > for UserSendTextMessageSvc < T > { type Response = super :: UserSendTextMessageResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: UserSendTextMessageRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . user_send_text_message (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = UserSendTextMessageSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/upsetter.simple_client.SimpleClientUserService/UserCreateStatusUpdateChannel" => { # [allow (non_camel_case_types)] struct UserCreateStatusUpdateChannelSvc < T : SimpleClientUserService > (pub Arc < T >) ; impl < T : SimpleClientUserService > tonic :: server :: UnaryService < super :: UserCreateStatusUpdateChannelRequest > for UserCreateStatusUpdateChannelSvc < T > { type Response = super :: UserCreateStatusUpdateChannelResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: UserCreateStatusUpdateChannelRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . user_create_status_update_channel (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = UserCreateStatusUpdateChannelSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/upsetter.simple_client.SimpleClientUserService/UserSubscribeToStatusUpdates" => { # [allow (non_camel_case_types)] struct UserSubscribeToStatusUpdatesSvc < T : SimpleClientUserService > (pub Arc < T >) ; impl < T : SimpleClientUserService > tonic :: server :: UnaryService < super :: UserSubscribeRequest > for UserSubscribeToStatusUpdatesSvc < T > { type Response = super :: UserSubscribeResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: UserSubscribeRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . user_subscribe_to_status_updates (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = UserSubscribeToStatusUpdatesSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/upsetter.simple_client.SimpleClientUserService/UserUnsubscribeFromStatusUpdates" => { # [allow (non_camel_case_types)] struct UserUnsubscribeFromStatusUpdatesSvc < T : SimpleClientUserService > (pub Arc < T >) ; impl < T : SimpleClientUserService > tonic :: server :: UnaryService < super :: UserUnsubscribeRequest > for UserUnsubscribeFromStatusUpdatesSvc < T > { type Response = super :: UserUnsubscribeResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: UserUnsubscribeRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . user_unsubscribe_from_status_updates (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = UserUnsubscribeFromStatusUpdatesSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/upsetter.simple_client.SimpleClientUserService/UserNewPost" => { # [allow (non_camel_case_types)] struct UserNewPostSvc < T : SimpleClientUserService > (pub Arc < T >) ; impl < T : SimpleClientUserService > tonic :: server :: UnaryService < super :: UserNewPostRequest > for UserNewPostSvc < T > { type Response = super :: UserNewPostResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: UserNewPostRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . user_new_post (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = UserNewPostSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/upsetter.simple_client.SimpleClientUserService/UserCreateGroup" => { # [allow (non_camel_case_types)] struct UserCreateGroupSvc < T : SimpleClientUserService > (pub Arc < T >) ; impl < T : SimpleClientUserService > tonic :: server :: UnaryService < super :: UserCreateGroupRequest > for UserCreateGroupSvc < T > { type Response = super :: UserCreateGroupResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: UserCreateGroupRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . user_create_group (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = UserCreateGroupSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/upsetter.simple_client.SimpleClientUserService/UserJoinGroup" => { # [allow (non_camel_case_types)] struct UserJoinGroupSvc < T : SimpleClientUserService > (pub Arc < T >) ; impl < T : SimpleClientUserService > tonic :: server :: UnaryService < super :: UserJoinGroupRequest > for UserJoinGroupSvc < T > { type Response = super :: UserJoinGroupResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: UserJoinGroupRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . user_join_group (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = UserJoinGroupSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/upsetter.simple_client.SimpleClientUserService/UserLeaveGroup" => { # [allow (non_camel_case_types)] struct UserLeaveGroupSvc < T : SimpleClientUserService > (pub Arc < T >) ; impl < T : SimpleClientUserService > tonic :: server :: UnaryService < super :: UserLeaveGroupRequest > for UserLeaveGroupSvc < T > { type Response = super :: UserLeaveGroupResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: UserLeaveGroupRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . user_leave_group (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = UserLeaveGroupSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/upsetter.simple_client.SimpleClientUserService/UserCreatePaidItem" => { # [allow (non_camel_case_types)] struct UserCreatePaidItemSvc < T : SimpleClientUserService > (pub Arc < T >) ; impl < T : SimpleClientUserService > tonic :: server :: UnaryService < super :: UserCreatePaidItemRequest > for UserCreatePaidItemSvc < T > { type Response = super :: UserCreatePaidItemResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: UserCreatePaidItemRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . user_create_paid_item (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = UserCreatePaidItemSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/upsetter.simple_client.SimpleClientUserService/UserBuyPaidItem" => { # [allow (non_camel_case_types)] struct UserBuyPaidItemSvc < T : SimpleClientUserService > (pub Arc < T >) ; impl < T : SimpleClientUserService > tonic :: server :: UnaryService < super :: UserBuyPaidItemRequest > for UserBuyPaidItemSvc < T > { type Response = super :: UserBuyPaidItemResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: UserBuyPaidItemRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . user_buy_paid_item (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = UserBuyPaidItemSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/upsetter.simple_client.SimpleClientUserService/UserListPaidContentItems" => { # [allow (non_camel_case_types)] struct UserListPaidContentItemsSvc < T : SimpleClientUserService > (pub Arc < T >) ; impl < T : SimpleClientUserService > tonic :: server :: UnaryService < super :: UserListPaidContentItemsRequest > for UserListPaidContentItemsSvc < T > { type Response = super :: UserListPaidContentItemsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: UserListPaidContentItemsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . user_list_paid_content_items (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = UserListPaidContentItemsSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/upsetter.simple_client.SimpleClientUserService/SetBlockchainService" => { # [allow (non_camel_case_types)] struct SetBlockchainServiceSvc < T : SimpleClientUserService > (pub Arc < T >) ; impl < T : SimpleClientUserService > tonic :: server :: UnaryService < super :: SetBlockchainServiceRequest > for SetBlockchainServiceSvc < T > { type Response = () ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: SetBlockchainServiceRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . set_blockchain_service (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = SetBlockchainServiceSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } _ => Box :: pin (async move { Ok (http :: Response :: builder () . status (200) . header ("grpc-status" , "12") . header ("content-type" , "application/grpc") . body (empty_body ()) . unwrap ()) }) , }
        }
    }
    impl<T: SimpleClientUserService> Clone for SimpleClientUserServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: SimpleClientUserService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SimpleClientUserService> tonic::transport::NamedService
        for SimpleClientUserServiceServer<T>
    {
        const NAME: &'static str = "upsetter.simple_client.SimpleClientUserService";
    }
}
