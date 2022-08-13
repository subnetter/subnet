// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::service::SimpleBlockchainService;
use anyhow::{anyhow, Result};
use base::snp::snp_blockchain::{Account, SetBalanceRequest, SetBalanceResponse};
use xactor::*;

impl SimpleBlockchainService {
    /// Update account settings
    pub(crate) async fn set_balance(request: SetBalanceRequest) -> Result<SetBalanceResponse> {
        SimpleBlockchainService::from_registry()
            .await?
            .call(SetBalanceMessage { request })
            .await?
    }
}

#[message(result = "Result<SetBalanceResponse>")]
struct SetBalanceMessage {
    request: SetBalanceRequest,
}

/// write me
#[async_trait::async_trait]
impl Handler<SetBalanceMessage> for SimpleBlockchainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: SetBalanceMessage,
    ) -> Result<SetBalanceResponse> {
        let req = msg.request;

        let address = req
            .address
            .as_ref()
            .ok_or_else(|| anyhow!("missing address"))?;

        let amount = req
            .amount
            .as_ref()
            .ok_or_else(|| anyhow!("missing amount"))?;

        let mut account = match SimpleBlockchainService::read_account(&address.data).await? {
            None => Account {
                address: Some(address.clone()),
                nonce: 0,
                balances: vec![],
            },
            Some(a) => a,
        };

        info!("setting balance: {} for account {}", amount.value, address);

        account.set_balance(amount);
        SimpleBlockchainService::store_account(&account).await?;
        Ok(SetBalanceResponse {})
    }
}
