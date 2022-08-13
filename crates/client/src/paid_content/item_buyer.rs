// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, bail, Result};
use base::snp::snp_client_to_client::BuyItemRequest;
use base::snp::snp_core_types::EntityId;
use base::snp::snp_server_api::MessageType;
use bytes::Bytes;
use xactor::*;

/// Buy a paid content item created by another client
#[message(result = "Result<()>")]
pub struct BuyItem {
    pub seller_id: EntityId,
    pub item_id: u64,
    pub price: u64,
}

/// Handles a user request to buy a paid content item for sale by another user
/// Precondition: this client must be provided by a provider. e.g. SetProvider was called in this client app session.
#[async_trait::async_trait]
impl Handler<BuyItem> for SimpleClient {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: BuyItem) -> Result<()> {
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

        // we fake a payment and create a receipt for now
        // todo: make an L2 payment here

        let _seller_wallet_address = seller_bundle
            .address
            .as_ref()
            .ok_or_else(|| anyhow!("missing seller current wallet address"))?;

        let _my_wallet_address = self
            .client_bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing client bundle"))?
            .address
            .as_ref()
            .ok_or_else(|| anyhow!("missing address from bundle"))?;

        // TODO: create transaction and attach it to message

        /*
        let receipt = Receipt {
            id: OsRng.next_u64(),
            time_stamp: Instant::now().seconds() as u64,
            from: Some(my_wallet_address.clone()), // client's wallet address with sufficient L2 balance
            to: Some(seller_wallet_address.clone()), // this needs to be the provider's wallet address as provided in its identity bundle
            amount: Some(Amount {
                value: msg.price,
                coin_type: CoinType::Stable as i32,
            }),
            signature: None,
            item_ids: vec![msg.item_id],
        };*/

        let buy_item_message = BuyItemRequest {
            item_id: msg.item_id,
            transaction_id: None,
        };

        use prost::Message;
        let mut buff = Vec::with_capacity(buy_item_message.encoded_len());
        buy_item_message.encode(&mut buff).unwrap();
        let typed_msg =
            self.create_typed_message(MessageType::BuyItemRequest, buff, seller_pub_key)?;
        self.send_typed_message(typed_msg, seller_id).await?;

        Ok(())
    }
}
