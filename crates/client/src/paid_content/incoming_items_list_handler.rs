// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, Result};
use base::hex_utils::short_hex_string;
use base::snp::snp_client_to_client::ListPaidItemsResponse;
use base::snp::snp_server_api::TypedMessage;
use prost::Message;

impl SimpleClient {
    /// New content item from another client
    pub(crate) async fn handle_incoming_items_list(&self, msg: TypedMessage) -> Result<()> {
        let sender_id = msg.get_ika()?;
        let resp: ListPaidItemsResponse = ListPaidItemsResponse::decode(msg.message.as_slice())
            .map_err(|e| anyhow!("failed to decode response {:?}", e))?;

        info!(
            "🎉 👋 incoming items list from {}",
            short_hex_string(sender_id.as_ref()),
        );

        for item in resp.content_items {
            info!("Id: {}. Price: {}", item.id, item.price)
        }

        Ok(())
    }
}
