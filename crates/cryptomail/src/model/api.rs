#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNewThreadIdRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNewThreadIdResponse {
    #[prost(uint64, tag = "1")]
    pub thread_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAccountRequest {
    /// to avoid replay attacks
    #[prost(uint64, tag = "1")]
    pub time_stamp: u64,
    #[prost(message, optional, tag = "2")]
    pub public_key: ::core::option::Option<super::types::PublicKey>,
    #[prost(message, optional, tag = "3")]
    pub settings: ::core::option::Option<super::types::Settings>,
    /// signed publishable account info including payment settings
    #[prost(message, optional, tag = "4")]
    pub public_account_info: ::core::option::Option<super::types::PublicAccountInfo>,
    /// signature on all other data with private key matching provided public
    #[prost(bytes = "vec", tag = "5")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAccountResponse {
    #[prost(enumeration = "CreateAccountResult", tag = "1")]
    pub result: i32,
    #[prost(message, optional, tag = "2")]
    pub account: ::core::option::Option<super::types::Account>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetThreadBoxesRequest {
    /// to avoid replay attacks from mim who can read https traffic
    #[prost(uint64, tag = "1")]
    pub time_stamp: u64,
    /// caller public key
    #[prost(message, optional, tag = "2")]
    pub public_key: ::core::option::Option<super::types::PublicKey>,
    /// bitmask of boxes to get
    #[prost(uint32, tag = "3")]
    pub thread_boxes: u32,
    #[prost(bytes = "vec", tag = "4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetThreadBoxesResponse {
    /// updated caller account (info+reputation from the server
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<super::types::Account>,
    #[prost(message, repeated, tag = "2")]
    pub threads_boxes: ::prost::alloc::vec::Vec<super::types::ThreadBox>,
    /// all messages in each thread in each thread-box
    #[prost(message, repeated, tag = "3")]
    pub messages: ::prost::alloc::vec::Vec<super::types::Message>,
    /// info for each sender of a message
    #[prost(message, repeated, tag = "4")]
    pub accounts: ::prost::alloc::vec::Vec<super::types::Account>,
    /// all threads that are included in one of the response's boxes
    #[prost(message, repeated, tag = "5")]
    pub threads: ::prost::alloc::vec::Vec<super::types::Thread>,
}
/// a request to start a new thread by sending a first message in the thread
/// thread id must be globally unique and set by caller. In case of conflict it will be rejected.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewThreadRequest {
    /// to avoid replay attacks
    #[prost(uint64, tag = "1")]
    pub time_stamp: u64,
    #[prost(message, optional, tag = "2")]
    pub public_key: ::core::option::Option<super::types::PublicKey>,
    /// binary MessageUserData
    #[prost(bytes = "vec", tag = "3")]
    pub message_user_data: ::prost::alloc::vec::Vec<u8>,
    /// signature on binary message_user_data
    #[prost(bytes = "vec", tag = "4")]
    pub message_user_data_signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub message_id: ::core::option::Option<super::types::MessageId>,
    /// on all other fields
    #[prost(bytes = "vec", tag = "6")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewThreadResponse {
    #[prost(enumeration = "NewThreadResult", tag = "1")]
    pub result: i32,
    #[prost(message, optional, tag = "2")]
    pub message_id: ::core::option::Option<super::types::MessageId>,
}
/// todo: consider getting from user transaction id for paid open!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenMessageRequest {
    /// to avoid replay attacks
    #[prost(uint64, tag = "1")]
    pub time_stamp: u64,
    #[prost(message, optional, tag = "2")]
    pub public_key: ::core::option::Option<super::types::PublicKey>,
    #[prost(message, optional, tag = "3")]
    pub message_id: ::core::option::Option<super::types::MessageId>,
    #[prost(bytes = "vec", tag = "4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenMessageResponse {}
