// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, bail, Result};
use base::api_types_extensions::Signed;
use base::hex_utils::short_hex_string;
use base::snp::snp_client_to_client::ChannelSubscriptionRequest;
use base::snp::snp_core_types::{
    ChannelBundle, ChannelSubscriptionRequestData, ChannelType, GroupMemberBundle,
};
use base::snp::snp_server_api::MessageType;
use bytes::Bytes;
use chrono::prelude::*;
use xactor::*;

#[message(result = "Result<()>")]
pub(crate) struct SubscribeToChannel {
    pub(crate) channel: ChannelBundle,
}

/// Request to subscribe this client to another user status update channel
/// or to join a group for a group channel on behalf of this client's user.
#[async_trait::async_trait]
impl Handler<SubscribeToChannel> for SimpleClient {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SubscribeToChannel) -> Result<()> {
        let channel_type = msg.channel.channel_type;

        if channel_type != ChannelType::StatusFeed as i32
            && channel_type != ChannelType::Group as i32
        {
            bail!("unexpected channel type: {}", channel_type)
        }

        let channel_id = msg
            .channel
            .channel_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing channel id"))?;

        let my_entity = self.get_client_entity()?;

        let channel_creator = msg
            .channel
            .creator_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing creator identity"))?;

        let creator_pub_key = channel_creator
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing key"))?
            .as_pub_key()?;

        debug!(
            "Channel creator: {:?}",
            short_hex_string(creator_pub_key.as_ref())
        );

        let message = match channel_type {
            t if t == ChannelType::StatusFeed as i32 => {
                "Hi, I'd like to subscribe to your status updates"
            }
            t if t == ChannelType::Group as i32 => "Hi, I'd like to join your group",
            _ => bail!("unexpected channel type"),
        };

        let membership = match channel_type {
            t if t == ChannelType::StatusFeed as i32 => None,

            t if t == ChannelType::Group as i32 => {
                let mut bundle = GroupMemberBundle {
                    user_id: Some(my_entity.clone()),
                    group_id: Some(channel_id.clone()),
                    signature: None,
                };
                bundle.sign(&self.client_id)?;
                Some(bundle)
            }
            _ => bail!("unexpected channel type"),
        };

        let channel_id_bytes = channel_id.get_id()?.to_vec();

        let subscribe_request = ChannelSubscriptionRequest {
            subscription_request_data: Some(ChannelSubscriptionRequestData {
                time_stamp: Utc::now().timestamp_nanos() as u64,
                channel_id: channel_id_bytes.clone(),
                user: Some(my_entity),
                message: message.into(),
                membership,
                tx_id: None,
            }),
        };

        // store this in pending requests store
        self.channels_subscriptions_requests
            .insert(channel_id_bytes, msg.channel);

        use prost::Message;
        let mut buff = Vec::with_capacity(subscribe_request.encoded_len());
        subscribe_request.encode(&mut buff).unwrap();

        let typed_msg =
            self.create_typed_message(MessageType::ChannelSubscribeRequest, buff, creator_pub_key)?;

        let receiver_id = Bytes::from(creator_pub_key.to_bytes().to_vec());
        debug!("sending subscription request to other client via our provider...");
        self.send_typed_message(typed_msg, receiver_id).await?;

        debug!("added channel to channel requests store");

        Ok(())
    }
}
