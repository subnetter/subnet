// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::channels::channels_data_service::{ChannelsService, UpsertChannel};
use crate::simple_client::SimpleClient;
use anyhow::Result;
use base::hex_utils::short_hex_string;
use base::snp::snp_core_types::{ChannelData, ChannelSubscriber, EntityId};
use chrono::prelude::*;
use xactor::Service;

impl SimpleClient {
    /// Subscribe a user to a status update channel created by this client
    pub(crate) async fn subscribe_to_status_update(
        &self,
        user: &EntityId,
        channel_data: &mut ChannelData,
    ) -> Result<()> {
        let subscriber = ChannelSubscriber {
            user_id: Some(user.clone()),
            date_subscribed: Utc::now().timestamp_nanos() as u64,
            time_next_payment_due: 0,
        };

        let subscriber_id = user.get_id()?;

        if let Some(_sub) = channel_data
            .subscribers
            .iter()
            .position(|sub| sub.get_subscriber_id().unwrap() == subscriber_id)
        {
            warn!("user is already a subscriber to this channel");
            return Ok(());
        };

        channel_data.subscribers.push(subscriber);

        let channels_service = ChannelsService::from_registry().await?;
        let _ = channels_service
            .call(UpsertChannel(channel_data.clone()))
            .await??;

        debug!(
            "subscribed user {:?} to status update channel",
            short_hex_string(subscriber_id.as_slice())
        );

        Ok(())
    }
}
