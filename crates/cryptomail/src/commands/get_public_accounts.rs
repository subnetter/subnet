//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::consts::PUB_ACCOUNTS_COL_FAMILY;
use crate::model::api::{GetPublicAccountsRequest, GetPublicAccountsResponse};
use crate::model::types::PublicKey;
use anyhow::Result;
use db::db_service::{DatabaseService, ReadAllItems};
use xactor::*;

impl CryptoMailService {
    // actor handler call wrapper
    pub(crate) async fn public_accounts(
        request: GetPublicAccountsRequest,
    ) -> Result<GetPublicAccountsResponse> {
        let service = CryptoMailService::from_registry().await?;
        service.call(GetPublicAccountsMessage { request }).await?
    }
}

#[message(result = "Result<GetPublicAccountsResponse>")]
pub(crate) struct GetPublicAccountsMessage {
    pub(crate) request: GetPublicAccountsRequest,
}

#[async_trait::async_trait]
impl Handler<GetPublicAccountsMessage> for CryptoMailService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetPublicAccountsMessage,
    ) -> Result<GetPublicAccountsResponse> {
        // for now, no pagination support as this is by name and not max items, etc...
        // todo: support offset by name in api level...
        let mut accounts = vec![];

        let from = if msg.request.from.is_empty() {
            None
        } else {
            Some(msg.request.from.to_lowercase())
        };

        let data = DatabaseService::read_all_items(ReadAllItems {
            from,
            max_results: msg.request.max_results,
            cf: PUB_ACCOUNTS_COL_FAMILY,
        })
        .await?;

        for item in data.items {
            let name = String::from_utf8(item.0.to_vec())?;
            let pub_key = PublicKey {
                key: item.1.value.to_vec(),
            };

            if let Some(account) = CryptoMailService::load_account_from_store(&pub_key).await? {
                accounts.push(account);
            } else {
                error!("failed to load account by name: {}", name);
            }
        }

        Ok(GetPublicAccountsResponse {
            total: data.total_keys as u32,
            accounts,
        })
    }
}
