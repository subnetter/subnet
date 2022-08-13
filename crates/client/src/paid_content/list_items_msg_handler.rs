// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, Result};
use base::hex_utils::short_hex_string;
use base::snp::snp_client_to_client::{ListPaidItemsRequest, ListPaidItemsResponse};
use base::snp::snp_core_types::ContentItem;
use base::snp::snp_server_api::{MessageType, TypedMessage};
use bytes::Bytes;

impl SimpleClient {
    /// Handle a list items request from another client
    pub(crate) async fn handle_list_items_request(&mut self, msg: TypedMessage) -> Result<()> {
        let sender_id = msg.get_ika()?;
        let _req: ListPaidItemsRequest = ListPaidItemsRequest::decode(msg.message.as_slice())
            .map_err(|e| anyhow!("failed to decode list items message {:?}", e))?;

        info!(
            "🎉 👋 List items request from {}.\n",
            short_hex_string(sender_id.as_ref()),
        );

        let mut items: Vec<ContentItem> = Vec::new();
        for item in self.paid_items.values() {
            items.push(item.clone())
        }

        let resp_msg = ListPaidItemsResponse {
            content_items: items,
        };

        use prost::Message;
        let mut buff = Vec::with_capacity(resp_msg.encoded_len());
        resp_msg.encode(&mut buff).unwrap();

        let typed_msg =
            self.create_typed_message(MessageType::ListPaidItemsResponse, buff, sender_id)?;
        let receiver_id = Bytes::from(sender_id.to_bytes().to_vec());
        self.send_typed_message(typed_msg, receiver_id).await?;

        Ok(())
    }
}
