// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::network_salt::NET_SALT;
use anyhow::Result;
use base::api_types_extensions::Signed;
use base::snp::snp_server_api::TypedMessage;
use bytes::Bytes;
use crypto::aead_cypher::AeadCipher;
use double_ratchet::message_key::MessageKey;

pub struct TypedMessageExtensions;

/// Decrypt an encrypted TypedMessage bytes using a message enc key and ad
/// is aware of base and the crypto crates
/// also authenticates that message was sent by identity in it
impl TypedMessageExtensions {
    pub fn decrypt_msg(enc_message: &[u8], key: &MessageKey, ad: &[u8]) -> Result<TypedMessage> {
        let cipher = AeadCipher::new(
            Bytes::from(NET_SALT.to_vec()),
            Bytes::from(key.to_vec()),
            Bytes::from(ad.to_vec()),
        );

        let clear_text = cipher.decrypt(enc_message)?;

        use prost::Message;
        let typed_message = TypedMessage::decode(clear_text.as_slice())?;
        typed_message.verify_signature()?;
        Ok(typed_message)
    }

    /// Encrypt a TypedMessage into an EncryptedTypedMessage using an encryption key and ad
    /// is aware of base and the crypto crates
    /// currently used by integration test by client
    pub fn encrypt_msg(message: TypedMessage, key: &MessageKey, ad: &[u8]) -> Result<Bytes> {
        use prost::Message;
        let mut buff = Vec::with_capacity(message.encoded_len());
        message.encode(&mut buff)?;

        let cipher = AeadCipher::new(
            Bytes::from(NET_SALT.to_vec()),
            Bytes::from(key.to_vec()),
            Bytes::from(ad.to_vec()),
        );

        cipher.encrypt(Bytes::from(buff))
    }
}
