//////////////////////
//
// Store is a decentralized data storage protocol implementing using DHT and S/kad algorithm.
// The following are the protocol p2p protocol messages.
//
// The protocol is implemented over the messaging service protocol which provides authentication and messages integrity.
// The protocol is implemented as a set of supported requests and responses that should be sent in a new or existing DR session between 2 nodes
// The protocol is used to implement NameService. Nodes only agree to store specific types of data objects on the network (e.g. identity bundles)
//
// SNP name service is implemented using the store protocol.
//
// Providers provide the store service to their clients and to any other verified provider (participates in consensus protocol)
// Bootstrap nodes provide the store to ANY remote client so new clients can locate service providers bundles without having to know providers dialup info. Just based on provider id.
//
//

/// Needed as part of s/kad algo - connect to a node using dialup info and obtain its identity
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingResponse {
    #[prost(message, optional, tag = "1")]
    pub provider_net_info: ::core::option::Option<super::core_types::ProviderNetInfo>,
}
/// Search for ProviderNetInfo of node given a key in the xor keyspace (256 bits)
/// Key can be a sha512 of a node's public key or arbitrary value in the keyspace.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeLookupRequest {
    /// the dad id is a sha256 of the public key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// the max number of results caller wants to get
    #[prost(uint32, tag = "2")]
    pub max_results: u32,
    /// requester reported net info - used so we can ping it and add it to routing table
    #[prost(message, optional, tag = "3")]
    pub net_info: ::core::option::Option<super::core_types::ProviderNetInfo>,
}
/// Response includes up to max_results requested nodes closest to lookup key known to callee.
/// For additional implementation info. See Kad algo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeLookupResponse {
    /// up to max_results
    #[prost(message, repeated, tag = "2")]
    pub provider_net_infos: ::prost::alloc::vec::Vec<super::core_types::ProviderNetInfo>,
}
/// Get distributed stored data request for a key
/// This is used to get ProvidersBundle and ProviderSignedClientBundle stored on the network
/// For additional implementation info. See Kad algo.
/// The key is deterministic determined by the value and its type by all nodes. e.g. SHA512(id(value) || type))
/// and must be uniformly distributed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataRequest {
    /// keys must be a uniformly distributed over the key-space. e.g. sha256('client_bundle' || client_id).
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// requester reported net info - used so we can ping it and add it to routing table
    #[prost(message, optional, tag = "2")]
    pub net_info: ::core::option::Option<super::core_types::ProviderNetInfo>,
}
/// response include the data or 0 bytes if callee doesn't have the data stored for the key
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataResponse {
    /// encoded data. Client needs to decode based on expected data type (which is implicit in key)
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
////////////////////

/// Request to stare data.
/// For additional implementation info. See Kad algo.
///
/// key must combine data type and its unique key and have uniformly distributed bits.
/// e.g. SHA256('provider_bundle' || provider_id) or SHA256('client_bundle' || client_id)
/// Any node can call this as needed by the kademlia algorithm to refresh data to new nodes
/// so the request is not signed but received should verify signature on the data before storing
/// Providers should only store verified data in the supported data types - see StoredDataType.
/// The key is deterministic determined by the value and its type by all nodes. e.g. SHA512(id(value) || type))
/// and must be uniformly distributed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreDataRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// serialized protobuf data of a given type
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// serialized data type
    #[prost(enumeration = "StoredDataType", tag = "3")]
    pub data_type: i32,
    /// requester reported net info - used so we can ping it and add it to routing table
    #[prost(message, optional, tag = "4")]
    pub net_info: ::core::option::Option<super::core_types::ProviderNetInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreDataResponse {}
/// Supported data types to be stored in a SNP decentralized storage system
/// Note that each data item is signed by its created so only authenticated data is saved
/// Only data signed by its author that can be verified as such should be saved in the discovery data store.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StoredDataType {
    ClientBundle = 0,
    /// TODO: consider also storing provider terms of service in the distributed storage
    ProviderBundle = 1,
}
