//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::consts::SIGN_UP_TOKENS_AMOUNT;
use crate::features::eth_api_client::{KOVAN_ADMIN_ETH_ACCOUNT, KOVAN_ADMIN_ETH_ACCOUNT_SIGNATURE};
use crate::model::extensions::Signer;
use crate::model::types::{
    Account, Amount, EthAddress, PaidAction, PaidActionType, PaymentSettings, PreKey,
    PublicAccountInfo, PublicKey, Reputation, Settings, ThreadBox, ThreadBoxType, Token,
    WebResource, WebResourcesTypes,
};
use anyhow::{anyhow, Result};
use base::hex_utils::hex_string;
use chrono::Utc;
use ed25519_dalek::{Keypair, SecretKey};
use x25519_dalek::StaticSecret;

const _ADMIN_ACCOUNT_NAME: &str = "@admin";

impl CryptoMailService {
    /// Get the admin account - there should always be one unless of an internal error
    pub(crate) async fn _admin_account() -> Result<Account> {
        let address = CryptoMailService::read_account_by_name(_ADMIN_ACCOUNT_NAME)
            .await?
            .ok_or_else(|| anyhow!("can't find admin account address by name"))?;

        CryptoMailService::load_account_from_store(&address)
            .await?
            .ok_or_else(|| anyhow!("can't find admin account"))
    }

    /// Get the admin account key-pair
    pub(crate) fn admin_account_key_pair() -> Keypair {
        // todo: move to config
        let secret_key_bytes = hex::decode(
            "b5fd91f620dbe429958850b997deaacad7269858ab1b3a38dd7f694a94f17bc2".to_string(),
        )
        .unwrap();
        let secret_key = SecretKey::from_bytes(secret_key_bytes.as_slice()).unwrap();
        let public_key: ed25519_dalek::PublicKey = (&secret_key).into();

        info!(
            "Admin account private key: {}",
            hex_string(secret_key_bytes.as_slice())
        );
        info!(
            "Admin account public key: {}",
            hex_string(public_key.as_ref())
        );

        Keypair {
            secret: secret_key,
            public: public_key,
        }
    }

    pub(crate) fn admin_account_pre_key_pair() -> (StaticSecret, x25519_dalek::PublicKey) {
        // todo: move to config
        let pre_key_seed = hex::decode(
            "78626d407a64fd97d48f9490b3cd4b5cdd5674d96579736e9a39274113137e53".to_string(),
        )
        .unwrap();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(pre_key_seed.as_slice());

        info!("Seed: {}", hex_string(bytes.as_ref()));

        let pre_key_private = StaticSecret::from(bytes);
        info!(
            "Admin pre-key private: {}",
            hex_string(pre_key_private.to_bytes().as_ref())
        );
        let pre_key_public = x25519_dalek::PublicKey::from(&pre_key_private);
        info!(
            "Admin pre-key public: {}",
            hex_string(pre_key_public.as_bytes().as_ref())
        );

        (pre_key_private, pre_key_public)
    }

