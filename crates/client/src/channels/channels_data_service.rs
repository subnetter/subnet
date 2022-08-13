// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use base::snp::snp_core_types::ChannelData;
use std::collections::HashMap;
use xactor::*;

/// Channel service manages data for channels created and owned by this client.
/// Channels handle groups and status updates data.
/// Only channel or group creator has the channel's data.
/// Currently uses an in-memory db - should use a persistent db
#[derive(Debug)]
pub struct ChannelsService {
    // key is channel_id
    channels: HashMap<Vec<u8>, ChannelData>,
}

impl Service for ChannelsService {}

impl Default for ChannelsService {
    fn default() -> Self {
        ChannelsService {
            channels: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl Actor for ChannelsService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("ChannelsService started");
        Ok(())
    }
}

#[message(result = "Result<()>")]
pub(crate) struct UpsertChannel(pub(crate) ChannelData);

#[async_trait::async_trait]
impl Handler<UpsertChannel> for ChannelsService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: UpsertChannel) -> Result<()> {
        let key = msg.0.get_channel_id()?;
        self.channels.insert(key, msg.0);
        Ok(())
    }
}

#[message(result = "Result<Option<ChannelData>>")]
pub(crate) struct GetChannel(pub(crate) Vec<u8>);

#[async_trait::async_trait]
impl Handler<GetChannel> for ChannelsService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetChannel,
    ) -> Result<Option<ChannelData>> {
        match self.channels.get(&msg.0) {
            Some(channel) => Ok(Some(channel.clone())),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::channels::channels_data_service::ChannelsService;
    use base::snp::snp_core_types::{ChannelBundle, ChannelType, PricingModel};
    use base::test_helpers::enable_logger;
    use chrono::prelude::*;
    use crypto::utils::entity_from_ed25519_pub_key;

    #[tokio::test]
    async fn upsert_channel() {
        enable_logger();
        let channels_service = ChannelsService::from_registry().await.unwrap();
        let client_id_key_pair = ed25519_dalek::Keypair::generate(&mut rand_core::OsRng);
        let channel_id_key_pair = ed25519_dalek::Keypair::generate(&mut rand_core::OsRng);

        let mut bundle = ChannelBundle {
            channel_id: Some(entity_from_ed25519_pub_key(
                &channel_id_key_pair.public,
                "DJ Fuzzy Logic Status Updates".into(),
            )),
            creator_id: Some(entity_from_ed25519_pub_key(
                &client_id_key_pair.public,
                "DJ Fuzzy Logic".into(),
            )),
            channel_type: ChannelType::StatusFeed as i32,
            created: Utc::now().timestamp_nanos() as u64,
            description: "My Upsetter Status Updates".to_string(),
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
            .sign(&client_id_key_pair, &channel_id_key_pair)
            .unwrap();

        let channel_data = ChannelData {
            bundle: Some(bundle),
            discoverable: true,
            last_updated: 0,
            blocked_repliers: vec![],
            content_items: vec![],
            sub_requests: vec![],
            subscribers: vec![],
            group_members: None,
            channel_key_pair: channel_id_key_pair.to_bytes().to_vec(),
        };

        channels_service
            .call(UpsertChannel(channel_data))
            .await
            .unwrap()
            .unwrap();

        let key = channel_id_key_pair.public.as_ref().to_vec();

        let _ = channels_service
            .call(GetChannel(key))
            .await
            .unwrap()
            .unwrap()
            .unwrap();
    }
}
