//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::model::api::{ArchiveThreadRequest, ArchiveThreadResponse};
use crate::model::extensions::{Signed, Validatable};
use crate::model::types::{ThreadBoxType, ThreadId};
use anyhow::{anyhow, bail, Result};
use base::hex_utils::hex_string;
use xactor::*;

impl CryptoMailService {
    pub(crate) async fn archive_thread(
        request: ArchiveThreadRequest,
    ) -> Result<ArchiveThreadResponse> {
        let service = CryptoMailService::from_registry().await?;
        service.call(ArchiveThreadMessage { request }).await?
    }
}

#[message(result = "Result<ArchiveThreadResponse>")]
pub struct ArchiveThreadMessage {
    pub request: ArchiveThreadRequest,
}

/// Remove a thread form a user's inbox and add it to the archive-box
#[async_trait::async_trait]
impl Handler<ArchiveThreadMessage> for CryptoMailService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: ArchiveThreadMessage,
    ) -> Result<ArchiveThreadResponse> {
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

        let mut archive = account
            .load_thread_box(ThreadBoxType::Archive)
            .await?
            .ok_or_else(|| anyhow!("missing archive-box"))?;

        if let Some(idx) = inbox.thread_ids.iter().position(|id| *id == req.thread_id) {
            archive.thread_ids.push(inbox.thread_ids.remove(idx))
        } else {
            bail!("invalid thread id")
        }

        account.save_thread_box(inbox).await?;
        account.save_thread_box(archive).await?;
        info!(
            "moved thread {} to archive-box",
            hex_string(req.thread_id.as_ref())
        );
        Ok(ArchiveThreadResponse {
            thread_id: Some(ThreadId { id: req.thread_id }),
        })
    }
}
