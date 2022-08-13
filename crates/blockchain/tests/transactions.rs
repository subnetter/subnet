// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;
use anyhow::Result;
use base::api_types_extensions::Signed;
use base::snp::snp_blockchain::blockchain_service_client::BlockchainServiceClient;
use base::snp::snp_blockchain::transaction::Data;
use base::snp::snp_blockchain::{
    GetAccountRequest, GetTransactionRequest, PaymentTransactionData, SetBalanceRequest,
    SubmitTransactionRequest, Transaction, TransactionFee, TransactionState, TransactionType,
};
use base::snp::snp_payments::{Address, Amount, CoinType};
use base::test_helpers::enable_logger;
use blockchain::configure::Configure;
use blockchain::service::SimpleBlockchainService;
use blockchain::start_grpc_server::StartGrpcServer;
use db::db_service::DatabaseService;
use ed25519_dalek::Keypair;
use xactor::Service;

use base::hex_utils::hex_string;

/// Server api test - get provider bundle public method test
#[tokio::test]
async fn coin_transactions() {
    enable_logger();
    let server_port = 50051;
    let server = SimpleBlockchainService::from_registry().await.unwrap();
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

    let _address3 = Address {
        data: keypair3.public.to_bytes()[12..].to_vec(),
    };

    let amount1 = 100;
    let amount2 = 200;
    let tx_amount = 10;
    let tx_fee_amount = 1;

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

    // payment transaction from address 1 to 2
    let payment_data = PaymentTransactionData {
        receiver: Some(address2.clone()),
        coins: Some(Amount {
            value: tx_amount,
            coin_type: CoinType::Core as i32,
        }),
        id: 0,
    };

    let tx_fee = TransactionFee {
        amount: Some(Amount {
            value: tx_fee_amount,
            coin_type: CoinType::Core as i32,
        }),
        payer_public_key: vec![], // sender pays fee
    };

    // coin tx from account 1 to account 2
    let mut tx = Transaction {
        sender_pub_key: keypair1.public.to_bytes().to_vec(),
        fee: Some(tx_fee),
        counter: 1,
        entity_id: None,
        net_id: 0,
        signature: vec![],
        data: Some(Data::PaymentTransaction(payment_data)),
        fee_signature: vec![], // sender pays fee
    };

    tx.sign(&keypair1).unwrap();

    let res = client
        .submit_transaction(SubmitTransactionRequest {
            transaction: Some(tx),
        })
        .await
        .unwrap()
        .into_inner();

    let tx_id = res.id.unwrap();

    info!("tx id: {}", hex_string(tx_id.id.as_ref()));

    let tx_info = client
        .get_transaction(GetTransactionRequest {
            id: Some(tx_id.clone()),
        })
        .await
        .unwrap()
        .into_inner()
        .transaction_info
        .unwrap();

    assert_eq!(tx_info.state, TransactionState::Confirmed as i32);
    assert_eq!(tx_info.transaction_type, TransactionType::SendCoin as i32);
    assert_eq!(tx_info.id.unwrap().id, tx_id.id);

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

    assert_eq!(balance1, amount1 - tx_amount - tx_fee_amount);
    assert_eq!(balance2, amount2 + tx_amount);

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
