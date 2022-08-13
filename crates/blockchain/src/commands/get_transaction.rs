// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::service::SimpleBlockchainService;
use anyhow::{anyhow, Result};
use base::snp::snp_blockchain::{GetTransactionRequest, GetTransactionResponse};
use xactor::*;

impl SimpleBlockchainService {
    /// Update account settings
    pub(crate) async fn get_transaction(
        request: GetTransactionRequest,
    ) -> Result<GetTransactionResponse> {
        SimpleBlockchainService::from_registry()
            .await?
            .call(GetTransactionMessage { request })
            .await?
    }
}

#[message(result = "Result<GetTransactionResponse>")]
struct GetTransactionMessage {
    request: GetTransactionRequest,
}

/// write me
#[async_trait::async_trait]
impl Handler<GetTransactionMessage> for SimpleBlockchainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetTransactionMessage,
    ) -> Result<GetTransactionResponse> {
        let id: &Vec<u8> = msg
            .request
            .id
            .as_ref()
            .ok_or_else(|| anyhow!("missing account address"))?
            .id
            .as_ref();

        Ok(GetTransactionResponse {
            transaction_info: SimpleBlockchainService::read_transaction(id).await?,
        })
    }
}
