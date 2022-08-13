#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<super::payments::Address>,
    #[prost(uint64, tag = "2")]
    pub nonce: u64,
    #[prost(message, repeated, tag = "3")]
    pub balances: ::prost::alloc::vec::Vec<super::payments::Amount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    /// incremental id
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(message, repeated, tag = "2")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
    /// entity which seal the block
    #[prost(message, optional, tag = "3")]
    pub sealer: ::core::option::Option<super::core_types::EntityId>,
    /// entity's signature
    #[prost(bytes = "vec", tag = "4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// Transaction fee
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionFee {
    /// fee amount
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::payments::Amount>,
    /// empty when transaction sender pays the fee. Otherwise, payer payment address public key.
    #[prost(bytes = "vec", tag = "2")]
    pub payer_public_key: ::prost::alloc::vec::Vec<u8>,
}
/// Transaction can be submitted with fee payed by sender or with fee paid by another party.
/// When sender pays the fee he provides the fee amount and signs it as part of the tx signature and the fee_signature is empty.
/// When another party pays the transaction fee, the sender signs all fields besides the fee and the fee payer provides the fee info and signs it and the other tx fields.
/// The 3rd party tx fee feature is designed so providers can pay the transaction fees for transactions related to provided service for their users.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    /// sender pub key
    #[prost(bytes = "vec", tag = "1")]
    pub sender_pub_key: ::prost::alloc::vec::Vec<u8>,
    /// sender tx nonce
    #[prost(uint64, tag = "2")]
    pub counter: u64,
    /// Optional entity id (such as provider or user)
    #[prost(message, optional, tag = "3")]
    pub entity_id: ::core::option::Option<super::core_types::EntityId>,
    /// Subnet blockchain id
    #[prost(uint32, tag = "4")]
    pub net_id: u32,
    /// sender signature on all other fields besides fee and fee_signature field when tx is meant to be payed by another entity
    #[prost(bytes = "vec", tag = "8")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// transaction fee
    #[prost(message, optional, tag = "9")]
    pub fee: ::core::option::Option<TransactionFee>,
    /// signature of tx fee payer on all other fields in case sender doesn't pay the fee. Empty otherwise
    #[prost(bytes = "vec", tag = "10")]
    pub fee_signature: ::prost::alloc::vec::Vec<u8>,
    /// Transaction data
    #[prost(oneof = "transaction::Data", tags = "5, 6, 7")]
    pub data: ::core::option::Option<transaction::Data>,
}
/// Nested message and enum types in `Transaction`.
pub mod transaction {
    /// Transaction data
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "5")]
        PaymentTransaction(super::PaymentTransactionData),
        #[prost(message, tag = "6")]
        ProviderBundle(super::ProviderBundleTransactionData),
        #[prost(message, tag = "7")]
        ClientBundle(super::ClientBundleTransactionData),
    }
}
/// a blockchain transaction - can be a user-to-user payment or a user-to-provider payment
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentTransactionData {
    /// receiver account
    #[prost(message, optional, tag = "1")]
    pub receiver: ::core::option::Option<super::payments::Address>,
    /// tx amount
    #[prost(message, optional, tag = "3")]
    pub coins: ::core::option::Option<super::payments::Amount>,
    /// invoice or contract id this payment is for (optional)
    #[prost(uint64, tag = "4")]
    pub id: u64,
}
/// Provider identity bundle
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderBundleTransactionData {
    #[prost(message, optional, tag = "1")]
    pub provider_bundle: ::core::option::Option<super::core_types::ProviderIdentityBundle>,
}
/// Users client identity bundle signed by a provider
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientBundleTransactionData {
    #[prost(message, optional, tag = "1")]
    pub client_bundle:
        ::core::option::Option<super::core_types::ProviderSignedClientIdentityBundle>,
}
/// Information about a transaction obtainable from pool or from ledger
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfo {
    /// this is a h ash of binary Transaction data - implied from transaction
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::payments::TransactionId>,
    /// transaction current state - in pool, on ledger, unkown, etc...
    #[prost(enumeration = "TransactionState", tag = "2")]
    pub state: i32,
    #[prost(message, optional, tag = "3")]
    pub transaction: ::core::option::Option<Transaction>,
    /// implied from Transaction
    #[prost(enumeration = "TransactionType", tag = "4")]
    pub transaction_type: i32,
    /// block id if transaction is on ledger
    #[prost(uint64, tag = "5")]
    pub block_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitTransactionRequest {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<Transaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitTransactionResponse {
    /// computed by node - just hash of transaction binary data
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::payments::TransactionId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProviderIdentityBundleRequest {
    #[prost(message, optional, tag = "1")]
    pub entity_id: ::core::option::Option<super::core_types::EntityId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProviderIdentityBundleResponse {
    #[prost(message, optional, tag = "1")]
    pub provider_bundle: ::core::option::Option<super::core_types::ProviderIdentityBundle>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClientIdentityBundleRequest {
    #[prost(message, optional, tag = "1")]
    pub entity_id: ::core::option::Option<super::core_types::EntityId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClientIdentityBundleResponse {
    #[prost(message, optional, tag = "1")]
    pub client_bundle:
        ::core::option::Option<super::core_types::ProviderSignedClientIdentityBundle>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClientsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClientsResponse {
    #[prost(message, repeated, tag = "1")]
    pub clients_bundles:
        ::prost::alloc::vec::Vec<super::core_types::ProviderSignedClientIdentityBundle>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProvidersRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProvidersResponse {
    #[prost(message, repeated, tag = "1")]
    pub providers_bundles: ::prost::alloc::vec::Vec<super::core_types::ProviderIdentityBundle>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentBlockRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockRequest {
    #[prost(uint64, tag = "1")]
    pub block_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockResponse {
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<Block>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBalanceRequest {
    /// address
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<super::payments::Address>,
    ///  balance
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::payments::Amount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBalanceResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionRequest {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::payments::TransactionId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionResponse {
    #[prost(message, optional, tag = "1")]
    pub transaction_info: ::core::option::Option<TransactionInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountRequest {
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<super::payments::Address>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountResponse {
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<Account>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksCountByEntityRequest {
    #[prost(message, optional, tag = "1")]
    pub entity_id: ::core::option::Option<super::core_types::EntityId>,
    /// get up to max_count most recent blocks. e.g. last 10 blocks.
    #[prost(uint32, tag = "2")]
    pub max_count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksCountByEntityResponse {
    #[prost(uint64, tag = "1")]
    pub blocks_count: u64,
    #[prost(message, repeated, tag = "2")]
    pub blocks: ::prost::alloc::vec::Vec<Block>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionState {
    Unknown = 0,
    Submitted = 1,
    RejectedUnknownSender = 2,
    RejectedInternalError = 3,
    RejectedInvalidCounter = 4,
    RejectedInvalidData = 5,
    RejectedInvalidSignature = 6,
    RejectedInsufficientFunds = 7,
    /// approved but not yet finalized
    Confirmed = 8,
    /// finalized
    Final = 9,
    /// not found in ledger
    Unrecognized = 10,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionType {
    Unknown = 0,
    SendCoin = 1,
    SetProviderBundle = 2,
    SetClientBundle = 3,
}
#[doc = r" Generated client implementations."]
pub mod blockchain_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Subnet blockchain service"]
    #[doc = " Provided by the blockchain mock service for the alpha release, and by every node in the beta release."]
    #[doc = " The blockchain maintains accounts, identity bundles, blocks transactions in blocks or in mem pool"]
    #[derive(Debug, Clone)]
    pub struct BlockchainServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BlockchainServiceClient<tonic::transport::Channel> {
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
    impl<T> BlockchainServiceClient<T>
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
        ) -> BlockchainServiceClient<InterceptedService<T, F>>
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
            BlockchainServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Submit a transaction for processing"]
        pub async fn submit_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitTransactionRequest>,
        ) -> Result<tonic::Response<super::SubmitTransactionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/snp.blockchain.BlockchainService/SubmitTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets balance for an address. Address will be added to ledger if needed. Used in genesis only."]
        pub async fn set_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::SetBalanceRequest>,
        ) -> Result<tonic::Response<super::SetBalanceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/snp.blockchain.BlockchainService/SetBalance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets TransactionInfo for a tx id - will returned if in pool or on ledger"]
        pub async fn get_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionRequest>,
        ) -> Result<tonic::Response<super::GetTransactionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/snp.blockchain.BlockchainService/GetTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns account current state if exists on ledger"]
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
            let path = http::uri::PathAndQuery::from_static(
                "/snp.blockchain.BlockchainService/GetAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns block data"]
        pub async fn get_block(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockRequest>,
        ) -> Result<tonic::Response<super::GetBlockResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/snp.blockchain.BlockchainService/GetBlock");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns current block data"]
        pub async fn get_current_block(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCurrentBlockRequest>,
        ) -> Result<tonic::Response<super::GetBlockResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/snp.blockchain.BlockchainService/GetCurrentBlock",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the current provider identity bundle from the ledger for a provider id"]
        pub async fn get_provider_identity_bundle(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProviderIdentityBundleRequest>,
        ) -> Result<tonic::Response<super::GetProviderIdentityBundleResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/snp.blockchain.BlockchainService/GetProviderIdentityBundle",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get the current client identity bundle from the ledger for a client id"]
        pub async fn get_client_identity_bundle(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClientIdentityBundleRequest>,
        ) -> Result<tonic::Response<super::GetClientIdentityBundleResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/snp.blockchain.BlockchainService/GetClientIdentityBundle",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A temp convenience method used to get all clients registered in a network"]
        pub async fn get_clients(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClientsRequest>,
        ) -> Result<tonic::Response<super::GetClientsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/snp.blockchain.BlockchainService/GetClients",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns all providers registered in a network"]
        pub async fn get_providers(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProvidersRequest>,
        ) -> Result<tonic::Response<super::GetProvidersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/snp.blockchain.BlockchainService/GetProviders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns recent created blocks count by an entity - PoUW"]
        pub async fn get_validated_blocks_count_by_entity(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlocksCountByEntityRequest>,
        ) -> Result<tonic::Response<super::GetBlocksCountByEntityResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/snp.blockchain.BlockchainService/GetValidatedBlocksCountByEntity",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns recent sealed blocks count by an entity - PoUW"]
        pub async fn get_sealed_blocks_count_by_entity(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlocksCountByEntityRequest>,
        ) -> Result<tonic::Response<super::GetBlocksCountByEntityResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/snp.blockchain.BlockchainService/GetSealedBlocksCountByEntity",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod blockchain_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BlockchainServiceServer."]
    #[async_trait]
    pub trait BlockchainService: Send + Sync + 'static {
        #[doc = " Submit a transaction for processing"]
        async fn submit_transaction(
            &self,
            request: tonic::Request<super::SubmitTransactionRequest>,
        ) -> Result<tonic::Response<super::SubmitTransactionResponse>, tonic::Status>;
        #[doc = " Sets balance for an address. Address will be added to ledger if needed. Used in genesis only."]
        async fn set_balance(
            &self,
            request: tonic::Request<super::SetBalanceRequest>,
        ) -> Result<tonic::Response<super::SetBalanceResponse>, tonic::Status>;
        #[doc = " Gets TransactionInfo for a tx id - will returned if in pool or on ledger"]
        async fn get_transaction(
            &self,
            request: tonic::Request<super::GetTransactionRequest>,
        ) -> Result<tonic::Response<super::GetTransactionResponse>, tonic::Status>;
        #[doc = " Returns account current state if exists on ledger"]
        async fn get_account(
            &self,
            request: tonic::Request<super::GetAccountRequest>,
        ) -> Result<tonic::Response<super::GetAccountResponse>, tonic::Status>;
        #[doc = " Returns block data"]
        async fn get_block(
            &self,
            request: tonic::Request<super::GetBlockRequest>,
        ) -> Result<tonic::Response<super::GetBlockResponse>, tonic::Status>;
        #[doc = " Returns current block data"]
        async fn get_current_block(
            &self,
            request: tonic::Request<super::GetCurrentBlockRequest>,
        ) -> Result<tonic::Response<super::GetBlockResponse>, tonic::Status>;
        #[doc = " Gets the current provider identity bundle from the ledger for a provider id"]
        async fn get_provider_identity_bundle(
            &self,
            request: tonic::Request<super::GetProviderIdentityBundleRequest>,
        ) -> Result<tonic::Response<super::GetProviderIdentityBundleResponse>, tonic::Status>;
        #[doc = " Get the current client identity bundle from the ledger for a client id"]
        async fn get_client_identity_bundle(
            &self,
            request: tonic::Request<super::GetClientIdentityBundleRequest>,
        ) -> Result<tonic::Response<super::GetClientIdentityBundleResponse>, tonic::Status>;
        #[doc = " A temp convenience method used to get all clients registered in a network"]
        async fn get_clients(
            &self,
            request: tonic::Request<super::GetClientsRequest>,
        ) -> Result<tonic::Response<super::GetClientsResponse>, tonic::Status>;
        #[doc = " Returns all providers registered in a network"]
        async fn get_providers(
            &self,
            request: tonic::Request<super::GetProvidersRequest>,
        ) -> Result<tonic::Response<super::GetProvidersResponse>, tonic::Status>;
        #[doc = " Returns recent created blocks count by an entity - PoUW"]
        async fn get_validated_blocks_count_by_entity(
            &self,
            request: tonic::Request<super::GetBlocksCountByEntityRequest>,
        ) -> Result<tonic::Response<super::GetBlocksCountByEntityResponse>, tonic::Status>;
        #[doc = " Returns recent sealed blocks count by an entity - PoUW"]
        async fn get_sealed_blocks_count_by_entity(
            &self,
            request: tonic::Request<super::GetBlocksCountByEntityRequest>,
        ) -> Result<tonic::Response<super::GetBlocksCountByEntityResponse>, tonic::Status>;
    }
    #[doc = " Subnet blockchain service"]
    #[doc = " Provided by the blockchain mock service for the alpha release, and by every node in the beta release."]
    #[doc = " The blockchain maintains accounts, identity bundles, blocks transactions in blocks or in mem pool"]
    #[derive(Debug)]
    pub struct BlockchainServiceServer<T: BlockchainService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BlockchainService> BlockchainServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BlockchainServiceServer<T>
    where
        T: BlockchainService,
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
                "/snp.blockchain.BlockchainService/SubmitTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct SubmitTransactionSvc<T: BlockchainService>(pub Arc<T>);
                    impl<T: BlockchainService>
                        tonic::server::UnaryService<super::SubmitTransactionRequest>
                        for SubmitTransactionSvc<T>
                    {
                        type Response = super::SubmitTransactionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubmitTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).submit_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubmitTransactionSvc(inner);
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
                "/snp.blockchain.BlockchainService/SetBalance" => {
                    #[allow(non_camel_case_types)]
                    struct SetBalanceSvc<T: BlockchainService>(pub Arc<T>);
                    impl<T: BlockchainService> tonic::server::UnaryService<super::SetBalanceRequest>
                        for SetBalanceSvc<T>
                    {
                        type Response = super::SetBalanceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetBalanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_balance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetBalanceSvc(inner);
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
                "/snp.blockchain.BlockchainService/GetTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionSvc<T: BlockchainService>(pub Arc<T>);
                    impl<T: BlockchainService>
                        tonic::server::UnaryService<super::GetTransactionRequest>
                        for GetTransactionSvc<T>
                    {
                        type Response = super::GetTransactionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionSvc(inner);
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
                "/snp.blockchain.BlockchainService/GetAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountSvc<T: BlockchainService>(pub Arc<T>);
                    impl<T: BlockchainService> tonic::server::UnaryService<super::GetAccountRequest>
                        for GetAccountSvc<T>
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
                "/snp.blockchain.BlockchainService/GetBlock" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockSvc<T: BlockchainService>(pub Arc<T>);
                    impl<T: BlockchainService> tonic::server::UnaryService<super::GetBlockRequest> for GetBlockSvc<T> {
                        type Response = super::GetBlockResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockSvc(inner);
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
                "/snp.blockchain.BlockchainService/GetCurrentBlock" => {
                    #[allow(non_camel_case_types)]
                    struct GetCurrentBlockSvc<T: BlockchainService>(pub Arc<T>);
                    impl<T: BlockchainService>
                        tonic::server::UnaryService<super::GetCurrentBlockRequest>
                        for GetCurrentBlockSvc<T>
                    {
                        type Response = super::GetBlockResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCurrentBlockRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_current_block(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCurrentBlockSvc(inner);
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
                "/snp.blockchain.BlockchainService/GetProviderIdentityBundle" => {
                    #[allow(non_camel_case_types)]
                    struct GetProviderIdentityBundleSvc<T: BlockchainService>(pub Arc<T>);
                    impl<T: BlockchainService>
                        tonic::server::UnaryService<super::GetProviderIdentityBundleRequest>
                        for GetProviderIdentityBundleSvc<T>
                    {
                        type Response = super::GetProviderIdentityBundleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProviderIdentityBundleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_provider_identity_bundle(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProviderIdentityBundleSvc(inner);
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
                "/snp.blockchain.BlockchainService/GetClientIdentityBundle" => {
                    #[allow(non_camel_case_types)]
                    struct GetClientIdentityBundleSvc<T: BlockchainService>(pub Arc<T>);
                    impl<T: BlockchainService>
                        tonic::server::UnaryService<super::GetClientIdentityBundleRequest>
                        for GetClientIdentityBundleSvc<T>
                    {
                        type Response = super::GetClientIdentityBundleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetClientIdentityBundleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_client_identity_bundle(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetClientIdentityBundleSvc(inner);
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
                "/snp.blockchain.BlockchainService/GetClients" => {
                    #[allow(non_camel_case_types)]
                    struct GetClientsSvc<T: BlockchainService>(pub Arc<T>);
                    impl<T: BlockchainService> tonic::server::UnaryService<super::GetClientsRequest>
                        for GetClientsSvc<T>
                    {
                        type Response = super::GetClientsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetClientsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_clients(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetClientsSvc(inner);
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
                "/snp.blockchain.BlockchainService/GetProviders" => {
                    #[allow(non_camel_case_types)]
                    struct GetProvidersSvc<T: BlockchainService>(pub Arc<T>);
                    impl<T: BlockchainService>
                        tonic::server::UnaryService<super::GetProvidersRequest>
                        for GetProvidersSvc<T>
                    {
                        type Response = super::GetProvidersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProvidersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_providers(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProvidersSvc(inner);
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
                "/snp.blockchain.BlockchainService/GetValidatedBlocksCountByEntity" => {
                    #[allow(non_camel_case_types)]
                    struct GetValidatedBlocksCountByEntitySvc<T: BlockchainService>(pub Arc<T>);
                    impl<T: BlockchainService>
                        tonic::server::UnaryService<super::GetBlocksCountByEntityRequest>
                        for GetValidatedBlocksCountByEntitySvc<T>
                    {
                        type Response = super::GetBlocksCountByEntityResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlocksCountByEntityRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_validated_blocks_count_by_entity(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetValidatedBlocksCountByEntitySvc(inner);
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
                "/snp.blockchain.BlockchainService/GetSealedBlocksCountByEntity" => {
                    #[allow(non_camel_case_types)]
                    struct GetSealedBlocksCountByEntitySvc<T: BlockchainService>(pub Arc<T>);
                    impl<T: BlockchainService>
                        tonic::server::UnaryService<super::GetBlocksCountByEntityRequest>
                        for GetSealedBlocksCountByEntitySvc<T>
                    {
                        type Response = super::GetBlocksCountByEntityResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlocksCountByEntityRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_sealed_blocks_count_by_entity(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSealedBlocksCountByEntitySvc(inner);
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
    impl<T: BlockchainService> Clone for BlockchainServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: BlockchainService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BlockchainService> tonic::transport::NamedService for BlockchainServiceServer<T> {
        const NAME: &'static str = "snp.blockchain.BlockchainService";
    }
}
