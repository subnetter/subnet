// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::consts::CLIENTS_BUNDLES_CF;
use crate::service::SimpleBlockchainService;
use anyhow::Result;
use base::snp::snp_blockchain::{GetClientsRequest, GetClientsResponse};
use base::snp::snp_core_types::ProviderSignedClientIdentityBundle;
use db::db_service::{DatabaseService, ReadAllItems};
use xactor::*;

impl SimpleBlockchainService {
    /// write me
    pub(crate) async fn get_clients(request: GetClientsRequest) -> Result<GetClientsResponse> {
        SimpleBlockchainService::from_registry()
            .await?
            .call(GetClientsMessage { _request: request })
            .await?
    }
}

#[message(result = "Result<GetClientsResponse>")]
struct GetClientsMessage {
    _request: GetClientsRequest,
}

/// write me
#[async_trait::async_trait]
impl Handler<GetClientsMessage> for SimpleBlockchainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetClientsMessage,
    ) -> Result<GetClientsResponse> {
        let data = DatabaseService::read_all_items(ReadAllItems {
            from: None,
            max_results: 0,
            cf: CLIENTS_BUNDLES_CF,
        })
        .await?;

        let mut clients_bundles = vec![];
        use prost::Message;
        for item in data.items {
            let bundle = ProviderSignedClientIdentityBundle::decode(item.1.value.as_ref())?;
            clients_bundles.push(bundle)
        }
        Ok(GetClientsResponse { clients_bundles })
    }
}
