//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use anyhow::{bail, Result};
use base::hex_utils::hex_string;
use log::*;

const ADMIN_ACCOUNT_NAME: &str = "@admin";

impl CryptoMailService {
    /// One-time actions to perform on server start
    pub(crate) async fn boostrap() -> Result<()> {
        // todo: take this from config

        if let Some(admin_pub_key) =
            CryptoMailService::read_account_by_name(ADMIN_ACCOUNT_NAME).await?
        {
            if let Some(account) =
                CryptoMailService::load_account_from_store(&admin_pub_key).await?
            {
                info!("Admin account exists.");
                info!(
                    "Admin account pub key: {}",
                    hex_string(admin_pub_key.key.as_slice())
                );
                info!(
                    "Admin pub pre-key: {}",
                    hex_string(account.get_pre_key().as_ref().unwrap().key.as_slice())
                );

                return Ok(());
            } else {
                bail!("admin account listed by name but doesn't exist in store")
            }
        }

        info!("creating admin account named {}...", ADMIN_ACCOUNT_NAME);
        let account = CryptoMailService::create_admin_account(ADMIN_ACCOUNT_NAME).await?;
        info!(
            "🐣 Admin Account created name: {}, account pub key: {} successfully created",
            ADMIN_ACCOUNT_NAME.to_string(),
            account.get_public_key()
        );
        Ok(())
    }
}
