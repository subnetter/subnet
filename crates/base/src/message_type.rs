// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::snp::snp_server_api::MessageType;
use std::fmt;

/// Update this when adding a new MessageType
impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MessageType::ServiceTermsRequest => write!(f, "ServiceTerms request"),
            MessageType::ServiceTermsResponse => write!(f, "ServiceTerms response"),
            MessageType::ForwardMessageRequest => write!(f, "ForwardMessage request"),
            MessageType::ForwardMessageResponse => write!(f, "ForwardMessage response"),
            MessageType::StartServiceRequest => write!(f, "StartService request"),
            MessageType::StartServiceResponse => write!(f, "StartService response"),
            MessageType::StopServiceRequest => write!(f, "StopService request"),
            MessageType::StopServiceResponse => write!(f, "StopService response"),
            MessageType::TextMessageRequest => write!(f, "TextMessage request"),
            MessageType::TextMessageResponse => write!(f, "TextMessage response"),
            MessageType::RouteMessageRequest => write!(f, "RouteMessage request"),
            MessageType::RouteMessageResponse => write!(f, "RouteMessage request"),
            MessageType::SubscribeClientMessages => write!(f, "Subscribe to client messages"),
            MessageType::ChannelSubscribeRequest => write!(f, "Subscribe to channel or join group request"),
            MessageType::ChannelSubscribeResponse => write!(f, "Subscribe to channel response"),
            MessageType::ChannelUnsubscribeRequest => write!(f, "Unsubscribe from channel or leave group request"),
            MessageType::ChannelUnsubscribeResponse => write!(f, "Unsubscribe channel response"),
            MessageType::ChannelMessage => write!(f, "Channel message"),
            MessageType::ChannelMessageRequest => write!(
                f,
                "Request to post a reply to a status update or send a group message from a non-channel owner"
            ),
            MessageType::ChannelMessageResponse => write!(f, "Response from channel creator"),
            MessageType::BuyItemRequest => write!(f, "Buy a paid content item request"),
            MessageType::BuyItemResponse => write!(f, "Buy a paid content item response"),
            MessageType::ListPaidItemsRequest => write!(f, "List paid items available for sale request"),
            MessageType::ListPaidItemsResponse => write!(f, "Returns a list of paid items available for sale by client"),
            MessageType::ClientMessagesMetadata => write!(f, "A list of message's metadata that a provider has for a client"),
            MessageType::DeliverClientMessagesRequest => write!(f, "A request to deliver messages from a client to its provider"),
            MessageType::DeliverClientMessagesResponse => write!(f, "Provider response with client requested messages"),

            // Bundles discovery for clients
            MessageType::GetProviderBundleRequest => write!(f, "Get a provider identity bundle"),
            MessageType::GetProviderBundleResponse => write!(f, "Returns provider bundle if exists in store"),
            MessageType::GetClientBundleRequest  => write!(f, "Get a client provider-signed identity bundle "),
            MessageType::GetClientBundleResponse => write!(f, "Return client provider-signed bundle if exists in store"),

            // Client-provider data store (for encrypted backup of client owned-data)
            MessageType::StoreDataRequest => write!(f, "Store client data on the provider"),
            MessageType::StoreDataResponse => write!(f, "Response to store client data request"),
            MessageType::ReadDataRequest => write!(f, "Read previously provider stored client data"),
            MessageType::ReadDataResponse => write!(f, "Return previously provider stored client data"),

            MessageType::PingNodeRequest => write!(f, "A ping request for node to return its net info"),
            MessageType::PingNodeResponse => write!(f, "A ping response, includes signed node net info"),

        }
    }
}
