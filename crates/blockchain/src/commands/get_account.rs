// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::service::SimpleBlockchainService;
use anyhow::{anyhow, Result};
use base::snp::snp_blockchain::{GetAccountRequest, GetAccountResponse};
use xactor::*;

impl SimpleBlockchainService {
    /// Update account settings
    pub(crate) async fn get_account(request: GetAccountRequest) -> Result<GetAccountResponse> {
        SimpleBlockchainService::from_registry()
            .await?
            .call(GetAccountMessage { request })
            .await?
    }
}

#[message(result = "Result<GetAccountResponse>")]
struct GetAccountMessage {
    request: GetAccountRequest,
}

/// write me
#[async_trait::async_trait]
impl Handler<GetAccountMessage> for SimpleBlockchainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetAccountMessage,
    ) -> Result<GetAccountResponse> {
        let address = msg
            .request
            .address
            .as_ref()
            .ok_or_else(|| anyhow!("missing account address"))?;

        let account = SimpleBlockchainService::read_account(address.data.as_ref()).await?;

        Ok(GetAccountResponse { account })
    }
}
