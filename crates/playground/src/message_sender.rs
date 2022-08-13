// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::playground::Playground;
use anyhow::{anyhow, Result};
use base::snp::upsetter_simple_client::UserSendTextMessageRequest;

impl Playground {
    pub(crate) async fn send_message(
        &mut self,
        client_name: &str,
        to: &str,
        text: String,
        reply_to: u64,
    ) -> Result<()> {
        let client = self.clients.get_mut(client_name);
        if client.is_none() {
            return Err(anyhow!("unknown client"));
        }
        let other_client = self.clients_bundles.get(to);
        if other_client.is_none() {
            return Err(anyhow!("unknown receiver client"));
        }
        let other_entity = other_client.unwrap().get_client_entity()?;
        match client
            .unwrap()
            .user_send_text_message(UserSendTextMessageRequest {
                other_client_id: Some(other_entity),
                user_text: text,
                reply_to,
            })
            .await
        {
            Ok(_resp) => {
                // println!("🖖 message sent. Id: {}", resp.into_inner().message_id);
                Ok(())
            }
            Err(e) => Err(anyhow!("failed to send message: {}", e)),
        }
    }
}
