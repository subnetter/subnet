// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::channels::channels_data_service::{ChannelsService, GetChannel, UpsertChannel};
use crate::simple_client::SimpleClient;
use anyhow::{anyhow, bail, Result};
use base::hex_utils::short_hex_string;
use base::snp::snp_client_to_client::{
    CancelChannelSubscription, CancelChannelSubscriptionResponse,
};
use base::snp::snp_core_types::{ChannelData, ChannelType, EntityId};
use base::snp::snp_server_api::{MessageType, TypedMessage};
use xactor::Service;

impl SimpleClient {
    /// Handle a remote request from another client to unsubscribe to a status update channels owned by this client
    /// or to leave a group he's member of that is owned by this client
    pub(crate) async fn handle_unsubscribe_from_channel_message(
        &mut self,
        msg: TypedMessage,
    ) -> Result<()> {
        use prost::Message;
        let request: CancelChannelSubscription =
            CancelChannelSubscription::decode(msg.message.as_slice())
                .map_err(|e| anyhow!("failed to decode message {:?}", e))?;

        let caller = request
            .user
            .ok_or_else(|| anyhow!("missing user entity from request"))?;

        let channels_service = ChannelsService::from_registry().await?;

        let channel_data: ChannelData = channels_service
            .call(GetChannel(request.channel_id))
            .await??
            .ok_or_else(|| anyhow!("unknown channel"))?;

        let channel_id = channel_data.get_channel_id()?;

        match channel_data.get_bundle()?.channel_type {
            t if t == ChannelType::StatusFeed as i32 => {
                self.unsubscribe_from_channel(channel_data, &caller).await?;
            }
            t if t == ChannelType::Group as i32 => {
                self.leave_group(channel_data, &caller).await?;
            }
            _ => bail!("unknown channel type"),
        };

        // send confirmation of the unsubscription messge to caller
        let resp = CancelChannelSubscriptionResponse { channel_id };
        let caller_pub_key = caller
            .public_key
            .ok_or_else(|| anyhow!("missing pub key"))?;
        let mut buff = Vec::with_capacity(resp.encoded_len());
        resp.encode(&mut buff).unwrap();
        let typed_msg = self.create_typed_message(
            MessageType::ChannelUnsubscribeResponse,
            buff,
            caller_pub_key.as_pub_key()?,
        )?;
        let receiver_id = bytes::Bytes::from(caller_pub_key.key);
        debug!("sending subscription request to other client via our provider...");
        self.send_typed_message(typed_msg, receiver_id).await?;

        Ok(())
    }

    async fn unsubscribe_from_channel(
        &mut self,
        mut channel_data: ChannelData,
        subscriber: &EntityId,
    ) -> Result<()> {
        let subscriber_id = subscriber.get_id()?.as_slice();

        // todo: Implement efficient access to subscriber by id. Current implementation is inefficient for large groups and channels.

        match channel_data
            .subscribers
            .iter()
            .position(|sub| sub.has_subscriber_id(subscriber_id))
        {
            Some(idx) => {
                let _ = channel_data.subscribers.remove(idx);
            }
            None => {
                warn!("did not find subscriber in channel members");
            }
        }

        let channels_service = ChannelsService::from_registry().await?;
        let _ = channels_service.call(UpsertChannel(channel_data)).await??;
        info!(
            "unsubscribed user {:?} from channel",
            short_hex_string(subscriber_id)
        );

        Ok(())
    }

    async fn leave_group(
        &mut self,
        mut channel_data: ChannelData,
        member: &EntityId,
    ) -> Result<()> {
        let members_bundle = channel_data
            .group_members
            .as_mut()
            .ok_or_else(|| anyhow!("missing group members"))?;

        let member_id = member.get_id()?;

        if let Some(idx) = members_bundle
            .members
            .iter()
            .position(|sub| *sub.user_id.as_ref().unwrap().get_id().as_ref().unwrap() == member_id)
        {
            members_bundle.members.remove(idx);
        } else {
            bail!("non member of this group");
        }

        // Sign the new bundle without the removed member - in the future we are going to share the bundle
        // with other group members so they have an updated list of members which is authenticated

        let group_id_key_pair =
            ed25519_dalek::Keypair::from_bytes(channel_data.channel_key_pair.as_ref())?;

        members_bundle.sign(&self.client_id, &group_id_key_pair)?;
        let channels_service = ChannelsService::from_registry().await?;
        let _ = channels_service.call(UpsertChannel(channel_data)).await??;

        info!(
            "removed user {:?} from group",
            short_hex_string(member_id.as_ref())
        );

        Ok(())
    }
}
