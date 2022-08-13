// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;
use anyhow::Result;
use base::api_types_extensions::Signed;
use base::hex_utils::hex_string;
use base::snp::snp_blockchain::blockchain_service_client::BlockchainServiceClient;
use base::snp::snp_blockchain::transaction::Data;
use base::snp::snp_blockchain::{
    GetProviderIdentityBundleRequest, ProviderBundleTransactionData, SetBalanceRequest,
    SubmitTransactionRequest, Transaction, TransactionFee,
};
use base::snp::snp_core_types::{DialupInfo, EntityId, PrivateProviderIdentityBundle, PublicKey};
use base::snp::snp_payments::{Address, Amount, CoinType};
use base::test_helpers::enable_logger;
use blockchain::configure::Configure;
use blockchain::service::SimpleBlockchainService;
use blockchain::start_grpc_server::StartGrpcServer;
use db::db_service::DatabaseService;
use ed25519_dalek::Keypair;
use rand_core::OsRng;
// use std::time::Duration;
// use tokio::time::sleep;
use xactor::Service;

/// Server api test - get provider bundle public method test
#[tokio::test]
async fn providers_bundles() {
    enable_logger();
    let server = SimpleBlockchainService::from_registry().await.unwrap();
    let server_port = 50051;
    let _ = server
        .call(StartGrpcServer {
            grpc_port: server_port,
            grpc_host: "[::1]".to_string(),
            server_name: "blockchain service".to_string(),
        })
        .await
        .unwrap();

    SimpleBlockchainService::config(Configure {}).await.unwrap();

    let mut client = BlockchainServiceClient::connect(format!("http://[::1]:{}", server_port))
        .await
        .expect("failed to connect to grpc ping service");

    let key_pair = Keypair::generate(&mut OsRng);
    let pre_key_private = x25519_dalek::StaticSecret::new(&mut rand_core::OsRng);
    let payment_address = Address {
        data: key_pair.public.to_bytes()[12..].to_vec(),
    };

    let dialup_info = DialupInfo::new();
    let bundle = PrivateProviderIdentityBundle::new_for_id(
        &key_pair,
        &pre_key_private,
        &dialup_info,
        "provider1".to_string(),
        &payment_address,
        0,
    )
    .unwrap();

    client
        .set_balance(SetBalanceRequest {
            address: Some(payment_address.clone()),
            amount: Some(Amount {
                value: 100,
                coin_type: CoinType::Core as i32,
            }),
        })
        .await
        .unwrap();

    let bundle_data = ProviderBundleTransactionData {
        provider_bundle: Some(bundle.public_bundle.as_ref().unwrap().clone()),
    };

    let tx_fee = TransactionFee {
        amount: Some(Amount {
            value: 1,
            coin_type: CoinType::Core as i32,
        }),
        payer_public_key: vec![], // sender pays fee
    };

    // client 1 bundle submission by provider 1
    let mut tx = Transaction {
        sender_pub_key: key_pair.public.to_bytes().to_vec(),
        fee: Some(tx_fee),
        counter: 1,
        entity_id: None,
        net_id: 0,
        signature: vec![],
        data: Some(Data::ProviderBundle(bundle_data)),
        fee_signature: vec![], // sender pays fee
    };

    tx.sign(&key_pair).unwrap();
    let res = client
        .submit_transaction(SubmitTransactionRequest {
            transaction: Some(tx),
        })
        .await
        .unwrap()
        .into_inner();

    let tx_id = res.id.unwrap();
    info!("tx id: {}", hex_string(tx_id.id.as_ref()));
    let res = client
        .get_provider_identity_bundle(GetProviderIdentityBundleRequest {
            entity_id: Some(EntityId {
                public_key: Some(PublicKey {
                    key: key_pair.public.as_ref().to_vec(),
                }),
                nickname: "".to_string(),
            }),
        })
        .await
        .unwrap()
        .into_inner();

    let res_bundle = res.provider_bundle.unwrap();
    assert_eq!(
        res_bundle.address.unwrap().data,
        bundle.public_bundle.unwrap().address.unwrap().data
    );

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
