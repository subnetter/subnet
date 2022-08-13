// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::Result;
use base::api_types_extensions::Signed;
use base::snp::snp_core_types::ContentItem;
use base::snp::snp_server_api::{MessageType, TypedMessage};
use crypto::utils::entity_from_ed25519_pub_key;

/// Implementation of creation of a new text message with content designated to an entity
impl SimpleClient {
    /// Create a new text message to a user and sign it using sender_keys
    /// Returns a signed self-describing typed message with the text message as its content
    pub(crate) async fn new_text_message(
        &self,
        content: String,
        to: ed25519_dalek::PublicKey,
        reply_to: u64,
    ) -> Result<(TypedMessage, u64)> {
        let my_entity = entity_from_ed25519_pub_key(&self.client_id.public, "".into());
        let mut inner_msg = ContentItem::new_one_to_one_text_message(content, my_entity, reply_to);
        inner_msg.sign(&self.client_id)?;

        use prost::Message;
        let mut buff = Vec::with_capacity(inner_msg.encoded_len());
        inner_msg.encode(&mut buff).unwrap();

        // this is the message alice sends to bob
        let typed_msg = self.create_typed_message(MessageType::TextMessageRequest, buff, to)?;

        Ok((typed_msg, inner_msg.id))
    }
}
