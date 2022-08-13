// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::api_types_extensions::{Signed, SignedWithExternalVerifier};
use crate::snp::snp_core_types::{
    ChannelContentItem, CompressionCodec, ContentItem, EntityId, MediaItem, MimeType,
};
use anyhow::{anyhow, bail, Result};
use chrono::Utc;
use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::{Signer, Verifier};
use rand_core::{OsRng, RngCore};

impl ContentItem {
    // Get text content from item assuming first media item is the text message
    pub fn get_simple_text_content(&self) -> Result<String> {
        if self.media_item.is_empty() {
            bail!("missing content")
        }

        Ok(String::from_utf8(self.media_item[0].content.clone())?)
    }

    /// Creates a new simple 1:1 text message w/o a channel
    pub fn new_one_to_one_text_message(
        text: String,
        author: EntityId,
        reply_to: u64,
    ) -> ContentItem {
        let update_item = MediaItem {
            id: 0,
            name: "".into(),
            mime_type: MimeType::TextUtf8 as i32,
            compression: CompressionCodec::None as i32,
            content: text.into_bytes(),
        };

        ContentItem {
            id: OsRng.next_u64(),
            created: Utc::now().timestamp_nanos() as u64,
            channel_id: vec![],
            author: Some(author),
            ttl: 0,
            price: 0,
            name: "".to_string(),
            media_item: vec![update_item],
            signature: None,
            reply_to,
        }
    }

    /// Creates a new simple text status update
    pub fn new_channel_text_message(
        text: String,
        author: EntityId,
        channel_id: Vec<u8>,
        reply_to: u64,
    ) -> ContentItem {
        let mut item = ContentItem::new_one_to_one_text_message(text, author, reply_to);
        item.channel_id = channel_id;
        item
    }

    /// Creates a new simple text reply to a status update
    pub fn new_text_status_update_reply(_text: String) -> ContentItem {
        unimplemented!()
    }

    /// Creates a new simple text group message
    pub fn new_text_group_message(_text: String) -> ContentItem {
        unimplemented!()
    }

    /// Creates a new simple text group message reply
    pub fn new_text_group_reply_message(_text: String) -> ContentItem {
        unimplemented!()
    }
}

impl Signed for ContentItem {
    /// Sign content by author
    fn sign(&mut self, author_key_pair: &ed25519_dalek::Keypair) -> Result<()> {
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;

        use crate::snp::snp_core_types::Signature;
        self.signature = Some(Signature {
            scheme_id: 0,
            signature: author_key_pair.sign(&buf).as_ref().to_vec(),
        });

        Ok(())
    }

    fn verify_signature(&self) -> Result<()> {
        let signature = self
            .signature
            .as_ref()
            .ok_or_else(|| anyhow!("missing author signature"))?
            .clone();

        let mut data = self.clone();
        data.signature = None;

        use prost::Message;
        let mut buf = Vec::with_capacity(data.encoded_len());
        if data.encode(&mut buf).is_err() {
            return Err(anyhow!("failed to encode source data to binary data"));
        };

        let signature = ed25519_dalek::Signature::from_bytes(signature.signature.as_slice())?;

        let author = self
            .author
            .as_ref()
            .ok_or_else(|| anyhow!("missing author"))?;

        let author_pub_key = ed25519_dalek::PublicKey::from_bytes(author.get_id()?.as_slice())?;

        Ok(author_pub_key.verify(&buf, &signature)?)
    }
}

impl ChannelContentItem {
    pub fn new(content: ContentItem) -> ChannelContentItem {
        ChannelContentItem {
            content_item: Some(content),
            signature: None,
        }
    }
}

impl SignedWithExternalVerifier for ChannelContentItem {
    /// Sign a content by content item creator
    fn sign(&mut self, creator_key_pair: &ed25519_dalek::Keypair) -> Result<()> {
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;

        use crate::snp::snp_core_types::Signature;
        self.signature = Some(Signature {
            scheme_id: 0,
            signature: creator_key_pair.sign(&buf).as_ref().to_vec(),
        });

        Ok(())
    }

    /// Caller needs to provide channel creator id which is external to content item
    fn verify_signature(&self, signer: &ed25519_dalek::PublicKey) -> Result<()> {
        let item = self
            .content_item
            .as_ref()
            .ok_or_else(|| anyhow!("missing content item"))?;

        item.verify_signature()?;

        let signature = self
            .signature
            .as_ref()
            .ok_or_else(|| anyhow!("missing channel creator signature"))?
            .clone();

        let mut data = self.clone();
        data.signature = None;

        let channel_signature =
            ed25519_dalek::Signature::from_bytes(signature.signature.as_slice())?;

        use prost::Message;
        let mut buf = Vec::with_capacity(data.encoded_len());
        if data.encode(&mut buf).is_err() {
            return Err(anyhow!("failed to encode source data to binary data"));
        };

        if signer.verify(&buf, &channel_signature).is_err() {
            return Err(anyhow!("failed to verify channel creator signature"));
        };

        Ok(())
    }
}
