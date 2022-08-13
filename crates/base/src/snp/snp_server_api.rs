/////////////////
//
// Core SNP messages used in MessagingService)
//
/////////////////

/// A DR session header - see DR protocol def of HEADER(dh,pn,n)
///
/// DR session header
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DrSessionHeader {
    /// Unique session id - first created by session initiator. Used by other party to retrieve the session from storage when it needs to use it.
    #[prost(uint64, tag = "1")]
    pub session_id: u64,
    /// the public ratchet key currently in use by the sender in a dr session
    #[prost(message, optional, tag = "2")]
    pub dr_pub_key: ::core::option::Option<super::core_types::PublicKey>,
    /// the number of messages in the previous sending chain (PN in dr paper)
    #[prost(uint32, tag = "3")]
    pub prev_count: u32,
    /// the number of messages in the current sending chain
    #[prost(uint32, tag = "4")]
    pub count: u32,
}
/// A simple message has a DR header and an encrypted TypedMessage, encrypted using DR. See DR protocol for more info.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    /// DR protocol unencrypted header
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<DrSessionHeader>,
    /// a DR encrypted TypedMessage
    #[prost(bytes = "vec", tag = "2")]
    pub enc_typed_msg: ::prost::alloc::vec::Vec<u8>,
}
/// Typed message is a self-described typed message designated to a specific receiver authenticated by a sender.
/// It enables dynamic decoding of a proto-encoded messages to a specific runtime type which is needed as protobuf 3
/// does not support self-describing messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedMessage {
    /// msg creation time signed by sender (to avoid replay later on)
    #[prost(uint64, tag = "1")]
    pub time_stamp: u64,
    /// message type (enum)
    #[prost(enumeration = "MessageType", tag = "2")]
    pub msg_type: i32,
    /// Serialized protobuf message of msg_type
    #[prost(bytes = "vec", tag = "3")]
    pub message: ::prost::alloc::vec::Vec<u8>,
    /// The fields below are here here to protect them from being transferred as cleartext over the network in message wrappers.
    ///
    /// Message designated receiver id (long term public key) - used to prevent fake messages by sender sent to other receivers
    #[prost(message, optional, tag = "4")]
    pub receiver: ::core::option::Option<super::core_types::EntityId>,
    /// Message sender id (long term public key)
    #[prost(message, optional, tag = "5")]
    pub sender: ::core::option::Option<super::core_types::EntityId>,
    /// Message sender signature on all other fields - authenticating the msg
    #[prost(message, optional, tag = "6")]
    pub signature: ::core::option::Option<super::core_types::Signature>,
}
/// A 2-party DR session request using the X2DH protocol. Can be sent by Alice to Bob.
/// Can also be sent as an inner message sent from Alice to Bob designated to Charlie.
/// So receiver may be Bob or Charlie. DR is bootstrapped using shared secret and AD computed via the X2DH protocol.
/// Receiver should start a DR session with caller, decrypt the encrypted message
/// extract the caller id from the enc payload, verify signature on internal message
/// and only then associate DR session with the public long term id of the caller.
/// this is done so we don't leak sender public id in this clear-text network message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSessionRequest {
    /// request time signed by sender (to avoid replays at much later time)
    #[prost(uint64, tag = "1")]
    pub time_stamp: u64,
    /// Receiver's IKa - long term public key
    #[prost(message, optional, tag = "2")]
    pub receiver: ::core::option::Option<super::core_types::EntityId>,
    /// Alice's x25519 protocol pub key 2. see X2DH protocol.
    #[prost(message, optional, tag = "3")]
    pub sender_ephemeral_key: ::core::option::Option<super::core_types::PublicKey>,
    /// Receiver's bundle id used by sender. Also identifies the pre-key.
    #[prost(uint64, tag = "4")]
    pub receiver_bundle_id: u64,
    /// one time pre-key Bob should use for session (optional)
    #[prost(uint64, tag = "5")]
    pub receiver_one_time_prekey_id: u64,
    /// First message from Alice to Receiver. Enc in DR protocol in a new DR session Alice created with Receiver.
    #[prost(message, optional, tag = "6")]
    pub message: ::core::option::Option<Message>,
    /// on all other data (with long-term id key inside message)
    #[prost(message, optional, tag = "7")]
    pub sender_signature: ::core::option::Option<super::core_types::Signature>,
    /// net id - designed to avoid mixing of p2p messages between 2 different SNP networks
    #[prost(uint32, tag = "8")]
    pub net_id: u32,
    /// Snp protocol semantic version number implemented by caller
    #[prost(string, tag = "9")]
    pub protocol_version: ::prost::alloc::string::String,
}
/// A DDMessage is a NewSessionRequest or a Message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DrMessage {
    #[prost(oneof = "dr_message::Data", tags = "1, 2")]
    pub data: ::core::option::Option<dr_message::Data>,
}
/// Nested message and enum types in `DRMessage`.
pub mod dr_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "1")]
        NewSessionRequest(super::NewSessionRequest),
        #[prost(message, tag = "2")]
        Message(super::Message),
    }
}
/// Metadata about a DRMessage designated to a client that is stored
/// on provider for client delivery
/// Note that provider doesn't have by design any additional message meta-data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientMessageMetadata {
    /// unique id created by provider who stores a message to client by this id. Also used in receipt
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// Time of reception by provider
    #[prost(uint64, tag = "2")]
    pub received_date: u64,
    /// provider price to send message to the client
    #[prost(uint64, tag = "3")]
    pub price: u64,
    /// message byte size
    #[prost(uint64, tag = "4")]
    pub size: u64,
    /// how long will server hold this message for client before deleting it
    #[prost(uint64, tag = "5")]
    pub ttl: u64,
}
/// A list of messages metadata
/// Sent from provider to its client so client can decide which messages to request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientMessagesMetadata {
    #[prost(message, repeated, tag = "1")]
    pub messages_metadata: ::prost::alloc::vec::Vec<ClientMessageMetadata>,
}
/// Receiver returns a response message in the new DR session between the parties based on the message that the sender sent
/// or an error status if it failed or refused to create new DR session between the parties.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSessionResponse {
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<Message>,
}
/// A message request between two parties using an existing DR session between them
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageRequest {
    /// message enc in Dr session between sender and receiver
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<Message>,
}
/// A response to a MessageRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageResponse {
    /// the response message, encoded in the DR session between the parties
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<Message>,
}
/// A request by a client to subscribe to messages designated to him that reached his provider
/// Note that for now we don't let clients start a new DR session by sending a DRMessage
/// so they always have to send a Message (and not a NewSessionRequest) here
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeToClientMessagesRequest {
    #[prost(message, optional, tag = "1")]
    pub dr_message: ::core::option::Option<Message>,
}
/// empty message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeToClientMessagesRequestPayload {}
/// A request to get the current provider identity bundle
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIdentityBundleRequest {
    /// Snp protocol semantic version number implemented by caller
    #[prost(string, tag = "1")]
    pub protocol_version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIdentityBundleResponse {
    #[prost(message, optional, tag = "1")]
    pub bundle: ::core::option::Option<super::core_types::ProviderIdentityBundle>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTermsOfServiceRequest {
    #[prost(string, tag = "1")]
    pub promo_code: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTermsOfServiceResponse {
    #[prost(message, optional, tag = "1")]
    pub terms: ::core::option::Option<super::core_types::ServiceTermsBundle>,
}
//////////////////////////////////////////////////////////////////////////////////////
//
// SNP - Messages routing protocol
//
// The api is implemented by request and a response message pairs.
// Caller sends a request and receives a response or a status error message back from the receiver.
// All messages are sent as TypedMessages using core SNP protocol by clients and providers to a provider
//
//////////////////////////////////////////////////////////////////////////////////////

/// A client A requests for its service provider (SA) to forward a message to another provider.
/// Forward_message is encrypted with eph-dh to the other service provider (SB).
/// This message is implemented by providers that accept routing messages for their clients.
/// See the basic user-to-user messaging flow for more info.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteMessageRequest {
    /// the message for the destination service provider (SB)
    #[prost(message, optional, tag = "1")]
    pub forward_message: ::core::option::Option<ForwardMessageRequest>,
    /// destination service provider dial-up info (temp here)
    #[prost(message, optional, tag = "2")]
    pub dialup_info: ::core::option::Option<super::core_types::DialupInfo>,
}
/// empty as it only includes status
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteMessageResponse {}
////////////////////////////////

/// A request to a provider to send a message to one of its serviced clients.
/// Clients use this to send a message to another client serviced by the same provider
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMessageToServicedClientRequest {
    /// new session request or message to serviced client
    #[prost(message, optional, tag = "1")]
    pub dr_message: ::core::option::Option<DrMessage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMessageToServicedClientResponse {}
/// A request from client to deliver messages designated to it from its provider
/// Includes a receipt for te messages delivery price
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeliverClientMessagesRequest {
    /// payment include item ids
    #[prost(message, optional, tag = "1")]
    pub payment: ::core::option::Option<super::payments::Payment>,
}
/// A response from a server to deliver messages to a client. Includes the receipt id of the client's payment
/// and the full messages content it has pending for delivery to the client.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeliverClientMessagesResponse {
    #[prost(uint64, tag = "1")]
    pub receipt_id: u64,
    #[prost(message, repeated, tag = "2")]
    pub messages: ::prost::alloc::vec::Vec<DrMessage>,
}
////////////////////////////////

/// The sender is requesting the receiver to forward the message to one of the entities it is providing a service for.
/// Payload is encrypted using key and ad obtained from EDH and can be a NewSessionRequest sent to a client that
/// the receiver is providing service for or a Message to that client.
/// Sender should create a new ephemeral key for each such message and destroy the private key once
/// the message was sent - it should be a one time key
/// >>> there is no DR session created between sender and receiver only a 1 time key to decrypt the payload
/// The enc/dec key is obtained by doing DH with receiver public pre-key and sender ephemeral key
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardMessageRequest {
    /// Provider receiver id - long term public key
    #[prost(message, optional, tag = "1")]
    pub receiver: ::core::option::Option<super::core_types::EntityId>,
    /// Receiver's bundle id used by sender. Also identifies the pre-key.
    #[prost(uint64, tag = "2")]
    pub receiver_bundle_id: u64,
    /// Sender's x25519 protocol pub key. see 2XDH protocol
    #[prost(message, optional, tag = "3")]
    pub sender_ephemeral_key: ::core::option::Option<super::core_types::PublicKey>,
    /// binary ForwardMessagePayload message
    #[prost(bytes = "vec", tag = "4")]
    pub enc_payload: ::prost::alloc::vec::Vec<u8>,
}
/// Payload is a NewSessionRequest or a Message request
/// to another entity that the ForwardMessage receiver
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardMessagePayload {
    /// we need this because Message doesn't have receiver id in it and provider needs it.
    #[prost(message, optional, tag = "1")]
    pub receiver: ::core::option::Option<super::core_types::EntityId>,
    #[prost(message, optional, tag = "2")]
    pub dr_message: ::core::option::Option<DrMessage>,
}
/// The response just indicates a status to the sender who forwarded the message to the receiver
/// It is protected with the channel the sender rand the receiver have. e.g. a DR session.
/// So the response can be a DR-protected response... e.g. a Message with this as its internal
/// typed message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardMessageResponse {}
/// MessageType specifies the run-time type of a TypedMessage
/// Used for dynamic decoding of messages to a runtime typed object in code.
/// New protocol messages type ids need to be added here
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageType {
    //
    // Public service protocol messages (in a DR session)
    /////////////////////////////
    /// A request to send provider's signed terms of service bundle
    ServiceTermsRequest = 0,
    ServiceTermsResponse = 1,
    /// A request from a client to start getting serviced by a provider
    StartServiceRequest = 2,
    StartServiceResponse = 3,
    //
    // Provider messaging service protocol messages
    // Clients send these messages to their providers while being serviced
    /////////////////////////////
    /// A request from a client to stop being provided by a provider
    StopServiceRequest = 4,
    StopServiceResponse = 5,
    /// A request from a client to its provider to forward a message to another client via that client's provider
    ForwardMessageRequest = 6,
    ForwardMessageResponse = 7,
    /// A client request to receive to a stream messages designated to it from other entities via its provider
    /// Client unsubscribes by closing his end of the stream
    SubscribeClientMessages = 8,
    //
    // Provider to provider protocol messages
    /////////////////////////////
    /// A request from a provider to another provider to route a message to one of its client from one of the sender's provider clients
    RouteMessageRequest = 9,
    RouteMessageResponse = 10,
    //
    // Client to Client messages
    // These messages are routed from a client to another client
    /////////////////////////////
    /// A 1:1 text message from a client to another client
    TextMessageRequest = 11,
    TextMessageResponse = 12,
    /// Client request to subscribe to a client channel such as a status updates channel or to join a group
    ChannelSubscribeRequest = 13,
    ChannelSubscribeResponse = 14,
    /// client request to subscribe to a client channel such as a status updates channel or to leave a group
    ChannelUnsubscribeRequest = 15,
    ChannelUnsubscribeResponse = 16,
    /// a new channel message from channel creator client to a subscriber (or group member) client
    ChannelMessage = 17,
    /// a request by a client to post a reply on a channel post a message to a group in a channel he doesn't own
    /// message is sent by author to the channel creator
    ChannelMessageRequest = 18,
    ChannelMessageResponse = 19,
    /// A client is requesting to purchase a paid content item published by another client
    BuyItemRequest = 20,
    /// Response should have include item if requesters paid for it.
    BuyItemResponse = 21,
    /// A client is requesting to get a list of paid items currently for sale by another client
    ListPaidItemsRequest = 22,
    /// Response returns a list of meta-data about paid content items available for sale by a client
    ListPaidItemsResponse = 23,
    /// Metadata about messages that a provider has for a client
    ClientMessagesMetadata = 24,
    /// A request from client to its provider to deliver messages it has for it
    DeliverClientMessagesRequest = 25,
    /// A response from provider to client request with the requested messages
    DeliverClientMessagesResponse = 26,
    /// A request to get a provider's current id bundle
    GetProviderBundleRequest = 27,
    /// Get provider bundle response
    GetProviderBundleResponse = 28,
    /// A request to get a client's current id bundle
    GetClientBundleRequest = 29,
    /// Client id bundle response
    GetClientBundleResponse = 30,
    /// A request to store encrypted client data on provider (used to backup user data when migrating between devices)
    StoreDataRequest = 31,
    /// Response to a store data request
    StoreDataResponse = 32,
    /// A request to read previously stored client data
    ReadDataRequest = 33,
    /// Read data response
    ReadDataResponse = 34,
    /// A ping request to a remote node, asking for its up-to-date dialup info
    PingNodeRequest = 35,
    /// A ping response including new node dialup info (for follow-up requests)
    PingNodeResponse = 36,
}
#[doc = r" Generated client implementations."]
pub mod provider_core_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " todo: Change to ProviderService"]
    #[derive(Debug, Clone)]
    pub struct ProviderCoreServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProviderCoreServiceClient<tonic::transport::Channel> {
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
    impl<T> ProviderCoreServiceClient<T>
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
        ) -> ProviderCoreServiceClient<InterceptedService<T, F>>
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
            ProviderCoreServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Client (or another Service Provider) requests to create a new DR enc session using 2XDH"]
        #[doc = " and sends a first message in this session. This is basically the messaging service handshake protocol with an optional first message."]
        pub async fn new_session(
            &mut self,
            request: impl tonic::IntoRequest<super::NewSessionRequest>,
        ) -> Result<tonic::Response<super::NewSessionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/snp.server_api.ProviderCoreService/NewSession",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Client (or another Service Provider) requests to process new message in an existing DR session."]
        #[doc = " All higher-level protocol messages are sent using this method."]
        pub async fn message(
            &mut self,
            request: impl tonic::IntoRequest<super::MessageRequest>,
        ) -> Result<tonic::Response<super::MessageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/snp.server_api.ProviderCoreService/Message");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A streaming api for clients served by this provider to get incoming messages from other entities (or other types of messages)"]
        #[doc = " pushed to it. Providers announce the availability of this service via their terms of service."]
        #[doc = " In the full product implementation, servers push meta-data about a new message (id, byte_size), clients pay for it and request the message by providing receipt and message id"]
        pub async fn subscribe_to_client_messages(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeToClientMessagesRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::DrMessage>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/snp.server_api.ProviderCoreService/SubscribeToClientMessages",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Returns self's identity bundle for callers that dialed up directly."]
        #[doc = " This is useful for bootstrapping a network from a list of known providers addresses."]
        pub async fn get_identity_bundle(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIdentityBundleRequest>,
        ) -> Result<tonic::Response<super::GetIdentityBundleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/snp.server_api.ProviderCoreService/GetIdentityBundle",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get provider terms of service by id - these include terms for public content"]
        #[doc = " and for encrypted content."]
        pub async fn get_terms_of_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTermsOfServiceRequest>,
        ) -> Result<tonic::Response<super::GetTermsOfServiceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/snp.server_api.ProviderCoreService/GetTermsOfService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod provider_core_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ProviderCoreServiceServer."]
    #[async_trait]
    pub trait ProviderCoreService: Send + Sync + 'static {
        #[doc = " Client (or another Service Provider) requests to create a new DR enc session using 2XDH"]
        #[doc = " and sends a first message in this session. This is basically the messaging service handshake protocol with an optional first message."]
        async fn new_session(
            &self,
            request: tonic::Request<super::NewSessionRequest>,
        ) -> Result<tonic::Response<super::NewSessionResponse>, tonic::Status>;
        #[doc = " Client (or another Service Provider) requests to process new message in an existing DR session."]
        #[doc = " All higher-level protocol messages are sent using this method."]
        async fn message(
            &self,
            request: tonic::Request<super::MessageRequest>,
        ) -> Result<tonic::Response<super::MessageResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeToClientMessages method."]
        type SubscribeToClientMessagesStream: futures_core::Stream<Item = Result<super::DrMessage, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " A streaming api for clients served by this provider to get incoming messages from other entities (or other types of messages)"]
        #[doc = " pushed to it. Providers announce the availability of this service via their terms of service."]
        #[doc = " In the full product implementation, servers push meta-data about a new message (id, byte_size), clients pay for it and request the message by providing receipt and message id"]
        async fn subscribe_to_client_messages(
            &self,
            request: tonic::Request<super::SubscribeToClientMessagesRequest>,
        ) -> Result<tonic::Response<Self::SubscribeToClientMessagesStream>, tonic::Status>;
        #[doc = " Returns self's identity bundle for callers that dialed up directly."]
        #[doc = " This is useful for bootstrapping a network from a list of known providers addresses."]
        async fn get_identity_bundle(
            &self,
            request: tonic::Request<super::GetIdentityBundleRequest>,
        ) -> Result<tonic::Response<super::GetIdentityBundleResponse>, tonic::Status>;
        #[doc = " Get provider terms of service by id - these include terms for public content"]
        #[doc = " and for encrypted content."]
        async fn get_terms_of_service(
            &self,
            request: tonic::Request<super::GetTermsOfServiceRequest>,
        ) -> Result<tonic::Response<super::GetTermsOfServiceResponse>, tonic::Status>;
    }
    #[doc = " todo: Change to ProviderService"]
    #[derive(Debug)]
    pub struct ProviderCoreServiceServer<T: ProviderCoreService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ProviderCoreService> ProviderCoreServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ProviderCoreServiceServer<T>
    where
        T: ProviderCoreService,
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
                "/snp.server_api.ProviderCoreService/NewSession" => {
                    #[allow(non_camel_case_types)]
                    struct NewSessionSvc<T: ProviderCoreService>(pub Arc<T>);
                    impl<T: ProviderCoreService>
                        tonic::server::UnaryService<super::NewSessionRequest> for NewSessionSvc<T>
                    {
                        type Response = super::NewSessionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewSessionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_session(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewSessionSvc(inner);
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
                "/snp.server_api.ProviderCoreService/Message" => {
                    #[allow(non_camel_case_types)]
                    struct MessageSvc<T: ProviderCoreService>(pub Arc<T>);
                    impl<T: ProviderCoreService> tonic::server::UnaryService<super::MessageRequest> for MessageSvc<T> {
                        type Response = super::MessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MessageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).message(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MessageSvc(inner);
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
                "/snp.server_api.ProviderCoreService/SubscribeToClientMessages" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeToClientMessagesSvc<T: ProviderCoreService>(pub Arc<T>);
                    impl<T: ProviderCoreService>
                        tonic::server::ServerStreamingService<
                            super::SubscribeToClientMessagesRequest,
                        > for SubscribeToClientMessagesSvc<T>
                    {
                        type Response = super::DrMessage;
                        type ResponseStream = T::SubscribeToClientMessagesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeToClientMessagesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).subscribe_to_client_messages(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeToClientMessagesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/snp.server_api.ProviderCoreService/GetIdentityBundle" => {
                    #[allow(non_camel_case_types)]
                    struct GetIdentityBundleSvc<T: ProviderCoreService>(pub Arc<T>);
                    impl<T: ProviderCoreService>
                        tonic::server::UnaryService<super::GetIdentityBundleRequest>
                        for GetIdentityBundleSvc<T>
                    {
                        type Response = super::GetIdentityBundleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetIdentityBundleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_identity_bundle(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetIdentityBundleSvc(inner);
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
                "/snp.server_api.ProviderCoreService/GetTermsOfService" => {
                    #[allow(non_camel_case_types)]
                    struct GetTermsOfServiceSvc<T: ProviderCoreService>(pub Arc<T>);
                    impl<T: ProviderCoreService>
                        tonic::server::UnaryService<super::GetTermsOfServiceRequest>
                        for GetTermsOfServiceSvc<T>
                    {
                        type Response = super::GetTermsOfServiceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTermsOfServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_terms_of_service(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTermsOfServiceSvc(inner);
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
    impl<T: ProviderCoreService> Clone for ProviderCoreServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ProviderCoreService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ProviderCoreService> tonic::transport::NamedService for ProviderCoreServiceServer<T> {
        const NAME: &'static str = "snp.server_api.ProviderCoreService";
    }
} ///////////////////////////////////
  //
  // Application-level API provided to clients by a provider
  //
  // Clients sends a request message and providers respond with response message.
  // All other network entities should use the provider public service API.
  // Requests and responses are sent in an encrypted two-party DR session.
  //
  ///////////////////

/// Client requests to stop being serviced by this provider.
/// Client may optionally send a new identity bundle with its new provider
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopServiceRequest {
    /// optional
    #[prost(message, optional, tag = "1")]
    pub client_bundle: ::core::option::Option<super::core_types::ClientIdentityBundle>,
}
/// Stop service response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopServiceResponse {}
///////////////////////
// Current service status
//

/// Client request to receive its current service status
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceStatusRequest {}
/// return open invoices
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceStatusResponse {}
// todo: add GetOpenInvoices() - returns all client charges that he needs to pay such as monthly fee or monthly data storage fee

// todo: add client data backup service - SaveBundle(EncArchive), ListBundles(), DeleteBundle(id), GetBundle(id)

////////////////////////

/// Update client published pre-keys. Clients can refresh their pre-keys at any time.
/// Provider should sign if he's serving this client and store the updated signed bundle on the network's decentralized storage (Kad based)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBundleRequest {
    #[prost(message, optional, tag = "1")]
    pub bundle: ::core::option::Option<super::core_types::ClientIdentityBundle>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBundleResponse {
    #[prost(message, optional, tag = "1")]
    pub bundle: ::core::option::Option<super::core_types::ProviderSignedClientIdentityBundle>,
}
///////////////////////
// Users Data Backup and Restore SNP protocols

//////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientDataItem {
    /// provider storage item id
    #[prost(uint64, tag = "1")]
    pub storage_item_id: u64,
    /// file size in bytes
    #[prost(uint64, tag = "2")]
    pub size_bytes: u64,
    /// client set name
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// client set id
    #[prost(uint64, tag = "4")]
    pub id: u64,
    /// period client paid for
    #[prost(uint32, tag = "5")]
    pub period_months: u32,
    /// timestamp of storage start time
    #[prost(uint64, tag = "6")]
    pub store_time: u64,
    /// time storage agreement expires for the item
    #[prost(uint64, tag = "7")]
    pub expires: u64,
    /// price to download the data for client
    #[prost(uint64, tag = "8")]
    pub download_price: u64,
}
/// A request to store a data file on the provider. The file must be an archive of one or more user encrypted data files
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreClientDataRequest {
    #[prost(message, optional, tag = "1")]
    pub data_item: ::core::option::Option<ClientDataItem>,
    /// payment based on current service terms and period
    #[prost(message, optional, tag = "2")]
    pub payment: ::core::option::Option<super::payments::Payment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreClientDataResponse {
    #[prost(uint64, tag = "1")]
    pub storage_item_id: u64,
}
//////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClientDataItemsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClientDataItemsResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<ClientDataItem>,
}
//////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClientDataItemInfoRequest {
    #[prost(uint64, tag = "1")]
    pub storage_item_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClientDataItemInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<ClientDataItem>,
}
//////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadClientDataRequest {
    #[prost(uint64, tag = "1")]
    pub storage_item_id: u64,
    /// payment based on data size
    #[prost(message, optional, tag = "2")]
    pub payment: ::core::option::Option<super::payments::Payment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadClientDataResponse {
    /// encrypted uplaoded client data. e.g binary/zip mime compressed with zip. Uncompressed data is user-encrypted arbitrary data file.
    #[prost(message, optional, tag = "1")]
    pub media_item: ::core::option::Option<super::core_types::MediaItem>,
}
// Public services provided by a provider to anyone on the network.
// Clients, service providers or other types of nodes
// Implemented over messaging_service which provides DR session security for this message

// Get provider's current service terms bundle (might include new pre-keys)
// Clients should use GetTermsOfServiceRequest() and GetTermsOfServiceResponse messages defined in the messaging_service

/// Caller requests to start being serviced by this provider.
/// This should start the free trial period if it exists.
/// By calling this method the client agrees to the provider terms of service
/// as provided in provider bundle
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartServiceRequest {
    #[prost(message, optional, tag = "1")]
    pub bundle: ::core::option::Option<super::core_types::ClientIdentityBundle>,
    /// must be provided if provided requests a free to start service
    #[prost(message, optional, tag = "2")]
    pub payment: ::core::option::Option<super::payments::Payment>,
    #[prost(uint64, tag = "3")]
    pub service_contract_id: u64,
    /// fixed monthly fee or pay-per-usage
    #[prost(enumeration = "super::payments::PricingModel", tag = "4")]
    pub contract_options: i32,
}
/// Returns a signed client id bundle if accepted this client request
/// or an error status code
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartServiceResponse {
    #[prost(message, optional, tag = "1")]
    pub bundle: ::core::option::Option<super::core_types::ProviderSignedClientIdentityBundle>,
}
/// Request to get the most recent client bundle of a client provided by this provider
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClientPreKeysRequest {
    #[prost(message, optional, tag = "1")]
    pub client_id: ::core::option::Option<super::core_types::EntityId>,
}
/// Provider response. By returning a client's bundle that is provided by this provider,
/// the provider asserts that it is its service provider.
/// The provider id is signed by the client in the response envelope so client all asserts that
/// it is provided by the provider
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClientPreKeysResponse {
    #[prost(message, optional, tag = "1")]
    pub client_bundle:
        ::core::option::Option<super::core_types::ProviderSignedClientIdentityBundle>,
}
