use log::*;

use crate::commands::configure::ConfigureMessage;
use crate::commands::service::CryptoMailService;
use crate::commands::start_grpc_server::StartGrpcServerMessage;

use crate::features::eth_api_client::{ETH_TEST_ACCOUNT_1, ETH_TEST_ACCOUNT_1_SIGNATURE};
use crate::model::api::CreateAccountRequest;
use crate::model::extensions::Signer;
use crate::model::types::{
    Account, Amount, EthAddress, PaidAction, PaidActionType, PaymentSettings, PreKey,
    PublicAccountInfo, PublicKey, Reputation, Settings, Token, WebResource, WebResourcesTypes,
};
use anyhow::Result;
use base::logging_service::{InitLogger, LoggingService};
use base::server_config_service::{
    ServerConfigService, GRPC_ADMIN_PORT_CONFIG_KEY, GRPC_HOST_CONFIG_KEY,
    GRPC_SERVER_PORT_CONFIG_KEY,
};
use chrono::Utc;
use crypto::utils::create_key_pair;
use db::db_service::DatabaseService;
use ed25519_dalek::Keypair;
use rand_core::OsRng;
use std::{env, fs};
use x25519_dalek::StaticSecret;
use xactor::*;

/// Create a new account with default settings - unsigned
pub async fn create_account_request(
    eth_address: String,
    name: String,
) -> Result<(CreateAccountRequest, Keypair, StaticSecret)> {
    let key_pair = create_key_pair();
    let pub_key = PublicKey {
        key: Vec::from(key_pair.public.as_ref()),
    };

    // this is user's content enc private key - should be stored in his client

    let (pre_key_private, pre_key) = PreKey::new_pre_key(0);
    // let pre_key_public = x25519_dalek::PublicKey::from(&pre_key_private);

    let open_action = PaidAction {
        paid_action_type: PaidActionType::Open as i32,
        price: Some(Amount {
            token: Token::Eth as i32,
            amount: "100000000000000000".to_string(), // 0.1 eth in wei
        }),
    };

    let reply_action = PaidAction {
        paid_action_type: PaidActionType::Open as i32,
        price: Some(Amount {
            token: Token::Eth as i32,
            amount: "200000000000000000".to_string(), // 0.2 eth in wei
        }),
    };

    let payment_settings = PaymentSettings {
        eth_address: Some(EthAddress {
            bytes: hex::decode(eth_address).unwrap(),
        }),
        paid_actions: vec![open_action, reply_action],
        eth_signature: hex::decode(ETH_TEST_ACCOUNT_1_SIGNATURE.as_bytes())?,
    };

    let mut public_account_info = PublicAccountInfo {
        public_key: Some(pub_key.clone()),
        name,
        pre_key: Some(pre_key),
        eth_name: "upsetter.eth".to_string(),
        profile: "I'm the one and only upsetter".to_string(),
        org_name: "Subnet Project".to_string(),
        position: "Chief Subnetter".to_string(),
        profile_image_url: "https://upsetter.wtf/profile.jpg".to_string(),
        small_profile_image_url: "https://upsetter.wtf/profile_small.jpg".to_string(),
        custom_profile_background_image_url: "https://upsetter.wtf/bcg1.jpg".to_string(),
        profile_urls: vec![WebResource {
            web_resource_type: WebResourcesTypes::Website as i32,
            name: "Website".to_string(),
            url: "https://linktree.xyz/upsetter".to_string(),
        }],
        payment_settings: Some(payment_settings),
        signature: vec![],
        full_name: "Joe Subnetter".to_string(),
        location: "Worldwide".to_string(),
    };
    public_account_info.sign(&key_pair)?;

    let settings = Settings {
        public_list_account: true,
        active: true,
        display_art_background: true,
    };

    Ok((
        CreateAccountRequest {
            time_stamp: Utc::now().timestamp_nanos() as u64,
            public_key: Some(pub_key),
            settings: Some(settings),
            public_account_info: Some(public_account_info),
            signature: vec![],
        },
        key_pair,
        pre_key_private,
    ))
}

