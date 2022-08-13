//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::model::api::{UpdateSettingsRequest, UpdateSettingsResponse};
use crate::model::extensions::Signed;
use anyhow::{bail, Result};
use xactor::*;

impl CryptoMailService {
    /// Update account settings
    pub(crate) async fn update_settings(
        request: UpdateSettingsRequest,
    ) -> Result<UpdateSettingsResponse> {
        let service = CryptoMailService::from_registry().await?;
        service.call(UpdateSettingsMessage { request }).await?
    }
}

#[message(result = "Result<UpdateSettingsResponse>")]
pub struct UpdateSettingsMessage {
    pub request: UpdateSettingsRequest,
}

/// Update account settings
#[async_trait::async_trait]
impl Handler<UpdateSettingsMessage> for CryptoMailService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: UpdateSettingsMessage,
    ) -> Result<UpdateSettingsResponse> {
        let req = msg.request;
        req.validate().await?;
        req.verify_signature()?;

        let pub_key = req.public_key.as_ref().unwrap();

        if let Some(mut account) = CryptoMailService::load_account_from_store(pub_key).await? {
            // read account props before update
            let old_name = account.get_name();
            let curr_pub_listing_settings = account.get_public_listing();
            let new_pub_listing_settings = req.settings.as_ref().unwrap().public_list_account;

            info!(
                "account public listing before update: {}, requested pub listing: {} ",
                curr_pub_listing_settings, new_pub_listing_settings,
            );

            // update the account info and settings
            account.public_account_info = req.public_account_info;
            account.settings = req.settings;

            // handle account name change - bail if user wants to rename account and there's already an account with this name
            let new_name = account.get_name();
            if new_name != old_name {
                CryptoMailService::update_account_name(
                    old_name.as_str(),
                    new_name.as_str(),
                    pub_key,
                )
                .await?;
            }

            // handle public listing change
            if curr_pub_listing_settings != account.get_public_listing() {
                info!(
                    "Updating public listings to: {}",
                    account.get_public_listing()
                );
                account.update_public_listing().await?;
            }

            // only store account once all validation and updates are done
            CryptoMailService::store_account(&account).await?;
            info!("updated account settings");
        } else {
            bail!("account not recognized")
        }

        Ok(UpdateSettingsResponse {})
    }
}
