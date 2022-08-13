// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, Result};
use base::hex_utils::short_hex_string;
use base::snp::snp_client_to_client::BuyItemResponse;
use base::snp::snp_server_api::TypedMessage;
use prost::Message;

impl SimpleClient {
    /// New content item from another client
    pub(crate) async fn handle_incoming_item(&self, msg: TypedMessage) -> Result<()> {
        let sender_id = msg.get_ika()?;
        let item_resp: BuyItemResponse = BuyItemResponse::decode(msg.message.as_slice())
            .map_err(|e| anyhow!("failed to decode response {:?}", e))?;

        let item = item_resp
            .item
            .ok_or_else(|| anyhow!("missing item from response"))?;

        info!(
            "🎉 👋 incoming paid item from {}: {}.\nItem id: {}",
            short_hex_string(sender_id.as_ref()),
            item.get_simple_text_content()?,
            item.id,
        );

        Ok(())
    }
}