/// Returns a new user account for tests
pub async fn create_new_test_account() -> Result<Account> {
    let key_pair = create_key_pair();
    let pub_key = PublicKey {
        key: Vec::from(key_pair.public.as_ref()),
    };

    let now = Utc::now().timestamp_nanos();

    // this is user's content enc private key - should be stored in his client
    let pre_key_private = StaticSecret::new(OsRng);
    let pre_key_public = x25519_dalek::PublicKey::from(&pre_key_private);

    let pre_key = PreKey {
        id: 0,
        key: pre_key_public.as_bytes().to_vec(),
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
            amount: "100000000000000000".to_string(), // 0.1 eth
        }),
    };

    let reply_action = PaidAction {
        paid_action_type: PaidActionType::Open as i32,
        price: Some(Amount {
            token: Token::Eth as i32,
            amount: "200000000000000000".to_string(), // 0.2 eth
        }),
    };

    let payment_settings = PaymentSettings {
        eth_address: Some(EthAddress {
            bytes: hex::decode(ETH_TEST_ACCOUNT_1.to_string()).unwrap(),
        }),
        paid_actions: vec![open_action, reply_action],
        eth_signature: hex::decode(ETH_TEST_ACCOUNT_1_SIGNATURE.as_bytes())?,
    };

    let mut public_account_info = PublicAccountInfo {
        public_key: Some(pub_key.clone()),
        name: "Upsetter".to_string(),
        full_name: "Joe Upsetter".to_string(),
        position: "Chief Upsetter".to_string(),
        org_name: "Upsetter Project".to_string(),
        pre_key: Some(pre_key),
        eth_name: "upsetter.eth".to_string(),
        profile: "I'm the one and only upsetter".to_string(),
        profile_image_url: "https://upsetter.wtf/profile.jpg".to_string(),
        small_profile_image_url: "https://upsetter.wtf/profile_small.jpg".to_string(),
        custom_profile_background_image_url: "https://upsetter.wtf/bcg1.jpg".to_string(),
        profile_urls: vec![WebResource {
            web_resource_type: WebResourcesTypes::Website as i32,
            name: "Website".to_string(),
            url: "https://linktree.xyz/upsetter".to_string(),
        }],
        payment_settings: Some(payment_settings),
        signature: vec![],
        location: "Worldwide".to_string(),
    };
    public_account_info.sign(&key_pair)?;

    let og_counter = CryptoMailService::get_and_update_og_counter().await?;

    Ok(Account {
        id_pub_key: Some(pub_key),
        reputation: Some(Reputation::new_account_reputation(og_counter)),
        time_created: now as u64,
        time_last_login: now as u64,
        settings: Some(settings),
        public_account_info: Some(public_account_info),
    })
}

pub async fn get_admin_grpc_server_connection_string() -> Result<String> {
    let grpc_host = ServerConfigService::get(GRPC_HOST_CONFIG_KEY.into())
        .await?
        .unwrap();

    let grpc_port = ServerConfigService::get_u64(GRPC_ADMIN_PORT_CONFIG_KEY.into())
        .await?
        .unwrap();

    Ok(format!("{}:{}", grpc_host, grpc_port))
}

pub async fn get_grpc_server_connection_string() -> Result<String> {
    let grpc_host = ServerConfigService::get(GRPC_HOST_CONFIG_KEY.into())
        .await?
        .unwrap();

    let grpc_port = ServerConfigService::get_u64(GRPC_SERVER_PORT_CONFIG_KEY.into())
        .await?
        .unwrap();

    Ok(format!("{}:{}", grpc_host, grpc_port))
}

/// Inits logging and starts the CryptoMailService grpc server
pub async fn test_setup() -> Result<()> {
    // Start app logging
    let logging = LoggingService::from_registry().await?;
    let _ = logging
        .call(InitLogger {
            peer_name: "CMService".into(),
            brief: false, // todo: take from config
        })
        .await?;

    info!("test starting...");

    // Start and configure the CryptoMail service
    let service = CryptoMailService::from_registry().await?;
    CryptoMailService::config(ConfigureMessage {})
        .await
        .unwrap();

    // start the grpc server
    let _ = service.call(StartGrpcServerMessage {}).await?;

    // delete the local test db
    let curr_dir = env::current_dir().unwrap();
    let db_path = curr_dir.join("upsetter_db");

    // remove the db directory
    if db_path.exists() {
        info!("deleting db at: {}", db_path.to_str().unwrap());
        let _ = fs::remove_dir_all(db_path);
    }

    // use std::time::Duration;
    // use tokio::time::sleep;
    // sleep(Duration::from_secs(4)).await;

    Ok(())
}

// Gracefully shutdown the db so it is deleted if it is configured to be deleted when stopped
pub async fn test_teardown() -> Result<()> {
    tokio::task::spawn(async {
        // stop the db service so it has a chance to destroy itself if it is configured to destroy storage on stop...
        let mut db_service = DatabaseService::from_registry().await.unwrap();
        let _ = db_service.stop(None);
        info!("resources cleanup completed");
    })
    .await
    .unwrap();
    Ok(())
}
