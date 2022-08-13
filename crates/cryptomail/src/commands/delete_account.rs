//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::model::api::{DeleteAccountRequest, DeleteAccountResponse};
use crate::model::extensions::{Signed, Validatable};
use anyhow::{bail, Result};
use xactor::*;

impl CryptoMailService {
    // actor handler call wrapper
    pub(crate) async fn delete_account(
        request: DeleteAccountRequest,
    ) -> Result<DeleteAccountResponse> {
        let service = CryptoMailService::from_registry().await?;
        service.call(DeleteAccountMessage { request }).await?
    }
}

#[message(result = "Result<DeleteAccountResponse>")]
pub(crate) struct DeleteAccountMessage {
    pub(crate) request: DeleteAccountRequest,
}

#[async_trait::async_trait]
impl Handler<DeleteAccountMessage> for CryptoMailService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: DeleteAccountMessage,
    ) -> Result<DeleteAccountResponse> {
        let request = msg.request;
        request.validate()?;
        request.verify_signature()?;

        let opt_account =
            CryptoMailService::load_account_from_store(request.public_key.as_ref().unwrap())
                .await?;
        if opt_account.is_none() {
            bail!("unknown account")
        }

        let account = opt_account.unwrap();
        account.delete_all_thread_boxes().await?;
        CryptoMailService::delete_account_name(account.get_name().as_str()).await?;
        if account.get_public_listing() {
            CryptoMailService::remove_account_from_public_listing(account.get_name().as_str())
                .await?;
        }

        CryptoMailService::delete_account_from_store(&account).await?;
        Ok(DeleteAccountResponse {})
    }
}
