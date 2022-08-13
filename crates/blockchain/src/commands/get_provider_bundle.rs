// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::service::SimpleBlockchainService;
use anyhow::{anyhow, Result};
use base::snp::snp_blockchain::{
    GetProviderIdentityBundleRequest, GetProviderIdentityBundleResponse,
};
use xactor::*;

impl SimpleBlockchainService {
    /// write me
    pub(crate) async fn get_provider_bundle(
        request: GetProviderIdentityBundleRequest,
    ) -> Result<GetProviderIdentityBundleResponse> {
        SimpleBlockchainService::from_registry()
            .await?
            .call(GetProviderIdentityBundleMessage { request })
            .await?
    }
}

#[message(result = "Result<GetProviderIdentityBundleResponse>")]
struct GetProviderIdentityBundleMessage {
    request: GetProviderIdentityBundleRequest,
}

/// write me
#[async_trait::async_trait]
impl Handler<GetProviderIdentityBundleMessage> for SimpleBlockchainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetProviderIdentityBundleMessage,
    ) -> Result<GetProviderIdentityBundleResponse> {
        let id = msg
            .request
            .entity_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing provider id"))?
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing provider public key"))?;

        Ok(GetProviderIdentityBundleResponse {
            provider_bundle: SimpleBlockchainService::read_provider_bundle(id.key.as_ref()).await?,
        })
    }
}
