// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, Result};
use crypto::utils::entity_from_ed25519_pub_key;
use xactor::*;

use crate::channels::channels_data_service::{ChannelsService, UpsertChannel};
use base::api_types_extensions::Signed;
use base::snp::snp_core_types::{
    ChannelBundle, ChannelData, ChannelType, GroupMemberBundle, GroupMembersBundle, PricingModel,
};
use chrono::prelude::*;

#[message(result = "Result<ChannelBundle>")]
pub(crate) struct CreateNewChannel {
    pub(crate) name: String,
    pub(crate) channel_type: ChannelType, // status updates, group, etc...
    pub(crate) description: String,
}

/// Create a new status update channel or a group by this client and returns its bundle
///
#[async_trait::async_trait]
impl Handler<CreateNewChannel> for SimpleClient {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: CreateNewChannel,
    ) -> Result<ChannelBundle> {
        let channels_service = ChannelsService::from_registry().await?;
        let channel_id_key_pair = ed25519_dalek::Keypair::generate(&mut rand_core::OsRng);
        let channel_id = entity_from_ed25519_pub_key(&channel_id_key_pair.public, msg.name);
        let creator_id =
            entity_from_ed25519_pub_key(&self.client_id.public, "TheOnlyTermiNATor".into());

        let mut bundle = ChannelBundle {
            channel_id: Some(channel_id.clone()),
            creator_id: Some(creator_id.clone()),
            channel_type: msg.channel_type as i32,
            created: Utc::now().timestamp_nanos() as u64,
            description: msg.description,
            acceptable_content_policy:
                "Be nice, thoughtful, and kind when replying or I'll kick you out".to_string(),
            logo: None,
            payable_address: None,
            subscription_fee: None,
            signature: None,
            creator_signature: None,
            pricing_model: PricingModel::Free as i32,
        };

        bundle
            .sign(&self.client_id, &channel_id_key_pair)
            .map_err(|_| anyhow!("failed to sign"))?;

        let mut channel_data = ChannelData {
            bundle: Some(bundle.clone()),
            discoverable: true,
            last_updated: 0,
            blocked_repliers: vec![],
            content_items: vec![],
            sub_requests: vec![],
            subscribers: vec![],
            group_members: None,
            channel_key_pair: channel_id_key_pair.to_bytes().to_vec(),
        };

        if msg.channel_type == ChannelType::Group {
            // Create group members bundle and group creator as sole member
            let mut membership = GroupMemberBundle {
                user_id: Some(creator_id.clone()),
                group_id: Some(channel_id.clone()),
                signature: None,
            };
            membership.sign(&self.client_id)?;

            let mut members_bundle = GroupMembersBundle {
                created: Utc::now().timestamp_nanos() as u64,
                group_id: Some(channel_id),
                creator_id: Some(creator_id),
                members: vec![membership],
                group_signature: None,
                creator_signature: None,
            };

            members_bundle.sign(&self.client_id, &channel_id_key_pair)?;
            channel_data.group_members = Some(members_bundle);
        }

        channels_service.call(UpsertChannel(channel_data)).await??;

        Ok(bundle)
    }
}
