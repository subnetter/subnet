// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, Result};
use base::hex_utils::short_hex_string;
use base::snp::snp_client_to_client::CancelChannelSubscriptionResponse;
use base::snp::snp_server_api::TypedMessage;

impl SimpleClient {
    pub(crate) async fn handle_unsubscribe_response_message(
        &mut self,
        msg: TypedMessage,
    ) -> Result<()> {
        use prost::Message;
        let response: CancelChannelSubscriptionResponse =
            CancelChannelSubscriptionResponse::decode(msg.message.as_slice())
                .map_err(|e| anyhow!("failed to decode message {:?}", e))?;

        info!(
            "Got confirmation for channel unsubscribe request. Channel id: {}",
            short_hex_string(&*response.channel_id)
        );
        Ok(())
    }
}
