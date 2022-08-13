// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.

use crate::playground::Playground;
use anyhow::{anyhow, Result};
use base::snp::upsetter_simple_client::UserCreatePaidItemRequest;

impl Playground {
    pub(crate) async fn create_paid_item(
        &mut self,
        client_name: &str,
        price: u64,
        content: String,
        name: String,
    ) -> Result<()> {
        let client = self.clients.get_mut(client_name);
        if client.is_none() {
            return Err(anyhow!("unknown client"));
        }

        match client
            .unwrap()
            .user_create_paid_item(UserCreatePaidItemRequest {
                price,
                name: name.clone(),
                content,
            })
            .await
        {
            Ok(resp) => {
                let item_id = resp.into_inner().item_id;
                println!(
                    "🖖 Paid item created. item name: {}, id: {}, Price: {}",
                    name, item_id, price
                );
                Ok(())
            }
            Err(e) => Err(anyhow!("failed to create paid item: {}", e)),
        }
    }
}
