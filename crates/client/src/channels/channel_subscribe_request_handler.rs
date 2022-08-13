// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::channels::channels_data_service::{ChannelsService, GetChannel};
use crate::simple_client::SimpleClient;
use anyhow::{anyhow, bail, Result};
use base::snp::snp_client_to_client::{ChannelSubscriptionRequest, ChannelSubscriptionResponse};
use base::snp::snp_core_types::{ChannelData, ChannelType};
use base::snp::snp_server_api::{MessageType, TypedMessage};
use bytes::Bytes;
use xactor::Service;

impl SimpleClient {
    /// Handles a remote request from another client to subscribe to one of this client's status update channels
    /// or to join a group channel
    pub(crate) async fn handle_subscribe_to_channel_message(
        &mut self,
        msg: TypedMessage,
    ) -> Result<()> {
        //
        // Currently, accept any request from other client to subscribe to status updates or to join a group
        // In a real product, this client's user will need to review and approve the request or set it to auto approve (public channel)
        // In addition, for paid channels, client needs to verify the receipt from the request before
        // adding the requesting client as a subscriber
        //
        use prost::Message;
        let request: ChannelSubscriptionRequest =
            ChannelSubscriptionRequest::decode(msg.message.as_slice())
                .map_err(|e| anyhow!("failed to decode message {:?}", e))?;

        let request_data = request
            .subscription_request_data
            .ok_or_else(|| anyhow!("missing request data"))?;

        let channels_service = ChannelsService::from_registry().await?;
        let mut channel_data: ChannelData = channels_service
            .call(GetChannel(request_data.channel_id))
            .await??
            .ok_or_else(|| anyhow!("unknown channel"))?;

        let subscriber = request_data
            .user
            .ok_or_else(|| anyhow!("missing subscriber id"))?;

        let subscriber_id = Bytes::from(subscriber.get_id()?.clone());

        let channel_bundle = channel_data
            .bundle
            .as_mut()
            .ok_or_else(|| anyhow!("missing channel bundle"))?;

        match channel_bundle.channel_type {
            t if t == ChannelType::StatusFeed as i32 => {
                self.subscribe_to_status_update(&subscriber, &mut channel_data)
                    .await?
            }
            t if t == ChannelType::Group as i32 => {
                let membership = request_data
                    .membership
                    .ok_or_else(|| anyhow!("missing membership data in channel's data"))?;

                self.add_group_member(&subscriber, &mut channel_data, membership)
                    .await?
            }
            _ => bail!("unrecognized channel type"),
        }

        // Send subscription confirmation message the to subscriber
        let resp_msg = ChannelSubscriptionResponse {
            channel_id: channel_data.get_channel_id()?,
            subscribed: true,
            message: "Welcome aboard!".into(),
        };
        let mut buff = Vec::with_capacity(resp_msg.encoded_len());
        resp_msg.encode(&mut buff).unwrap();
        let receiver = subscriber.get_ed_pub_key()?;
        let typed_msg =
            self.create_typed_message(MessageType::ChannelSubscribeResponse, buff, receiver)?;

        debug!("sending subscription resp to other client via our provider...");

        self.send_typed_message(typed_msg, subscriber_id).await?;

        Ok(())
    }
}
