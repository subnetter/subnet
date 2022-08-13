//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::model::api::{DeleteThreadRequest, DeleteThreadResponse};
use crate::model::extensions::{Signed, Validatable};
use crate::model::types::{ThreadBoxType, ThreadId};
use anyhow::{anyhow, bail, Result};
use xactor::*;

impl CryptoMailService {
    pub(crate) async fn delete_thread(
        request: DeleteThreadRequest,
    ) -> Result<DeleteThreadResponse> {
        let service = CryptoMailService::from_registry().await?;
        service.call(DeleteThreadMessage { request }).await?
    }
}

#[message(result = "Result<DeleteThreadResponse>")]
pub struct DeleteThreadMessage {
    pub request: DeleteThreadRequest,
}

/// Remove a thread form a user's inbox and add it to the archive-box
#[async_trait::async_trait]
impl Handler<DeleteThreadMessage> for CryptoMailService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: DeleteThreadMessage,
    ) -> Result<DeleteThreadResponse> {
        let req = msg.request;
        req.validate()?;
        req.verify_signature()?;
        let opt_account =
            CryptoMailService::load_account_from_store(req.public_key.as_ref().unwrap()).await?;
        if opt_account.is_none() {
            bail!("unknown account")
        }

        let account = opt_account.unwrap();
        let mut inbox = account
            .load_thread_box(ThreadBoxType::Inbox)
            .await?
            .ok_or_else(|| anyhow!("missing inbox"))?;

        if let Some(idx) = inbox.thread_ids.iter().position(|id| *id == req.thread_id) {
            inbox.thread_ids.remove(idx);
        } else {
            bail!("invalid thread id")
        }

        account.save_thread_box(inbox).await?;

        Ok(DeleteThreadResponse {
            thread_id: Some(ThreadId { id: req.thread_id }),
        })
    }
}