    /// Create the admin account
    pub(crate) async fn create_admin_account(account_name: &str) -> Result<Account> {
        let key_pair = CryptoMailService::admin_account_key_pair();

        let pub_key = PublicKey {
            key: Vec::from(key_pair.public.as_ref()),
        };

        let now = Utc::now().timestamp_nanos();

        let pre_key_pair = CryptoMailService::admin_account_pre_key_pair();
        let pre_key = PreKey {
            id: 0,
            key: pre_key_pair.1.as_bytes().to_vec(),
        };

        let settings = Settings {
            public_list_account: true,
            active: true,
            display_art_background: true,
        };

        let open_action = PaidAction {
            paid_action_type: PaidActionType::Open as i32,
            price: Some(Amount {
                token: Token::Eth as i32,
                amount: "100000000000000".to_string(), // 0.0001 eth in wei
            }),
        };

        let reply_action = PaidAction {
            paid_action_type: PaidActionType::Reply as i32,
            price: Some(Amount {
                token: Token::Eth as i32,
                amount: "200000000000000".to_string(), // 0.0002 eth in wei
            }),
        };

        let payment_settings = PaymentSettings {
            eth_address: Some(EthAddress {
                bytes: hex::decode(KOVAN_ADMIN_ETH_ACCOUNT.to_string()).unwrap(),
            }),
            paid_actions: vec![open_action, reply_action],
            eth_signature: hex::decode(KOVAN_ADMIN_ETH_ACCOUNT_SIGNATURE.as_bytes())?,
        };

        // profile_image_url: "https://lh3.googleusercontent.com/SJ4Jo_sAShY7iIhlfdIWW8RcosTokINB5r1JquW1SPGtnPyQgBqEyB9pzHUq4_6eRleMamRc28EUOwy9bwshNaRmeNTADqt9aLKtyw=w800".to_string(),
        //             small_profile_image_url: "https://lh3.googleusercontent.com/SJ4Jo_sAShY7iIhlfdIWW8RcosTokINB5r1JquW1SPGtnPyQgBqEyB9pzHUq4_6eRleMamRc28EUOwy9bwshNaRmeNTADqt9aLKtyw=w200".to_string(),
        //

        let mut public_account_info = PublicAccountInfo {
            public_key: Some(pub_key.clone()),
            name: account_name.to_string(),
            full_name: "Cmail Admin".to_string(),
            org_name: "Cmail DAO".to_string(),
            position: "Community Coordinator".to_string(),
            location: "Worldwide".to_string(),
            pre_key: Some(pre_key),
            eth_name: "cmail.eth".to_string(),
            profile: "I'm the cmail alpha community coordinator".to_string(),
            profile_image_url: "https://cmail.wtf/admin_profile_large.jpg".to_string(),
            small_profile_image_url: "https://cmail.wtf/admin_profile_small.jpg".to_string(),

            custom_profile_background_image_url: "https://upsetter.wtf/bcg1.jpg".to_string(),
            profile_urls: vec![
                WebResource {
                    web_resource_type: WebResourcesTypes::Website as i32,
                    name: "Website".to_string(),
                    url: "https://cmail.wtf".to_string(),
                },
                WebResource {
                    web_resource_type: WebResourcesTypes::Twitter as i32,
                    name: "Twitter".to_string(),
                    url: "@cmwdev2".to_string(),
                },
                /*
                WebResource {
                    web_resource_type: WebResourcesTypes::Telegram as i32,
                    name: "Telegram ".to_string(),
                    url: "@subnetter1".to_string(),
                },
                WebResource {
                    web_resource_type: WebResourcesTypes::Linkedin as i32,
                    name: "LinkedIn ".to_string(),
                    url: "@subnetter123".to_string(),
                },*/
            ],
            payment_settings: Some(payment_settings),
            signature: vec![],
        };

        public_account_info.sign(&key_pair)?;

        let og_counter = CryptoMailService::get_and_update_og_counter().await?;

        let mut account = Account {
            id_pub_key: Some(pub_key),
            reputation: Some(Reputation::new_account_reputation(og_counter)),
            time_created: now as u64,
            time_last_login: now as u64,
            settings: Some(settings),
            public_account_info: Some(public_account_info),
        };

        account.add_cmail_tokens(SIGN_UP_TOKENS_AMOUNT)?;

        CryptoMailService::store_account(&account).await?;

        // create thread boxes
        let mut threads_boxes = vec![];

        let inbox = ThreadBox {
            thread_box_type: ThreadBoxType::Inbox as i32,
            thread_ids: vec![],
        };
        threads_boxes.push(inbox.clone());
        account.save_thread_box(inbox).await?;

        let sent = ThreadBox {
            thread_box_type: ThreadBoxType::Sent as i32,
            thread_ids: vec![],
        };
        threads_boxes.push(sent.clone());
        account.save_thread_box(sent).await?;

        let archive = ThreadBox {
            thread_box_type: ThreadBoxType::Archive as i32,
            thread_ids: vec![],
        };
        threads_boxes.push(archive.clone());
        account.save_thread_box(archive).await?;

        let account_pub_key = account.get_public_key();
        CryptoMailService::store_account_by_name(&account_name, account_pub_key).await?;
        CryptoMailService::public_list_account(&account_name, account_pub_key).await?;

        Ok(account)
    }
}
