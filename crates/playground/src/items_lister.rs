// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::playground::Playground;
use anyhow::{anyhow, Result};
use base::snp::upsetter_simple_client::UserListPaidContentItemsRequest;

impl Playground {
    pub(crate) async fn list_paid_items(&mut self, client_name: &str, seller: &str) -> Result<()> {
        let client = self.clients.get_mut(client_name);
        if client.is_none() {
            return Err(anyhow!("unknown client"));
        }
        let seller_client = self.clients_bundles.get(seller);
        if seller_client.is_none() {
            return Err(anyhow!("unknown seller client"));
        }
        let seller_entity = seller_client.unwrap().get_client_entity()?;
        match client
            .unwrap()
            .user_list_paid_content_items(UserListPaidContentItemsRequest {
                seller_client_id: Some(seller_entity),
            })
            .await
        {
            Ok(_resp) => Ok(()),
            Err(e) => Err(anyhow!("failed to send list items message: {}", e)),
        }
    }
}
