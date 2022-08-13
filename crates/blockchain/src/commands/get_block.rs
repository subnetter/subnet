// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::service::SimpleBlockchainService;
use anyhow::Result;
use base::snp::snp_blockchain::{GetBlockRequest, GetBlockResponse, GetCurrentBlockRequest};
use xactor::*;

impl SimpleBlockchainService {
    ///
    pub(crate) async fn get_block(request: GetBlockRequest) -> Result<GetBlockResponse> {
        SimpleBlockchainService::from_registry()
            .await?
            .call(GetBlockMessage { request })
            .await?
    }

    ///
    pub(crate) async fn get_current_block(
        request: GetCurrentBlockRequest,
    ) -> Result<GetBlockResponse> {
        SimpleBlockchainService::from_registry()
            .await?
            .call(GetCurrentBlockMessage { _request: request })
            .await?
    }
}

#[message(result = "Result<GetBlockResponse>")]
struct GetCurrentBlockMessage {
    _request: GetCurrentBlockRequest,
}

/// write me
#[async_trait::async_trait]
impl Handler<GetCurrentBlockMessage> for SimpleBlockchainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetCurrentBlockMessage,
    ) -> Result<GetBlockResponse> {
        let block_id = SimpleBlockchainService::read_current_block_id().await?;

        Ok(GetBlockResponse {
            block: SimpleBlockchainService::read_block(block_id).await?,
        })
    }
}

#[message(result = "Result<GetBlockResponse>")]
struct GetBlockMessage {
    request: GetBlockRequest,
}

/// write me
#[async_trait::async_trait]
impl Handler<GetBlockMessage> for SimpleBlockchainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetBlockMessage,
    ) -> Result<GetBlockResponse> {
        Ok(GetBlockResponse {
            block: SimpleBlockchainService::read_block(msg.request.block_id).await?,
        })
    }
}
