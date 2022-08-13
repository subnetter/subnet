//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::provider_id::ProviderIdService;
use crate::services::provider_id_service::GetCurrentIdentityBundle;
use anyhow::{anyhow, Result};
use base::api_types_extensions::Signed;
use base::snp::snp_core_types::{EntityId, PrivateProviderIdentityBundle, PublicKey};
use base::snp::snp_server_api::{DrSessionHeader, Message, MessageType, TypedMessage};
use bytes::Bytes;
use chrono::prelude::*;
use common::dr_service::DRService;
use common::typed_msg_extensions::TypedMessageExtensions;
use double_ratchet::dr::DoubleRatchet;
use xactor::Service;

/// Creates a new outgoing message from this provider to another receiver (provider or a client)
/// DR session is going to be persisted after being updated.
pub async fn new_outgoing_message(
    message_type: MessageType,
    message: Bytes,
    dr: &mut DoubleRatchet,
    receiver_id: ed25519_dalek::PublicKey,
) -> Result<Message> {
    let pub_dr_key = dr.get_public_key().unwrap();
    let send_key = dr.next_sending_key().unwrap();
    let provider_id_service = ProviderIdService::from_registry()
        .await
        .map_err(|e| anyhow!(format!("failed to get provider id service: {:?}", e)))?;
    let bundle: PrivateProviderIdentityBundle = provider_id_service
        .call(GetCurrentIdentityBundle {})
        .await??;

    // this is the message we sends to the client
    let mut typed_msg = TypedMessage {
        time_stamp: Utc::now().timestamp_nanos() as u64,
        msg_type: message_type as i32,
        message: message.to_vec(),
        receiver: Some(EntityId {
            public_key: Some(PublicKey {
                key: receiver_id.as_ref().to_vec(),
            }),
            nickname: "".to_string(),
        }),
        sender: Some(bundle.get_provider_id_entity()?.clone()),
        signature: None,
    };

    let key_pair = bundle
        .provider_id_keypair
        .as_ref()
        .ok_or_else(|| anyhow!("missing key pair"))?
        .to_ed2559_kaypair();

    typed_msg
        .sign(&key_pair)
        .map_err(|e| anyhow!("failed to sign message: {:?}", e))?;

    let enc_msg = TypedMessageExtensions::encrypt_msg(typed_msg, &send_key.1, dr.get_ad()?)
        .map_err(|e| anyhow!("failed to enc message: {:?}", e))?;

    // step 5: save the dr session using the dr server
    DRService::save_dr_session(receiver_id, dr.clone()).await?;

    // step 6: send NewSessionRequest(message) to bob
    let message = base::snp::snp_server_api::Message {
        header: Some(DrSessionHeader {
            session_id: dr.session_id,
            dr_pub_key: Some(base::snp::snp_core_types::PublicKey {
                key: pub_dr_key.as_bytes().to_vec(),
            }),
            prev_count: 0,
            count: send_key.0,
        }),
        enc_typed_msg: enc_msg.to_vec(),
    };

    Ok(message)
}
