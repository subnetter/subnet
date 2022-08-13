//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::consts::SIGN_UP_TOKENS_AMOUNT;
use crate::model::api::{CreateAccountRequest, CreateAccountResponse, CreateAccountResult};
use crate::model::extensions::{Signed, Validatable};
use crate::model::types::{Account, Reputation, ThreadBox, ThreadBoxType};
use anyhow::{anyhow, bail, Result};
use chrono::Utc;
use xactor::*;

impl CryptoMailService {
    // actor handler call wrapper
    pub(crate) async fn create_account(msg: CreateAccountMessage) -> Result<CreateAccountResponse> {
        let service = CryptoMailService::from_registry().await?;
        service.call(msg).await?
    }
}

/////////////////////

#[message(result = "Result<CreateAccountResponse>")]
pub(crate) struct CreateAccountMessage {
    pub(crate) request: CreateAccountRequest,
}

#[async_trait::async_trait]
impl Handler<CreateAccountMessage> for CryptoMailService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: CreateAccountMessage,
    ) -> Result<CreateAccountResponse> {
        let r = msg.request;
        r.validate()?;
        r.verify_signature()?;

        let pub_key = r.public_key.unwrap();
        pub_key.validate()?;

        if CryptoMailService::load_account_from_store(&pub_key)
            .await?
            .is_some()
        {
            info!(
                "can't create new account - account exists with public key: {}",
                pub_key
            );

            return Ok(CreateAccountResponse {
                account: None,
                result: CreateAccountResult::Exists as i32,
            });
        }

        let settings = r.settings.unwrap();
        settings
            .validate()
            .map_err(|e| anyhow!(format!("invalid settings: {}", e)))?;

        //let payment_settings = r.public_account_info.unwrap().payment_settings.unwrap()
        // todo: verify user payment settings - at least price per open and reply

        let pub_listing = settings.public_list_account;
        let public_account_info = r.public_account_info.unwrap();
        public_account_info.validate().await?;
        public_account_info.verify_signature()?;

        if public_account_info.public_key.as_ref().unwrap().key != pub_key.key {
            bail!("public keys mismatch - pub key in account info must be same pub key in requests")
        }

        let account_name = public_account_info.name.clone();

        info!(
            "creating account name: {}, pub_key: {}",
            account_name, pub_key
        );

        /*
        payment_settings
            .validate(&account_name)
            .map_err(|e| anyhow!(format!("invalid payment settings: {}", e)))?;
        */

        if CryptoMailService::read_account_by_name(&account_name)
            .await?
            .is_some()
        {
            info!("Can't create account - Name {} taken", account_name);
            return Ok(CreateAccountResponse {
                account: None,
                result: CreateAccountResult::NameTaken as i32,
            });
        }

        let now = Utc::now().timestamp_nanos();

        let og_counter = CryptoMailService::get_and_update_og_counter().await?;

        // write the account to store
        let mut account = Account {
            id_pub_key: Some(pub_key.clone()),
            reputation: Some(Reputation::new_account_reputation(og_counter)),
            time_created: now as u64,
            time_last_login: now as u64,
            settings: Some(settings),
            public_account_info: Some(public_account_info),
        };

        // tokens for sign-up
        account.add_cmail_tokens(SIGN_UP_TOKENS_AMOUNT)?;

        CryptoMailService::store_account(&account).await?;

        // Register the account by name in the name registry
        CryptoMailService::store_account_by_name(&account_name, &pub_key).await?;

        if pub_listing {
            info!("publicly listing account");
            CryptoMailService::public_list_account(&account_name, &pub_key).await?;
        }

        // create thread boxes
        let inbox = ThreadBox::new(ThreadBoxType::Inbox);
        account.save_thread_box(inbox).await?;

        let sent = ThreadBox::new(ThreadBoxType::Sent);
        account.save_thread_box(sent).await?;

        let archive = ThreadBox::new(ThreadBoxType::Archive);
        account.save_thread_box(archive).await?;

        CryptoMailService::create_inbox_with_welcome_message(&account).await?;

        info!(
            "🐣 Account name:{}, pub_key:{} successfully created and welcome message sent",
            account_name, pub_key
        );

        return Ok(CreateAccountResponse {
            account: Some(account),
            result: CreateAccountResult::Created as i32,
        });
    }
}
