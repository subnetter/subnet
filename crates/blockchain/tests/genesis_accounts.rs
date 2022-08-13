// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;
use anyhow::Result;
use base::snp::snp_blockchain::blockchain_service_client::BlockchainServiceClient;
use base::snp::snp_blockchain::{GetAccountRequest, SetBalanceRequest};
use base::snp::snp_payments::{Address, Amount, CoinType};
use base::test_helpers::enable_logger;
use blockchain::configure::Configure;
use blockchain::service::SimpleBlockchainService;
use blockchain::start_grpc_server::StartGrpcServer;
use db::db_service::DatabaseService;
use ed25519_dalek::Keypair;
// use std::time::Duration;
// use tokio::time::sleep;
use xactor::Service;

/// Server api test - get provider bundle public method test
#[tokio::test]
async fn genesis_accounts() {
    enable_logger();
    let server = SimpleBlockchainService::from_registry().await.unwrap();
    let server_port = 50051;

    let _ = server
        .call(StartGrpcServer {
            grpc_port: server_port,
            grpc_host: "[::1]".into(),
            server_name: "Blockchain Service".to_string(),
        })
        .await
        .unwrap();

    SimpleBlockchainService::config(Configure {}).await.unwrap();

    let mut client = BlockchainServiceClient::connect(format!("http://[::1]:{}", server_port))
        .await
        .expect("failed to connect to grpc ping service");

    // generate 2 accounts key pairs here
    let keypair1 = Keypair::generate(&mut rand_core::OsRng);
    let keypair2 = Keypair::generate(&mut rand_core::OsRng);
    let keypair3 = Keypair::generate(&mut rand_core::OsRng);

    let address1 = Address {
        data: keypair1.public.to_bytes()[12..].to_vec(),
    };

    let address2 = Address {
        data: keypair2.public.to_bytes()[12..].to_vec(),
    };

    let address3 = Address {
        data: keypair3.public.to_bytes()[12..].to_vec(),
    };

    let amount1 = 100;
    let amount2 = 200;

    client
        .set_balance(SetBalanceRequest {
            address: Some(address1.clone()),
            amount: Some(Amount {
                value: amount1,
                coin_type: CoinType::Core as i32,
            }),
        })
        .await
        .unwrap();

    client
        .set_balance(SetBalanceRequest {
            address: Some(address2.clone()),
            amount: Some(Amount {
                value: amount2,
                coin_type: CoinType::Core as i32,
            }),
        })
        .await
        .unwrap();

    let balance1 = client
        .get_account(GetAccountRequest {
            address: Some(address1.clone()),
        })
        .await
        .unwrap()
        .into_inner()
        .account
        .unwrap()
        .get_balance(CoinType::Core as i32);

    let balance2 = client
        .get_account(GetAccountRequest {
            address: Some(address2.clone()),
        })
        .await
        .unwrap()
        .into_inner()
        .account
        .unwrap()
        .get_balance(CoinType::Core as i32);

    assert_eq!(amount1, balance1);
    assert_eq!(amount2, balance2);

    let res = client
        .get_account(GetAccountRequest {
            address: Some(address3.clone()),
        })
        .await
        .unwrap()
        .into_inner()
        .account;

    assert!(res.is_none());

    test_teardown().await.unwrap();
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
