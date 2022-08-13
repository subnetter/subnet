//////////////////
//
// Basic SNP data types used in services definitions and in complex types
//
/////////////////

/// An public encryption key
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublicKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyPair {
    #[prost(message, optional, tag = "1")]
    pub private_key: ::core::option::Option<PrivateKey>,
    #[prost(message, optional, tag = "2")]
    pub public_key: ::core::option::Option<PublicKey>,
}
/// an x2dh or x3dh pre-key
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreKey {
    /// x2dh protocol semantic version
    #[prost(string, tag = "1")]
    pub x2dh_version: ::prost::alloc::string::String,
    /// public key bytes
    #[prost(message, optional, tag = "2")]
    pub key: ::core::option::Option<PublicKey>,
    /// unique key id. This is the id of the bundle which published this prekey
    #[prost(uint64, tag = "3")]
    pub key_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreKeypair {
    /// x2dh protocol semantic version
    #[prost(string, tag = "1")]
    pub x2dh_version: ::prost::alloc::string::String,
    /// key pair
    #[prost(message, optional, tag = "2")]
    pub key_pair: ::core::option::Option<KeyPair>,
    /// unique key id
    #[prost(uint64, tag = "3")]
    pub key_id: u64,
}
/// A public entity such as client, group, channel or provider Id
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct EntityId {
    /// identity based on public key
    #[prost(message, optional, tag = "1")]
    pub public_key: ::core::option::Option<PublicKey>,
    /// optional
    #[prost(string, tag = "2")]
    pub nickname: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateEntityId {
    #[prost(message, optional, tag = "1")]
    pub key_pair: ::core::option::Option<KeyPair>,
    #[prost(string, tag = "2")]
    pub nickname: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    #[prost(uint32, tag = "1")]
    pub scheme_id: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentItem {
    /// unique item id
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// item's channel (when applicable)
    #[prost(bytes = "vec", tag = "2")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    /// authoring time-stamp
    #[prost(uint64, tag = "3")]
    pub created: u64,
    /// item's author
    #[prost(message, optional, tag = "4")]
    pub author: ::core::option::Option<EntityId>,
    /// optional expiration for self-destructing messages
    #[prost(uint64, tag = "5")]
    pub ttl: u64,
    /// price for a paid content item. 0 otherwise
    #[prost(uint64, tag = "6")]
    pub price: u64,
    /// item unique name;
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
    /// actual content (multi-part, multi-mime)
    #[prost(message, repeated, tag = "8")]
    pub media_item: ::prost::alloc::vec::Vec<MediaItem>,
    /// can be a reply to another item
    #[prost(uint64, tag = "9")]
    pub reply_to: u64,
    /// on all of the above data by the AUTHOR id keypair
    #[prost(message, optional, tag = "10")]
    pub signature: ::core::option::Option<Signature>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaItem {
    /// content unique id, for reference from other content
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// e.g image/gif, text/ut8, etc...
    #[prost(enumeration = "MimeType", tag = "3")]
    pub mime_type: i32,
    /// e.g. zlib/0.3.0.0 When set - content is compressed with the algo
    #[prost(enumeration = "CompressionCodec", tag = "4")]
    pub compression: i32,
    /// encoded content, may be compressed by stated compression algo
    #[prost(bytes = "vec", tag = "5")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceTermsBundle {
    /// Provider public id
    #[prost(message, optional, tag = "1")]
    pub provider_id: ::core::option::Option<EntityId>,
    /// generic service contract for new users
    #[prost(message, optional, tag = "2")]
    pub service_terms: ::core::option::Option<super::payments::ServiceTerms>,
    /// bundles are self contained and are signed
    #[prost(message, optional, tag = "3")]
    pub signature: ::core::option::Option<Signature>,
}
/// A set of pre-keys for an entity (provider or client)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityPreKeys {
    #[prost(message, optional, tag = "1")]
    pub entity_id: ::core::option::Option<EntityId>,
    #[prost(message, optional, tag = "2")]
    pub pre_key: ::core::option::Option<PublicKey>,
    #[prost(message, repeated, tag = "3")]
    pub one_time_keys: ::prost::alloc::vec::Vec<PublicKey>,
}
/// Supported media items' media types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MimeType {
    TextUtf8 = 0,
    ImagePng = 1,
    ImageGif = 2,
    ImageJpg = 3,
    VideoMp4 = 4,
    AudioAac = 5,
    AudioMp3 = 6,
    AudioM4a = 7,
}
/// A compression codec. Specifies the codec name and min required implementation version
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CompressionCodec {
    None = 0,
    Zlib030 = 1,
    Rar010 = 2,
}
////////////////////
//
// SNP - Identity Bundles
//
///////////////////////

/// Public service provider identity bundle.
/// See X2dh protocol for additional details.
/// Only include public data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderIdentityBundle {
    /// unique publishing time per provider
    #[prost(uint64, tag = "1")]
    pub time_stamp: u64,
    /// This is IKb in x2dh terms for Bob
    #[prost(message, optional, tag = "2")]
    pub provider_id: ::core::option::Option<EntityId>,
    /// Provider account's address for payments
    #[prost(message, optional, tag = "3")]
    pub address: ::core::option::Option<super::payments::Address>,
    /// node dial-up info
    #[prost(message, repeated, tag = "4")]
    pub dial_up_info: ::prost::alloc::vec::Vec<DialupInfo>,
    /// current x2dh pre-key (SPKb in x2dh protocol)
    #[prost(message, optional, tag = "5")]
    pub pre_key: ::core::option::Option<PreKey>,
    /// x2dh one-time keys (optional)
    #[prost(message, repeated, tag = "6")]
    pub one_time_keys: ::prost::alloc::vec::Vec<PreKey>,
    /// profile image
    #[prost(message, optional, tag = "7")]
    pub profile_image: ::core::option::Option<MediaItem>,
    /// provider current bond id on L1
    #[prost(uint64, tag = "8")]
    pub current_bond_id: u64,
    /// provider attests node id (node belongs to provider)
    #[prost(message, optional, tag = "10")]
    pub provider_signature: ::core::option::Option<Signature>,
    /// net-id of the SNP network that this identity is for
    #[prost(uint32, tag = "11")]
    pub net_id: u32,
}
/// Provider identity bundle with private data corresponding to the public data.
/// Private data includes private keys for public keys shared via the public bundle
/// todo: move this to upsetter package - this is a server implementation data object not an SNP type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateProviderIdentityBundle {
    #[prost(message, optional, tag = "1")]
    pub public_bundle: ::core::option::Option<ProviderIdentityBundle>,
    #[prost(message, optional, tag = "2")]
    pub provider_id_keypair: ::core::option::Option<KeyPair>,
    /// current x2dh x25519 pre-key private (SPKb in x2dh protocol)
    #[prost(message, optional, tag = "3")]
    pub pre_key: ::core::option::Option<PrivateKey>,
    /// x2dh one-time key-pairs (optional)
    #[prost(message, repeated, tag = "4")]
    pub one_time_keys_pairs: ::prost::alloc::vec::Vec<KeyPair>,
}
/// Client published bundle specifying current provider and x2dh pre-keys
/// Client represents a pseudo-anon identity that has its private key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientIdentityBundle {
    #[prost(uint64, tag = "1")]
    pub time_stamp: u64,
    /// cryptographic id - public key - ika...
    #[prost(message, optional, tag = "2")]
    pub client_id: ::core::option::Option<EntityId>,
    /// client current wallet address for payments
    #[prost(message, optional, tag = "3")]
    pub address: ::core::option::Option<super::payments::Address>,
    /// client's current provider
    #[prost(message, optional, tag = "4")]
    pub provider_bundle: ::core::option::Option<ProviderIdentityBundle>,
    /// client's current x2dh pre-key
    #[prost(message, optional, tag = "5")]
    pub pre_key: ::core::option::Option<PreKey>,
    /// x2dh one-time keys (optional)
    #[prost(message, repeated, tag = "6")]
    pub one_time_keys: ::prost::alloc::vec::Vec<PreKey>,
    /// profile data. e.g. profile image
    #[prost(message, optional, tag = "7")]
    pub profile_image: ::core::option::Option<MediaItem>,
    /// client signature on all other data fields
    #[prost(message, optional, tag = "8")]
    pub signature: ::core::option::Option<Signature>,
    /// net-id of the SNP network that this identity is for
    #[prost(uint32, tag = "9")]
    pub net_id: u32,
}
/// Provider client service data - not API specific - move to data objects package
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientServiceData {
    #[prost(uint64, tag = "1")]
    pub service_started: u64,
    #[prost(uint64, tag = "2")]
    pub service_ended: u64,
    #[prost(message, optional, tag = "3")]
    pub client_identity_bundle: ::core::option::Option<ClientIdentityBundle>,
}
/// Provider published client bundle - includes provider signature on the data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderSignedClientIdentityBundle {
    #[prost(message, optional, tag = "1")]
    pub client_bundle: ::core::option::Option<ClientIdentityBundle>,
    /// provider attests all data
    #[prost(message, optional, tag = "2")]
    pub signature: ::core::option::Option<Signature>,
}
/// Providers p2p protocol dialup info
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DialupInfo {
    /// endpoint type
    #[prost(enumeration = "ApiEndPoint", tag = "1")]
    pub end_point: i32,
    /// api semantic version
    #[prost(string, tag = "2")]
    pub api_version: ::prost::alloc::string::String,
    /// public server domain name or ip address
    #[prost(string, tag = "3")]
    pub ip_address: ::prost::alloc::string::String,
    /// endpoint port
    #[prost(uint32, tag = "4")]
    pub port: u32,
    /// SNP network id that this api is for
    #[prost(uint32, tag = "5")]
    pub net_id: u32,
    /// provider chosen name
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
}
/// Provider info includes public key and dialup info
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ProviderNetInfo {
    /// provider id
    #[prost(message, optional, tag = "1")]
    pub provider_id: ::core::option::Option<EntityId>,
    /// provider dialup info
    #[prost(message, optional, tag = "2")]
    pub dial_up_info: ::core::option::Option<DialupInfo>,
    /// data must be signed by provider
    #[prost(message, optional, tag = "3")]
    pub signature: ::core::option::Option<Signature>,
}
/// basic types

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ApiEndPoint {
    Unspecified = 0,
    /// grpc over web2 - connection not secure by TLS - for testing
    GrpcWeb2 = 1,
    /// grpc over web2 - connection secured by TLS - for production
    GrpcWeb2s = 2,
    /// grpc json gateway over http - insecure for testing
    JsonHttp = 3,
    /// grpc json gateway over https - for production
    JsonHttps = 4,
}
/////////////////////////
//
// SNP - Channels data
//

