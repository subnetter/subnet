// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::playground::Playground;
use anyhow::{anyhow, Result};
use base::snp::upsetter_simple_client::UserBuyPaidItemRequest;

impl Playground {
    pub(crate) async fn buy_item(
        &mut self,
        client_name: &str,
        seller: &str,
        item_id: u64,
        price: u64,
    ) -> Result<()> {
        let client = self.clients.get_mut(client_name);
        if client.is_none() {
            return Err(anyhow!("unknown client"));
        }
        let seller_client = self.clients_bundles.get(seller);
        if seller_client.is_none() {
            return Err(anyhow!("unknown receiver client"));
        }
        let seller_entity = seller_client.unwrap().get_client_entity()?;
        match client
            .unwrap()
            .user_buy_paid_item(UserBuyPaidItemRequest {
                seller_client_id: Some(seller_entity),
                item_id,
                price,
            })
            .await
        {
            Ok(_resp) => Ok(()),
            Err(e) => Err(anyhow!("failed to send buy item message: {}", e)),
        }
    }
}
