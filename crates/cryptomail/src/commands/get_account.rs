//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::model::api::get_account_request::Data::*;
use crate::model::api::{GetAccountRequest, GetAccountResponse};
use crate::model::types::PublicKey;
use anyhow::{bail, Result};
use xactor::*;

impl CryptoMailService {
    pub(crate) async fn get_account(request: GetAccountRequest) -> Result<GetAccountResponse> {
        let service = CryptoMailService::from_registry().await?;
        service.call(GetAccountMessage { request }).await?
    }
}

#[message(result = "Result<GetAccountResponse>")]
pub struct GetAccountMessage {
    pub request: GetAccountRequest,
}

/// Return public account info for a name or a pub key
#[async_trait::async_trait]
impl Handler<GetAccountMessage> for CryptoMailService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetAccountMessage,
    ) -> Result<GetAccountResponse> {
        let req = msg.request;

        if req.data.is_none() {
            info!("missing request required data");
            bail!("missing request required data")
        }

        // pick address from name registry or from caller
        let pub_key: Option<PublicKey> = match req.data.unwrap() {
            PublicKey(pub_key) => {
                info!("request for account pub_key: {}", pub_key);
                Some(pub_key)
            }
            Name(name) => {
                info!("request for account name: {}", name);
                CryptoMailService::read_account_by_name(&name).await?
            }
        };

        if pub_key.is_none() {
            info!("account not found by pub key");
            return Ok(GetAccountResponse { account: None });
        }

        let account = CryptoMailService::load_account_from_store(&pub_key.unwrap()).await?;

        info!("returning account info");
        Ok(GetAccountResponse { account })
    }
}