/// Request to subscribe to a channel or to a group.
/// Note that there's no need to sign as app-level protocol messages are always delivered as a signed TypedMessage
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSubscriptionRequestData {
    #[prost(uint64, tag = "1")]
    pub time_stamp: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub user: ::core::option::Option<EntityId>,
    /// short request message. e.g. Hi, this is foo - we talked on...
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
    /// optional - payment tx id
    #[prost(message, optional, tag = "5")]
    pub tx_id: ::core::option::Option<super::payments::TransactionId>,
    /// empty for status updates channel. // For groups - subscriber adds signed membership bundle used by group creator
    #[prost(message, optional, tag = "6")]
    pub membership: ::core::option::Option<GroupMemberBundle>,
}
/// A signed immutable bundle describing a status updates channel or a group channel.
/// Designed to be made available for subscribers who have obtained channel_id.
/// Signed by channel id and user to ensure that no-one else can create a channel with the same channel id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBundle {
    /// Channel public key id
    #[prost(message, optional, tag = "1")]
    pub channel_id: ::core::option::Option<EntityId>,
    /// Channel public key id
    #[prost(message, optional, tag = "2")]
    pub creator_id: ::core::option::Option<EntityId>,
    /// status feed or group
    #[prost(enumeration = "ChannelType", tag = "3")]
    pub channel_type: i32,
    /// channel creation time
    #[prost(uint64, tag = "4")]
    pub created: u64,
    /// channel desc
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// creator stated acceptable content and moderation policy
    #[prost(string, tag = "6")]
    pub acceptable_content_policy: ::prost::alloc::string::String,
    /// channel logo (optional)
    #[prost(message, optional, tag = "7")]
    pub logo: ::core::option::Option<MediaItem>,
    /// account-payable for channel related users fees (paid content, subs, etc...)
    #[prost(message, optional, tag = "8")]
    pub payable_address: ::core::option::Option<super::payments::Address>,
    /// Optional required monthly subscription fee for a status update channel
    #[prost(message, optional, tag = "9")]
    pub subscription_fee: ::core::option::Option<super::payments::Amount>,
    /// signature of channel_id on all above fields- proves user has private key to channel id
    #[prost(message, optional, tag = "10")]
    pub signature: ::core::option::Option<Signature>,
    /// channels' client signature on all other fields (including channel signature)
    #[prost(message, optional, tag = "11")]
    pub creator_signature: ::core::option::Option<Signature>,
    /// channel pricing model
    #[prost(enumeration = "PricingModel", tag = "12")]
    pub pricing_model: i32,
}
/// Private data stored on publisher's client with mutable channel's or group's state.
/// Channel can be public or private, status update channel or a group channel
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelData {
    /// current channel bundle and its data
    #[prost(message, optional, tag = "1")]
    pub bundle: ::core::option::Option<ChannelBundle>,
    /// when true, should be discoverable to anyone given client id
    #[prost(bool, tag = "2")]
    pub discoverable: bool,
    /// time of last content update
    #[prost(uint64, tag = "3")]
    pub last_updated: u64,
    /// Users blocked by publisher from replying to status updates or group messages.
    #[prost(message, repeated, tag = "5")]
    pub blocked_repliers: ::prost::alloc::vec::Vec<EntityId>,
    /// all channel content item (including replies by other non-blocked subscribers).
    #[prost(message, repeated, tag = "6")]
    pub content_items: ::prost::alloc::vec::Vec<ContentItem>,
    /// Note: this needs to be indexed by item id and by reply_to in order to support threaded discussions and time-based fetch.
    ///
    /// pending requests for subscription or membership
    #[prost(message, repeated, tag = "7")]
    pub sub_requests: ::prost::alloc::vec::Vec<ChannelSubscriptionRequestData>,
    /// subscribers for status updates
    #[prost(message, repeated, tag = "8")]
    pub subscribers: ::prost::alloc::vec::Vec<ChannelSubscriber>,
    /// members if channel is a group
    #[prost(message, optional, tag = "9")]
    pub group_members: ::core::option::Option<GroupMembersBundle>,
    /// channel id private key corresponding to channel_id so creator can sign with channel id.
    #[prost(bytes = "vec", tag = "10")]
    pub channel_key_pair: ::prost::alloc::vec::Vec<u8>,
}
/// Data maintained by channel subscriber (or group member) client
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelClientData {
    #[prost(message, repeated, tag = "1")]
    pub content_items: ::prost::alloc::vec::Vec<ChannelContentItem>,
}
/// A channel member
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSubscriber {
    #[prost(message, optional, tag = "1")]
    pub user_id: ::core::option::Option<EntityId>,
    #[prost(uint64, tag = "2")]
    pub date_subscribed: u64,
    /// add receipts here for past payments
    #[prost(uint64, tag = "3")]
    pub time_next_payment_due: u64,
}
/// A ChannelContent Item is a content item signed by the channel's creator
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelContentItem {
    #[prost(message, optional, tag = "1")]
    pub content_item: ::core::option::Option<ContentItem>,
    /// channel creator
    #[prost(message, optional, tag = "2")]
    pub signature: ::core::option::Option<Signature>,
}
////////////// Groups specific data

