// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::Result;
use base::api_types_extensions::Signed;
use base::snp::snp_core_types::{EntityId, PublicKey};
use base::snp::snp_server_api::{MessageType, TypedMessage};
use chrono::prelude::*;

impl SimpleClient {
    /// Create a new typed message from this client using msg data and type
    /// Sign the message by this client, and sets its sender, receiver and timestamp
    pub(crate) fn create_typed_message(
        &self,
        msg_type: MessageType,
        msg_data: Vec<u8>,
        receiver: ed25519_dalek::PublicKey,
    ) -> Result<TypedMessage> {
        let ika = self.client_id.public;

        let a_id_pub_key = PublicKey {
            key: ika.as_ref().to_vec(),
        };

        let alice_entity = EntityId {
            public_key: Some(a_id_pub_key),
            nickname: "".to_string(),
        };

        let b_pub_key = PublicKey {
            key: receiver.to_bytes().to_vec(),
        };

        let b_entity = EntityId {
            public_key: Some(b_pub_key),
            nickname: "".to_string(),
        };

        let mut typed_msg = TypedMessage {
            time_stamp: Utc::now().timestamp_nanos() as u64,
            msg_type: msg_type as i32,
            message: msg_data,
            receiver: Some(b_entity),
            sender: Some(alice_entity),
            signature: None,
        };

        typed_msg.sign(&self.client_id)?;

        Ok(typed_msg)
    }
}
