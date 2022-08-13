// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, Result};
use base::hex_utils::short_hex_string;
use base::snp::snp_client_to_client::{BuyItemRequest, BuyItemResponse, BuyItemResult};
use base::snp::snp_server_api::{MessageType, TypedMessage};
use bytes::Bytes;

impl SimpleClient {
    /// Handle a buy item request from another client
    pub(crate) async fn handle_buy_item_request(&mut self, msg: TypedMessage) -> Result<()> {
        let sender_id = msg.get_ika()?;
        let req: BuyItemRequest = BuyItemRequest::decode(msg.message.as_slice())
            .map_err(|e| anyhow!("failed to decode buy item message {:?}", e))?;

        info!(
            "🎉 👋 Buy item request from {}.\nItem id: {}",
            short_hex_string(sender_id.as_ref()),
            req.item_id,
        );

        let resp_msg = BuyItemResponse {
            result: BuyItemResult::Success as i32,
            receipt_id: 0,
            item: None,
        };

        // todo: verify the transaction...
        /*
        match req.receipt {
            Some(receipt) => {
                // todo: fully verify the L2 receipt
                if let Some(item) = self.paid_items.get(&req.item_id) {
                    resp_msg.item = Some(item.clone());
                    resp_msg.receipt_id = receipt.id;
                } else {
                    resp_msg.result = BuyItemResult::ItemNotFound as i32;
                }
            }
            None => {
                resp_msg.result = BuyItemResult::InvalidReceipt as i32;
            }
        }*/

        use prost::Message;
        let mut buff = Vec::with_capacity(resp_msg.encoded_len());
        resp_msg.encode(&mut buff).unwrap();

        let typed_msg = self.create_typed_message(MessageType::BuyItemResponse, buff, sender_id)?;
        let receiver_id = Bytes::from(sender_id.to_bytes().to_vec());
        self.send_typed_message(typed_msg, receiver_id).await?;

        Ok(())
    }
}
