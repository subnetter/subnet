// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::channels::channels_data_service::{ChannelsService, UpsertChannel};
use crate::simple_client::SimpleClient;
use anyhow::{anyhow, Result};
use base::hex_utils::short_hex_string;
use base::snp::snp_core_types::{ChannelData, EntityId, GroupMemberBundle};
use xactor::Service;

impl SimpleClient {
    /// Add user to group
    pub(crate) async fn add_group_member(
        &self,
        user: &EntityId,
        channel_data: &mut ChannelData,
        membership: GroupMemberBundle,
    ) -> Result<()> {
        info!("Adding group member...");

        let members_bundle = channel_data
            .group_members
            .as_mut()
            .ok_or_else(|| anyhow!("missing membership bundle"))?;

        members_bundle.members.push(membership);

        let group_id_key_pair =
            ed25519_dalek::Keypair::from_bytes(channel_data.channel_key_pair.as_ref())?;

        // sign the new bundle with the new members
        members_bundle.sign(&self.client_id, &group_id_key_pair)?;

        // save the bundle
        let channels_service = ChannelsService::from_registry().await?;
        let _ = channels_service
            .call(UpsertChannel(channel_data.clone()))
            .await??;

        let subscriber_id = user.get_id()?;

        info!(
            "Added user {:?} to group",
            short_hex_string(subscriber_id.as_slice())
        );

        Ok(())
    }
}
