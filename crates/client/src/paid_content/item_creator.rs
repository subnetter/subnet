// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::Result;
use base::api_types_extensions::Signed;
use base::snp::snp_core_types::{CompressionCodec, ContentItem, MediaItem, MimeType};
use base::snp::upsetter_simple_client::UserCreatePaidItemRequest;
use chrono::prelude::*;
use rand_core::{OsRng, RngCore};
use xactor::*;

#[message(result = "Result<u64>")]
pub(crate) struct CreatePaidItem(pub(crate) UserCreatePaidItemRequest);

/// Request to subscribe this client to another user status update channel
/// or to join a group for a group channel on behalf of this client's user.
#[async_trait::async_trait]
impl Handler<CreatePaidItem> for SimpleClient {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: CreatePaidItem) -> Result<u64> {
        self.create_new_paid_item(msg.0).await
    }
}

impl SimpleClient {
    /// Create a new paid content item from user provided data and store it locally
    async fn create_new_paid_item(&mut self, req: UserCreatePaidItemRequest) -> Result<u64> {
        let update_item = MediaItem {
            id: 0,
            name: req.name,
            mime_type: MimeType::TextUtf8 as i32,
            compression: CompressionCodec::None as i32,
            content: req.content.into_bytes(),
        };

        let id = OsRng.next_u64();

        let mut item = ContentItem {
            id,
            created: Utc::now().timestamp_nanos() as u64,
            channel_id: vec![],
            author: Some(self.get_client_entity()?),
            ttl: 0,
            price: req.price,
            name: "".into(),
            media_item: vec![update_item],
            reply_to: 0,
            signature: None,
        };

        item.sign(&self.client_id)?;
        self.paid_items.insert(id, item);
        Ok(id)
    }
}
