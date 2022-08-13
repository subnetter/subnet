// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::api_types_extensions::Signed;
use crate::snp::snp_core_types::{GroupMemberBundle, GroupMembersBundle};
use anyhow::{anyhow, Result};
use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::{Signer, Verifier};

impl GroupMembersBundle {
    pub fn get_member(&self, user_id: &[u8]) -> Option<GroupMemberBundle> {
        match self
            .members
            .iter()
            .position(|sub| sub.get_member_id().unwrap().as_slice() == user_id)
        {
            Some(idx) => Some(self.members[idx].clone()),
            None => None,
        }
    }

    pub fn sign(
        &mut self,
        creator_key_pair: &ed25519_dalek::Keypair,
        group_key_pair: &ed25519_dalek::Keypair,
    ) -> Result<()> {
        self.group_signature = None;
        self.creator_signature = None;

        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;

        use crate::snp::snp_core_types::Signature;
        self.group_signature = Some(Signature {
            scheme_id: 0,
            signature: group_key_pair.sign(&buf).as_ref().to_vec(),
        });

        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;
        self.creator_signature = Some(Signature {
            scheme_id: 0,
            signature: creator_key_pair.sign(&buf).as_ref().to_vec(),
        });

        Ok(())
    }
}

impl GroupMemberBundle {
    pub fn get_member_id(&self) -> Result<Vec<u8>> {
        Ok(self
            .user_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing client id"))?
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing pub key"))?
            .key
            .clone())
    }
}

impl Signed for GroupMemberBundle {
    fn sign(&mut self, member_key_pair: &ed25519_dalek::Keypair) -> Result<()> {
        self.signature = None;
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;

        use crate::snp::snp_core_types::Signature;
        self.signature = Some(Signature {
            scheme_id: 0,
            signature: member_key_pair.sign(&buf).as_ref().to_vec(),
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

        let signer = self
            .user_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing user id"))?;

        let signer_pub_key = ed25519_dalek::PublicKey::from_bytes(signer.get_id()?.as_slice())?;
        Ok(signer_pub_key.verify(&buf, &signature)?)
    }
}
