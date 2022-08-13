// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, bail, Result};
use bytes::Bytes;
use xactor::*;

/// Send a text message to another client
#[message(result = "Result<u64>")]
pub struct SendTextMessage {
    pub message: String,
    pub receiver_id: Bytes,
    pub reply_to: u64,
}

/// Handle a user api request to send a 1:1 text message to another client.
/// Preconditions: (i) this client is provided by a provider. e.g. SetProvider was called in this client app session.
#[async_trait::async_trait]
impl Handler<SendTextMessage> for SimpleClient {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SendTextMessage) -> Result<u64> {
        if self.provider_bundle.is_none() {
            bail!("missing provider bundle")
        }

        let key = msg.receiver_id.as_ref();

        let sb_bundle = self
            .other_clients
            .get(key)
            .ok_or_else(|| anyhow!("missing bundle"))?
            .clone();

        let b_bundle = sb_bundle.client_bundle.as_ref().unwrap();
        let ikb = b_bundle.get_client_id_ed25519_public_key()?;

        // The message Alice (A's user) sends Bob (B's user)
        let (text_message, message_id) = self
            .new_text_message(msg.message, ikb, msg.reply_to)
            .await?;
        self.send_typed_message(text_message, msg.receiver_id)
            .await?;

        Ok(message_id)
    }
}
