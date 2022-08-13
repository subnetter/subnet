//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::model::api::{GetAccountsRequest, GetAccountsResponse};
use anyhow::Result;
use xactor::*;

impl CryptoMailService {
    /// Admin feature - get all accounts regardless of public listing settings
    pub(crate) async fn all_accounts(request: GetAccountsRequest) -> Result<GetAccountsResponse> {
        let service = CryptoMailService::from_registry().await?;

        service.call(GetAllAccountsMessage { request }).await?
    }
}

#[message(result = "Result<GetAccountsResponse>")]
pub(crate) struct GetAllAccountsMessage {
    pub(crate) request: GetAccountsRequest,
}

#[async_trait::async_trait]
impl Handler<GetAllAccountsMessage> for CryptoMailService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetAllAccountsMessage,
    ) -> Result<GetAccountsResponse> {
        let request = msg.request;
        let accounts =
            CryptoMailService::read_all_accounts_from_store(request.from, request.max_results)
                .await?;
        Ok(GetAccountsResponse { total: 0, accounts })
    }
}
