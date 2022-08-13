// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, Result};
use base::hex_utils::short_hex_string;
use base::snp::snp_core_types::ContentItem;
use base::snp::snp_server_api::TypedMessage;
use prost::Message;

impl SimpleClient {
    /// New text message from another client
    pub(crate) async fn handle_text_message(&self, msg: TypedMessage) -> Result<()> {
        let sender_id = msg.get_ika()?;
        let text_message: ContentItem = ContentItem::decode(msg.message.as_slice())
            .map_err(|e| anyhow!("failed to decode text message {:?}", e))?;

        info!(
            "🎉 👋 incoming text message from {}: {}\tmessage id: {} reply to: {}",
            short_hex_string(sender_id.as_ref()),
            text_message.get_simple_text_content()?,
            text_message.id,
            text_message.reply_to
        );

        Ok(())
    }
}
