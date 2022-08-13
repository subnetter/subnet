// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, bail, Result};
use base::snp::snp_client_to_client::ListPaidItemsRequest;
use base::snp::snp_core_types::EntityId;
use base::snp::snp_server_api::MessageType;
use bytes::Bytes;
use xactor::*;

/// Purchase a paid content item created by another client
#[message(result = "Result<()>")]
pub(crate) struct ListItems {
    pub(crate) seller_id: EntityId,
}

/// Handle a user request to list paid content items for sale by another user
/// Precondition: this client must be provided by a provider. e.g. SetProvider was called in this client app session.
#[async_trait::async_trait]
impl Handler<ListItems> for SimpleClient {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: ListItems) -> Result<()> {
        if self.provider_bundle.is_none() {
            bail!("missing provider bundle")
        }

        let key = msg.seller_id.get_id()?.clone();
        let seller_id = Bytes::from(key.clone());
        let sb_bundle = self
            .other_clients
            .get(&key)
            .ok_or_else(|| anyhow!("missing bundle"))?
            .clone();

        let seller_bundle = sb_bundle.client_bundle.as_ref().unwrap();
        let seller_pub_key = seller_bundle.get_client_id_ed25519_public_key()?;

        let list_items_req = ListPaidItemsRequest {};
        use prost::Message;
        let mut buff = Vec::with_capacity(list_items_req.encoded_len());
        list_items_req.encode(&mut buff).unwrap();
        let typed_msg =
            self.create_typed_message(MessageType::ListPaidItemsRequest, buff, seller_pub_key)?;
        self.send_typed_message(typed_msg, seller_id).await?;

        Ok(())
    }
}