/// todo: consider getting user transaction id for paid reply!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyRequest {
    /// to avoid replay attacks
    #[prost(uint64, tag = "1")]
    pub time_stamp: u64,
    #[prost(message, optional, tag = "2")]
    pub public_key: ::core::option::Option<super::types::PublicKey>,
    /// binary signed MessageUserData
    #[prost(bytes = "vec", tag = "3")]
    pub message_user_data: ::prost::alloc::vec::Vec<u8>,
    /// signature on binary message_user_data
    #[prost(bytes = "vec", tag = "4")]
    pub message_user_data_signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub message_id: ::core::option::Option<super::types::MessageId>,
    /// on all other fields
    #[prost(bytes = "vec", tag = "6")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyResponse {
    #[prost(message, optional, tag = "1")]
    pub message_id: ::core::option::Option<super::types::MessageId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveThreadRequest {
    /// to avoid replay attacks
    #[prost(uint64, tag = "1")]
    pub time_stamp: u64,
    #[prost(message, optional, tag = "2")]
    pub public_key: ::core::option::Option<super::types::PublicKey>,
    #[prost(bytes = "vec", tag = "3")]
    pub thread_id: ::prost::alloc::vec::Vec<u8>,
    /// user signature on thread id
    #[prost(bytes = "vec", tag = "4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveThreadResponse {
    #[prost(message, optional, tag = "1")]
    pub thread_id: ::core::option::Option<super::types::ThreadId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteThreadRequest {
    /// to avoid replay attacks
    #[prost(uint64, tag = "1")]
    pub time_stamp: u64,
    #[prost(message, optional, tag = "2")]
    pub public_key: ::core::option::Option<super::types::PublicKey>,
    #[prost(bytes = "vec", tag = "3")]
    pub thread_id: ::prost::alloc::vec::Vec<u8>,
    /// user signature on thread id
    #[prost(bytes = "vec", tag = "4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteThreadResponse {
    #[prost(message, optional, tag = "1")]
    pub thread_id: ::core::option::Option<super::types::ThreadId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSettingsRequest {
    /// to avoid replay attacks
    #[prost(uint64, tag = "1")]
    pub time_stamp: u64,
    /// account id
    #[prost(message, optional, tag = "2")]
    pub public_key: ::core::option::Option<super::types::PublicKey>,
    #[prost(message, optional, tag = "3")]
    pub settings: ::core::option::Option<super::types::Settings>,
    /// signed publishable account info - including new pre-key and payment settings
    #[prost(message, optional, tag = "4")]
    pub public_account_info: ::core::option::Option<super::types::PublicAccountInfo>,
    #[prost(bytes = "vec", tag = "5")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSettingsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAccountRequest {
    /// to avoid replay attacks
    #[prost(uint64, tag = "1")]
    pub time_stamp: u64,
    #[prost(message, optional, tag = "2")]
    pub public_key: ::core::option::Option<super::types::PublicKey>,
    #[prost(bytes = "vec", tag = "3")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAccountResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountRequest {
    #[prost(oneof = "get_account_request::Data", tags = "1, 2")]
    pub data: ::core::option::Option<get_account_request::Data>,
}
/// Nested message and enum types in `GetAccountRequest`.
pub mod get_account_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "1")]
        PublicKey(super::super::types::PublicKey),
        #[prost(string, tag = "2")]
        Name(::prost::alloc::string::String),
    }
}
/// Public account info
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountResponse {
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<super::types::Account>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPublicAccountsRequest {
    /// get from a name...
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    /// max number of results to return
    #[prost(uint32, tag = "2")]
    pub max_results: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPublicAccountsResponse {
    /// total accounts in the system
    #[prost(uint32, tag = "1")]
    pub total: u32,
    /// starting at offset and up to max_results
    #[prost(message, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<super::types::Account>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMessageDepositDataRequest {
    #[prost(message, optional, tag = "1")]
    pub message_id: ::core::option::Option<super::types::MessageId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMessageDepositDataResponse {
    #[prost(message, optional, tag = "1")]
    pub deposit_confirmation: ::core::option::Option<super::types::DepositConfirmation>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCoinPriceRequest {
    /// e.g ETH
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
    /// e.g. USD, EUR
    #[prost(string, repeated, tag = "2")]
    pub currencies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FiatCoinPrice {
    #[prost(string, tag = "1")]
    pub currency: ::prost::alloc::string::String,
    #[prost(float, tag = "2")]
    pub price: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCoinPriceResponse {
    #[prost(message, repeated, tag = "1")]
    pub prices: ::prost::alloc::vec::Vec<FiatCoinPrice>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CreateAccountResult {
    /// thread successfully created
    Created = 0,
    /// account with this pub key already exists
    Exists = 1,
    /// there's already an account with this name
    NameTaken = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NewThreadResult {
    /// thread successfully created
    Created = 0,
    /// id is taken - user should select a new one and call again
    InvalidThreadId = 1,
    /// invalid user signature on data
    InvalidSig = 2,
    /// missing user provided data
    MissingData = 3,
    /// can't find transaction in mem-pool or in a block
    InvalidTx = 4,
    /// message too old or too in the future
    InvalidTimeStamp = 5,
    /// sender doesn't have an account
    InvalidSenderAccount = 6,
    /// receiver doesn't have an account
    InvalidReceiverAccount = 7,
}
#[doc = r" Generated client implementations."]
pub mod cryptomail_api_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct CryptomailApiServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CryptomailApiServiceClient<tonic::transport::Channel> {
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
    impl<T> CryptomailApiServiceClient<T>
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
        ) -> CryptomailApiServiceClient<InterceptedService<T, F>>
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
            CryptomailApiServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Create a new user account"]
        pub async fn create_account(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAccountRequest>,
        ) -> Result<tonic::Response<super::CreateAccountResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/api.CryptomailApiService/CreateAccount");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update existing account settings"]
        pub async fn update_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSettingsRequest>,
        ) -> Result<tonic::Response<super::UpdateSettingsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/api.CryptomailApiService/UpdateSettings");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete account and all account data"]
        pub async fn delete_account(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAccountRequest>,
        ) -> Result<tonic::Response<super::DeleteAccountResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/api.CryptomailApiService/DeleteAccount");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets most recent Account data and one or more account threads-box. e.g. Inbox, Archive, Sent."]
        #[doc = " Called by users to view changes in their account (e.g. reputation) in receive updated thread-boxes"]
        #[doc = " For this version we return messages from the server for the each thread in each thread-box. In future"]
        #[doc = " release clients will query the server for a batch of message ids and will have a local store of messages by id"]
        #[doc = " messages are immutable and easily cacheable in clients e.g. local browser store"]
        pub async fn get_thread_boxes(
            &mut self,
            request: impl tonic::IntoRequest<super::GetThreadBoxesRequest>,
        ) -> Result<tonic::Response<super::GetThreadBoxesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/api.CryptomailApiService/GetThreadBoxes");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Open a message"]
        pub async fn open_message(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenMessageRequest>,
        ) -> Result<tonic::Response<super::OpenMessageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/api.CryptomailApiService/OpenMessage");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Reply to a message"]
        pub async fn reply(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplyRequest>,
        ) -> Result<tonic::Response<super::ReplyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.CryptomailApiService/Reply");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Move a thread from Inbox to Archive"]
        pub async fn archive_thread(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveThreadRequest>,
        ) -> Result<tonic::Response<super::ArchiveThreadResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/api.CryptomailApiService/ArchiveThread");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete a thread from all user's boxes"]
        pub async fn delete_thread(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteThreadRequest>,
        ) -> Result<tonic::Response<super::DeleteThreadResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/api.CryptomailApiService/DeleteThread");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Start a new thread with a new paid message"]
        pub async fn new_thread(
            &mut self,
            request: impl tonic::IntoRequest<super::NewThreadRequest>,
        ) -> Result<tonic::Response<super::NewThreadResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.CryptomailApiService/NewThread");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns basic public account info based on account name or address including pre-key which enables sending encrypted"]
        #[doc = " messages to this account"]
        pub async fn get_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountRequest>,
        ) -> Result<tonic::Response<super::GetAccountResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.CryptomailApiService/GetAccount");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns all publicly listed account for purpose of displaying a directory"]
        pub async fn get_public_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPublicAccountsRequest>,
        ) -> Result<tonic::Response<super::GetPublicAccountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/api.CryptomailApiService/GetPublicAccounts");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the current on-chain deposit information for a transaction in a message"]
        pub async fn get_message_deposit_data(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMessageDepositDataRequest>,
        ) -> Result<tonic::Response<super::GetMessageDepositDataResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.CryptomailApiService/GetMessageDepositData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns fiat price estimate for a coin such as ETH"]
        pub async fn get_coin_price(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCoinPriceRequest>,
        ) -> Result<tonic::Response<super::GetCoinPriceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/api.CryptomailApiService/GetCoinPrice");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod cryptomail_api_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CryptomailApiServiceServer."]
    #[async_trait]
    pub trait CryptomailApiService: Send + Sync + 'static {
        #[doc = " Create a new user account"]
        async fn create_account(
            &self,
            request: tonic::Request<super::CreateAccountRequest>,
        ) -> Result<tonic::Response<super::CreateAccountResponse>, tonic::Status>;
        #[doc = " Update existing account settings"]
        async fn update_settings(
            &self,
            request: tonic::Request<super::UpdateSettingsRequest>,
        ) -> Result<tonic::Response<super::UpdateSettingsResponse>, tonic::Status>;
        #[doc = " Delete account and all account data"]
        async fn delete_account(
            &self,
            request: tonic::Request<super::DeleteAccountRequest>,
        ) -> Result<tonic::Response<super::DeleteAccountResponse>, tonic::Status>;
        #[doc = " Gets most recent Account data and one or more account threads-box. e.g. Inbox, Archive, Sent."]
        #[doc = " Called by users to view changes in their account (e.g. reputation) in receive updated thread-boxes"]
        #[doc = " For this version we return messages from the server for the each thread in each thread-box. In future"]
        #[doc = " release clients will query the server for a batch of message ids and will have a local store of messages by id"]
        #[doc = " messages are immutable and easily cacheable in clients e.g. local browser store"]
        async fn get_thread_boxes(
            &self,
            request: tonic::Request<super::GetThreadBoxesRequest>,
        ) -> Result<tonic::Response<super::GetThreadBoxesResponse>, tonic::Status>;
        #[doc = " Open a message"]
        async fn open_message(
            &self,
            request: tonic::Request<super::OpenMessageRequest>,
        ) -> Result<tonic::Response<super::OpenMessageResponse>, tonic::Status>;
        #[doc = " Reply to a message"]
        async fn reply(
            &self,
            request: tonic::Request<super::ReplyRequest>,
        ) -> Result<tonic::Response<super::ReplyResponse>, tonic::Status>;
        #[doc = " Move a thread from Inbox to Archive"]
        async fn archive_thread(
            &self,
            request: tonic::Request<super::ArchiveThreadRequest>,
        ) -> Result<tonic::Response<super::ArchiveThreadResponse>, tonic::Status>;
        #[doc = " Delete a thread from all user's boxes"]
        async fn delete_thread(
            &self,
            request: tonic::Request<super::DeleteThreadRequest>,
        ) -> Result<tonic::Response<super::DeleteThreadResponse>, tonic::Status>;
        #[doc = " Start a new thread with a new paid message"]
        async fn new_thread(
            &self,
            request: tonic::Request<super::NewThreadRequest>,
        ) -> Result<tonic::Response<super::NewThreadResponse>, tonic::Status>;
        #[doc = " Returns basic public account info based on account name or address including pre-key which enables sending encrypted"]
        #[doc = " messages to this account"]
        async fn get_account(
            &self,
            request: tonic::Request<super::GetAccountRequest>,
        ) -> Result<tonic::Response<super::GetAccountResponse>, tonic::Status>;
        #[doc = " Returns all publicly listed account for purpose of displaying a directory"]
        async fn get_public_accounts(
            &self,
            request: tonic::Request<super::GetPublicAccountsRequest>,
        ) -> Result<tonic::Response<super::GetPublicAccountsResponse>, tonic::Status>;
        #[doc = " Returns the current on-chain deposit information for a transaction in a message"]
        async fn get_message_deposit_data(
            &self,
            request: tonic::Request<super::GetMessageDepositDataRequest>,
        ) -> Result<tonic::Response<super::GetMessageDepositDataResponse>, tonic::Status>;
        #[doc = " Returns fiat price estimate for a coin such as ETH"]
        async fn get_coin_price(
            &self,
            request: tonic::Request<super::GetCoinPriceRequest>,
        ) -> Result<tonic::Response<super::GetCoinPriceResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct CryptomailApiServiceServer<T: CryptomailApiService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CryptomailApiService> CryptomailApiServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CryptomailApiServiceServer<T>
    where
        T: CryptomailApiService,
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
            match req.uri().path() {
                "/api.CryptomailApiService/CreateAccount" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAccountSvc<T: CryptomailApiService>(pub Arc<T>);
                    impl<T: CryptomailApiService>
                        tonic::server::UnaryService<super::CreateAccountRequest>
                        for CreateAccountSvc<T>
                    {
                        type Response = super::CreateAccountResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateAccountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.CryptomailApiService/UpdateSettings" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSettingsSvc<T: CryptomailApiService>(pub Arc<T>);
                    impl<T: CryptomailApiService>
                        tonic::server::UnaryService<super::UpdateSettingsRequest>
                        for UpdateSettingsSvc<T>
                    {
                        type Response = super::UpdateSettingsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateSettingsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_settings(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateSettingsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.CryptomailApiService/DeleteAccount" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAccountSvc<T: CryptomailApiService>(pub Arc<T>);
                    impl<T: CryptomailApiService>
                        tonic::server::UnaryService<super::DeleteAccountRequest>
                        for DeleteAccountSvc<T>
                    {
                        type Response = super::DeleteAccountResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteAccountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.CryptomailApiService/GetThreadBoxes" => {
                    #[allow(non_camel_case_types)]
                    struct GetThreadBoxesSvc<T: CryptomailApiService>(pub Arc<T>);
                    impl<T: CryptomailApiService>
                        tonic::server::UnaryService<super::GetThreadBoxesRequest>
                        for GetThreadBoxesSvc<T>
                    {
                        type Response = super::GetThreadBoxesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetThreadBoxesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_thread_boxes(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetThreadBoxesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.CryptomailApiService/OpenMessage" => {
                    #[allow(non_camel_case_types)]
                    struct OpenMessageSvc<T: CryptomailApiService>(pub Arc<T>);
                    impl<T: CryptomailApiService>
                        tonic::server::UnaryService<super::OpenMessageRequest>
                        for OpenMessageSvc<T>
                    {
                        type Response = super::OpenMessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenMessageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).open_message(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OpenMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.CryptomailApiService/Reply" => {
                    #[allow(non_camel_case_types)]
                    struct ReplySvc<T: CryptomailApiService>(pub Arc<T>);
                    impl<T: CryptomailApiService> tonic::server::UnaryService<super::ReplyRequest> for ReplySvc<T> {
                        type Response = super::ReplyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReplyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).reply(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReplySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.CryptomailApiService/ArchiveThread" => {
                    #[allow(non_camel_case_types)]
                    struct ArchiveThreadSvc<T: CryptomailApiService>(pub Arc<T>);
                    impl<T: CryptomailApiService>
                        tonic::server::UnaryService<super::ArchiveThreadRequest>
                        for ArchiveThreadSvc<T>
                    {
                        type Response = super::ArchiveThreadResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ArchiveThreadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).archive_thread(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ArchiveThreadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.CryptomailApiService/DeleteThread" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteThreadSvc<T: CryptomailApiService>(pub Arc<T>);
                    impl<T: CryptomailApiService>
                        tonic::server::UnaryService<super::DeleteThreadRequest>
                        for DeleteThreadSvc<T>
                    {
                        type Response = super::DeleteThreadResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteThreadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_thread(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteThreadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.CryptomailApiService/NewThread" => {
                    #[allow(non_camel_case_types)]
                    struct NewThreadSvc<T: CryptomailApiService>(pub Arc<T>);
                    impl<T: CryptomailApiService>
                        tonic::server::UnaryService<super::NewThreadRequest> for NewThreadSvc<T>
                    {
                        type Response = super::NewThreadResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewThreadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_thread(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewThreadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.CryptomailApiService/GetAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountSvc<T: CryptomailApiService>(pub Arc<T>);
                    impl<T: CryptomailApiService>
                        tonic::server::UnaryService<super::GetAccountRequest> for GetAccountSvc<T>
                    {
                        type Response = super::GetAccountResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.CryptomailApiService/GetPublicAccounts" => {
                    #[allow(non_camel_case_types)]
                    struct GetPublicAccountsSvc<T: CryptomailApiService>(pub Arc<T>);
                    impl<T: CryptomailApiService>
                        tonic::server::UnaryService<super::GetPublicAccountsRequest>
                        for GetPublicAccountsSvc<T>
                    {
                        type Response = super::GetPublicAccountsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPublicAccountsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_public_accounts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPublicAccountsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.CryptomailApiService/GetMessageDepositData" => {
                    #[allow(non_camel_case_types)]
                    struct GetMessageDepositDataSvc<T: CryptomailApiService>(pub Arc<T>);
                    impl<T: CryptomailApiService>
                        tonic::server::UnaryService<super::GetMessageDepositDataRequest>
                        for GetMessageDepositDataSvc<T>
                    {
                        type Response = super::GetMessageDepositDataResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMessageDepositDataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_message_deposit_data(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMessageDepositDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.CryptomailApiService/GetCoinPrice" => {
                    #[allow(non_camel_case_types)]
                    struct GetCoinPriceSvc<T: CryptomailApiService>(pub Arc<T>);
                    impl<T: CryptomailApiService>
                        tonic::server::UnaryService<super::GetCoinPriceRequest>
                        for GetCoinPriceSvc<T>
                    {
                        type Response = super::GetCoinPriceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCoinPriceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_coin_price(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCoinPriceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: CryptomailApiService> Clone for CryptomailApiServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: CryptomailApiService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CryptomailApiService> tonic::transport::NamedService for CryptomailApiServiceServer<T> {
        const NAME: &'static str = "api.CryptomailApiService";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountsRequest {
    /// get from a name...
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    /// max number of results to return
    #[prost(uint32, tag = "2")]
    pub max_results: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountsResponse {
    /// total accounts in the system
    #[prost(uint32, tag = "1")]
    pub total: u32,
    /// starting at offset and up to max_results
    #[prost(message, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<super::types::Account>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountDataRequest {
    #[prost(oneof = "get_account_data_request::Data", tags = "1, 2")]
    pub data: ::core::option::Option<get_account_data_request::Data>,
}
/// Nested message and enum types in `GetAccountDataRequest`.
pub mod get_account_data_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "1")]
        PublicKey(super::super::types::PublicKey),
        #[prost(string, tag = "2")]
        Name(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountDataResponse {
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<super::types::Account>,
    #[prost(message, repeated, tag = "2")]
    pub thread_boxes: ::prost::alloc::vec::Vec<super::types::ThreadBox>,
}
#[doc = r" Generated client implementations."]
pub mod cryptomail_admin_api_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct CryptomailAdminApiServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CryptomailAdminApiServiceClient<tonic::transport::Channel> {
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
    impl<T> CryptomailAdminApiServiceClient<T>
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
        ) -> CryptomailAdminApiServiceClient<InterceptedService<T, F>>
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
            CryptomailAdminApiServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " list all accounts"]
        pub async fn get_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountsRequest>,
        ) -> Result<tonic::Response<super::GetAccountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/api.CryptomailAdminApiService/GetAccounts");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " get all server account data - including all thread-boxes, threads and messages"]
        pub async fn get_account_data(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountDataRequest>,
        ) -> Result<tonic::Response<super::GetAccountDataResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.CryptomailAdminApiService/GetAccountData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod cryptomail_admin_api_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CryptomailAdminApiServiceServer."]
    #[async_trait]
    pub trait CryptomailAdminApiService: Send + Sync + 'static {
        #[doc = " list all accounts"]
        async fn get_accounts(
            &self,
            request: tonic::Request<super::GetAccountsRequest>,
        ) -> Result<tonic::Response<super::GetAccountsResponse>, tonic::Status>;
        #[doc = " get all server account data - including all thread-boxes, threads and messages"]
        async fn get_account_data(
            &self,
            request: tonic::Request<super::GetAccountDataRequest>,
        ) -> Result<tonic::Response<super::GetAccountDataResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct CryptomailAdminApiServiceServer<T: CryptomailAdminApiService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CryptomailAdminApiService> CryptomailAdminApiServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CryptomailAdminApiServiceServer<T>
    where
        T: CryptomailAdminApiService,
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
            match req.uri().path() {
                "/api.CryptomailAdminApiService/GetAccounts" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountsSvc<T: CryptomailAdminApiService>(pub Arc<T>);
                    impl<T: CryptomailAdminApiService>
                        tonic::server::UnaryService<super::GetAccountsRequest>
                        for GetAccountsSvc<T>
                    {
                        type Response = super::GetAccountsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccountsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_accounts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccountsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.CryptomailAdminApiService/GetAccountData" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountDataSvc<T: CryptomailAdminApiService>(pub Arc<T>);
                    impl<T: CryptomailAdminApiService>
                        tonic::server::UnaryService<super::GetAccountDataRequest>
                        for GetAccountDataSvc<T>
                    {
                        type Response = super::GetAccountDataResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccountDataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_account_data(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccountDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: CryptomailAdminApiService> Clone for CryptomailAdminApiServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: CryptomailAdminApiService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CryptomailAdminApiService> tonic::transport::NamedService
        for CryptomailAdminApiServiceServer<T>
    {
        const NAME: &'static str = "api.CryptomailAdminApiService";
    }
}