/// A group member bundle
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMemberBundle {
    #[prost(message, optional, tag = "1")]
    pub user_id: ::core::option::Option<EntityId>,
    #[prost(message, optional, tag = "2")]
    pub group_id: ::core::option::Option<EntityId>,
    /// user_id signature attesting it is a group member
    #[prost(message, optional, tag = "3")]
    pub signature: ::core::option::Option<Signature>,
}
/// Group members bundle is shared by group creator with group members and
/// is updated with membership changes
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMembersBundle {
    /// signature timestamp
    #[prost(uint64, tag = "1")]
    pub created: u64,
    /// group id (channel)
    #[prost(message, optional, tag = "2")]
    pub group_id: ::core::option::Option<EntityId>,
    /// channel's creator current client id
    #[prost(message, optional, tag = "3")]
    pub creator_id: ::core::option::Option<EntityId>,
    /// group members ids
    #[prost(message, repeated, tag = "4")]
    pub members: ::prost::alloc::vec::Vec<GroupMemberBundle>,
    /// signature of channel_id on all other fields (proves bundle author created this channel)
    #[prost(message, optional, tag = "5")]
    pub group_signature: ::core::option::Option<Signature>,
    /// channel's user's client signature on all other fields (proves identity of owner's client)
    #[prost(message, optional, tag = "6")]
    pub creator_signature: ::core::option::Option<Signature>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PricingModel {
    Free = 0,
    MonthlyFee = 1,
}
/// Supported types of channel
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChannelType {
    StatusFeed = 0,
    Group = 1,
}
