//////////////
//
// SNP - Channels client-to-client network protocol
// Messages requests and responses from a client to a client related to channels and content
//
/////////////

/// Request to subscribe to a channel or to join a group
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSubscriptionRequest {
    #[prost(message, optional, tag = "1")]
    pub subscription_request_data:
        ::core::option::Option<super::core_types::ChannelSubscriptionRequestData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSubscriptionResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    /// confirmation
    #[prost(bool, tag = "2")]
    pub subscribed: bool,
    /// rejection message or welcome message
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
/////////////

/// Request to unsubscribe from a status update channel or to leave a group
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelChannelSubscription {
    #[prost(uint64, tag = "1")]
    pub time_stamp: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub user: ::core::option::Option<super::core_types::EntityId>,
    /// user can provide reason
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelChannelSubscriptionResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
}
/////////////

/// New channel message
/// Sent from channel creator to all status updates subscribers (or for a group, to group members)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewChannelMessage {
    #[prost(message, optional, tag = "1")]
    pub content_item: ::core::option::Option<super::core_types::ChannelContentItem>,
}
///////////////

/// A request from a client to post a message to a channel. Sent from author to channel owner.
/// Message is a reply to a status message or a group message in a group that client is member of.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewChannelMessageRequest {
    #[prost(message, optional, tag = "1")]
    pub content_item: ::core::option::Option<super::core_types::ContentItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewChannelMessageResponse {}
//////////////
//
// SNP - Paid Items client-to-client network protocol
// Messages requests and responses from client to client regarding paid content items
//
/////////////

/// Request to list all current content items for sale by a client
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPaidItemsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPaidItemsResponse {
    #[prost(message, repeated, tag = "1")]
    pub content_items: ::prost::alloc::vec::Vec<super::core_types::ContentItem>,
}
/// Request to return meta data about a content items
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetItemDataRequest {
    #[prost(uint64, tag = "1")]
    pub item_id: u64,
}
/// Response - a content item with an empty MediaItems is returned with all content meta-data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetItemDataResponse {
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<super::core_types::ContentItem>,
}
/////////////

/// Request to unsubscribe from a status update channel or to leave a group
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyItemRequest {
    #[prost(uint64, tag = "1")]
    pub item_id: u64,
    #[prost(message, optional, tag = "2")]
    pub transaction_id: ::core::option::Option<super::payments::TransactionId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyItemResponse {
    #[prost(enumeration = "BuyItemResult", tag = "1")]
    pub result: i32,
    #[prost(uint64, tag = "2")]
    pub receipt_id: u64,
    #[prost(message, optional, tag = "3")]
    pub item: ::core::option::Option<super::core_types::ContentItem>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BuyItemResult {
    InvalidTransaction = 0,
    ItemNotFound = 1,
    Success = 2,
}
