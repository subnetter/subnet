//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::model::api::get_account_data_request::Data::*;
use crate::model::api::{GetAccountDataRequest, GetAccountDataResponse};
use crate::model::types::PublicKey;
use anyhow::{bail, Result};
use xactor::*;

impl CryptoMailService {
    pub(crate) async fn admin_account_info(
        request: GetAccountDataRequest,
    ) -> Result<GetAccountDataResponse> {
        let service = CryptoMailService::from_registry().await?;
        service.call(GetAccountDataMessage { request }).await?
    }
}

#[message(result = "Result<GetAccountDataResponse>")]
pub struct GetAccountDataMessage {
    pub request: GetAccountDataRequest,
}

/// Returns complete account and thradboxes info known to server to an admin
#[async_trait::async_trait]
impl Handler<GetAccountDataMessage> for CryptoMailService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetAccountDataMessage,
    ) -> Result<GetAccountDataResponse> {
        let req = msg.request;

        if req.data.is_none() {
            bail!("missing request info")
        }

        // pick address from name registry or from caller
        let public_key: Option<PublicKey> = match req.data.unwrap() {
            PublicKey(pub_key) => Some(pub_key),
            Name(name) => CryptoMailService::read_account_by_name(&name).await?,
        };

        if public_key.is_none() {
            bail!("unrecognized address")
        }

        match CryptoMailService::load_account_from_store(&public_key.unwrap()).await? {
            Some(account) => {
                let thread_boxes = account.get_thread_boxes().await?;
                Ok(GetAccountDataResponse {
                    account: Some(account),
                    thread_boxes,
                })
            }
            None => bail!("unrecognized account"),
        }
    }
}
