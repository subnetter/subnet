//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::model::types::{Compression, ContentItem, MessageContent, MimeType, PreKey};
use anyhow::{bail, Result};
use base::hex_utils::hex_string;
use common::network_salt::AES_IV;
use crypto::aes_cypher::AesCypher;
use ed25519_dalek::ed25519::signature::rand_core::OsRng;
use std::fmt;
use std::fmt::{Display, Formatter};
use x25519_dalek::{PublicKey, StaticSecret};

impl Display for MessageContent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        /*
        write!(
            f,
            "preimage: {}. ",
            short_hex_string(self.preimage.as_slice())
        )?;
         */

        write!(f, "thread")
    }
}

impl MessageContent {
    // todo: implement display

    /// Returns a basic message with subject line and text body
    pub fn new_basic_message() -> Self {
        MessageContent {
            subject: Some(ContentItem {
                mime_type: MimeType::TextUtf8 as i32,
                compression: Compression::Uncompressed as i32,
                data: "this is a the subject".as_bytes().to_vec(),
            }),
            body: Some(ContentItem {
                mime_type: MimeType::TextUtf8 as i32,
                compression: Compression::Uncompressed as i32,
                data: "This is a message's body".as_bytes().to_vec(),
            }),
            media_items: vec![],
        }
    }

    /// Encrypt a message for a recipient, using its pre-key x25519 key and a new ephemeral key
    /// Returns public ephemeral key of private ephemeral key used to encrypt the message
    pub(crate) fn encrypt_message(
        &self,
        recipient_pre_key: &PreKey,
    ) -> Result<(PublicKey, Vec<u8>)> {
        // all bellow needs to be encapsulated in a method with all the keys input and the message content
        // with result being the enc message

        let x_recipient_pre_key_pub = recipient_pre_key.get_x25519_pub_key()?;
        info!(
            "enc message, recipient pre key id {} and bytes: {}",
            recipient_pre_key.id,
            hex_string(x_recipient_pre_key_pub.as_bytes())
        );

        let eph_secret = StaticSecret::new(OsRng);
        let x_eph_pub_key: x25519_dalek::PublicKey = x25519_dalek::PublicKey::from(&eph_secret);
        info!(
            "enc message eph private key: {}",
            hex_string(eph_secret.to_bytes().as_ref())
        );

        info!(
            "enc message eph public key: {}",
            hex_string(x_eph_pub_key.as_bytes())
        );

        let shared_secret = eph_secret.diffie_hellman(&x_recipient_pre_key_pub);
        info!(
            "enc message shared secret: {}",
            hex_string(shared_secret.as_bytes().to_vec().as_slice())
        );
        let enc_message = self.encrypt(shared_secret.as_bytes()).unwrap();
        info!("enc message bytes: {}", hex_string(enc_message.as_slice()));
        Ok((x_eph_pub_key, enc_message))
    }

    /// Basic verification of required content
    pub fn verify(&self) -> Result<()> {
        if self.body.is_none() {
            bail!("missing body")
        }

        if self.subject.is_none() {
            bail!("missing subject")
        }

        Ok(())
    }

    /// Decrypt ciphertext to a MessageContent using the provided encryption key
    pub fn decrypt(enc_message: &[u8], key: &[u8; 32]) -> Result<MessageContent> {
        let clear_text = AesCypher::aes256_cbc_pkcs7_decrypt(key, AES_IV.as_ref(), enc_message)?;
        use prost::Message;
        Ok(MessageContent::decode(clear_text.as_slice())?)
    }

    /// Encrypt a message to bytes using provided encryption key
    pub fn encrypt(&self, key: &[u8; 32]) -> Result<Vec<u8>> {
        use prost::Message;
        let mut buff = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buff)?;
        AesCypher::aes256_cbc_pkcs7_encrypt(key, AES_IV.as_ref(), buff.as_ref())
    }
}
