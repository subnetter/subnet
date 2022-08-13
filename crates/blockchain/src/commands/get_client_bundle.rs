// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::service::SimpleBlockchainService;
use anyhow::{anyhow, Result};
use base::snp::snp_blockchain::{GetClientIdentityBundleRequest, GetClientIdentityBundleResponse};
use xactor::*;

impl SimpleBlockchainService {
    /// write me
    pub(crate) async fn get_client_bundle(
        request: GetClientIdentityBundleRequest,
    ) -> Result<GetClientIdentityBundleResponse> {
        SimpleBlockchainService::from_registry()
            .await?
            .call(GetClientIdentityBundleMessage { request })
            .await?
    }
}

#[message(result = "Result<GetClientIdentityBundleResponse>")]
struct GetClientIdentityBundleMessage {
    request: GetClientIdentityBundleRequest,
}

/// write me
#[async_trait::async_trait]
impl Handler<GetClientIdentityBundleMessage> for SimpleBlockchainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetClientIdentityBundleMessage,
    ) -> Result<GetClientIdentityBundleResponse> {
        let id = msg
            .request
            .entity_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing provider id"))?
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing provider public key"))?;

        Ok(GetClientIdentityBundleResponse {
            client_bundle: SimpleBlockchainService::read_client_bundle(id.key.as_ref()).await?,
        })
    }
}
