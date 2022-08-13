// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::consts::PROVIDERS_BUNDLES_CF;
use crate::service::SimpleBlockchainService;
use anyhow::Result;
use base::snp::snp_blockchain::{GetProvidersRequest, GetProvidersResponse};
use base::snp::snp_core_types::ProviderIdentityBundle;
use db::db_service::{DatabaseService, ReadAllItems};
use xactor::*;

impl SimpleBlockchainService {
    /// write me
    pub(crate) async fn get_providers(
        request: GetProvidersRequest,
    ) -> Result<GetProvidersResponse> {
        SimpleBlockchainService::from_registry()
            .await?
            .call(GetProvidersMessage { _request: request })
            .await?
    }
}

#[message(result = "Result<GetProvidersResponse>")]
struct GetProvidersMessage {
    _request: GetProvidersRequest,
}

/// write me
#[async_trait::async_trait]
impl Handler<GetProvidersMessage> for SimpleBlockchainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetProvidersMessage,
    ) -> Result<GetProvidersResponse> {
        let data = DatabaseService::read_all_items(ReadAllItems {
            from: None,
            max_results: 0,
            cf: PROVIDERS_BUNDLES_CF,
        })
        .await?;

        let mut providers_bundles = vec![];
        use prost::Message;
        for item in data.items {
            let bundle = ProviderIdentityBundle::decode(item.1.value.as_ref())?;
            providers_bundles.push(bundle)
        }
        Ok(GetProvidersResponse { providers_bundles })
    }
}
