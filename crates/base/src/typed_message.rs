// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::api_types_extensions::{Signed, SignedWithExternalVerifier};
use crate::snp::snp_server_api::{NewSessionRequest, TypedMessage};
use anyhow::anyhow;
use ed25519_dalek::ed25519::signature::Signature;

impl Signed for TypedMessage {
    /// Sign the message
    fn sign(&mut self, signer: &ed25519_dalek::Keypair) -> anyhow::Result<()> {
        use ed25519_dalek::Signer;
        use prost::Message;
        // Sign and add signature to typed_msg as alice
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;
        let typed_message_signature = signer.sign(&buf);
        use crate::snp::snp_core_types::Signature;
        self.signature = Some(Signature {
            scheme_id: 0,
            signature: typed_message_signature.to_bytes().to_vec(),
        });

        Ok(())
    }

    /// Verify sender's signature on the message
    fn verify_signature(&self) -> anyhow::Result<()> {
        let signature_data = self
            .signature
            .as_ref()
            .ok_or_else(|| anyhow!("missing signature"))?;

        // create message binary data for signature verification
        let mut data = self.clone();

        data.signature = None; // remove signature from message before verification
        use ed25519_dalek::Verifier;
        use prost::Message;
        let mut buf = Vec::with_capacity(data.encoded_len());
        if data.encode(&mut buf).is_err() {
            return Err(anyhow!("failed to encode source data to binary data"));
        };

        let signature = ed25519_dalek::Signature::from_bytes(signature_data.signature.as_slice())?;

        let ika = self.get_ika()?;

        if ika.verify(&buf, &signature).is_err() {
            return Err(anyhow!(
                "failed to verify signature of sender on message content"
            ));
        };

        Ok(())
    }
}

impl TypedMessage {
    // Get message sender public key from the message
    pub fn get_ika(&self) -> anyhow::Result<ed25519_dalek::PublicKey> {
        self.sender
            .as_ref()
            .ok_or_else(|| anyhow!("missing sender"))?
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing public key"))?
            .as_pub_key()
    }

    pub fn get_receiver_id(&self) -> anyhow::Result<ed25519_dalek::PublicKey> {
        self.receiver
            .as_ref()
            .ok_or_else(|| anyhow!("missing reciever"))?
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing public key"))?
            .as_pub_key()
    }
}

impl SignedWithExternalVerifier for NewSessionRequest {
    fn sign(&mut self, signer: &ed25519_dalek::Keypair) -> anyhow::Result<()> {
        use ed25519_dalek::Signer;
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;
        let signature = signer.sign(&buf);
        use crate::snp::snp_core_types::Signature;
        self.sender_signature = Some(Signature {
            scheme_id: 0,
            signature: signature.to_bytes().to_vec(),
        });

        Ok(())
    }

    // verify sender's signature
    fn verify_signature(&self, pub_key: &ed25519_dalek::PublicKey) -> anyhow::Result<()> {
        if self.sender_signature.is_none() {
            return Err(anyhow!("missing message signature"));
        }

        let ika_signature_data = self.sender_signature.as_ref().unwrap().clone();

        // create message binary data for signature verification
        let mut mut_req_data = self.clone();
        mut_req_data.sender_signature = None; // remove signature from message before verification
        use ed25519_dalek::Verifier;
        use prost::Message;
        let mut buf = Vec::with_capacity(mut_req_data.encoded_len());
        if mut_req_data.encode(&mut buf).is_err() {
            return Err(anyhow!("failed to encode source data to binary data"));
        };

        let ika_signature =
            ed25519_dalek::Signature::from_bytes(ika_signature_data.signature.as_slice())?;

        if pub_key.verify(&buf, &ika_signature).is_err() {
            return Err(anyhow!(
                "failed to verify signature of sender on message content"
            ));
        };

        Ok(())
    }
}
