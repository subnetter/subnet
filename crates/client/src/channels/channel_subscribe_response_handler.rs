// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, Result};
use base::hex_utils::short_hex_string;
use base::snp::snp_client_to_client::ChannelSubscriptionResponse;
use base::snp::snp_server_api::TypedMessage;

impl SimpleClient {
    pub(crate) async fn handle_subscribe_response_message(
        &mut self,
        msg: TypedMessage,
    ) -> Result<()> {
        use prost::Message;
        let response: ChannelSubscriptionResponse =
            ChannelSubscriptionResponse::decode(msg.message.as_slice())
                .map_err(|e| anyhow!("failed to decode message {:?}", e))?;

        info!(
            "Got channel subscription confirmation. Channel id: {}. Message: {}",
            short_hex_string(&*response.channel_id),
            response.message
        );

        let key: &[u8] = response.channel_id.as_ref();
        if let Some(channel_data) = self.channels_subscriptions_requests.get(key) {
            info!("subscribed to channel");
            self.channels_subscriptions
                .insert(key.to_vec(), channel_data.clone());
            self.channels_subscriptions_requests.remove(key);
        } else {
            warn!("did not find a request to subscribe to this channel by this client")
        }

        Ok(())
    }
}
