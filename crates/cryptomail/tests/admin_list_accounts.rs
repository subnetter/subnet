//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use cryptomail::features::eth_api_client::{ETH_TEST_ACCOUNT_1, ETH_TEST_ACCOUNT_2};
use cryptomail::model::api::cryptomail_admin_api_service_client::CryptomailAdminApiServiceClient;
use cryptomail::model::api::cryptomail_api_service_client::CryptomailApiServiceClient;
use cryptomail::model::api::get_account_data_request::Data::PublicKey;
use cryptomail::model::api::{CreateAccountResult, GetAccountDataRequest, GetAccountsRequest};
use cryptomail::model::extensions::Signer;
use cryptomail::tests::setup::{
    create_account_request, get_admin_grpc_server_connection_string,
    get_grpc_server_connection_string, test_setup, test_teardown,
};
use log::*;

/// Admin should be able to list all accounts
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn admin_accounts() {
    test_setup().await.unwrap();

    let grpc_server_addr = get_grpc_server_connection_string().await.unwrap();
    let mut api_service =
        CryptomailApiServiceClient::connect(format!("http://{}", grpc_server_addr))
            .await
            .unwrap();

    // account 1
    let (mut create_account1, key_pair, _) =
        create_account_request(ETH_TEST_ACCOUNT_1.to_string(), "account1".to_string())
            .await
            .unwrap();
    create_account1.sign(&key_pair).unwrap();
    let response = api_service
        .create_account(create_account1)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(response.result, CreateAccountResult::Created as i32);
    assert!(response.account.is_some());

    // account 2

    let (mut create_account2, account2_key_pair, _) =
        create_account_request(ETH_TEST_ACCOUNT_2.to_string(), "account 2".to_string())
            .await
            .unwrap();
    create_account2.sign(&account2_key_pair).unwrap();
    let response = api_service
        .create_account(create_account2)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(response.result, CreateAccountResult::Created as i32);
    assert!(response.account.is_some());

    // account 3

    let (mut create_account3, account_3_key_pair, _) =
        create_account_request(ETH_TEST_ACCOUNT_2.to_string(), "account 3".to_string())
            .await
            .unwrap();
    create_account3.sign(&account_3_key_pair).unwrap();
    let response = api_service
        .create_account(create_account3)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(response.result, CreateAccountResult::Created as i32);
    assert!(response.account.is_some());

    // account 4 - a private account

    let (mut create_account4, account_4_key_pair, _) =
        create_account_request(ETH_TEST_ACCOUNT_2.to_string(), "account 4".to_string())
            .await
            .unwrap();

    create_account4
        .settings
        .as_mut()
        .unwrap()
        .public_list_account = false;
    create_account4.sign(&account_4_key_pair).unwrap();
    let response = api_service
        .create_account(create_account4)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(response.result, CreateAccountResult::Created as i32);
    assert!(response.account.is_some());

    ////// Get all accounts and check expected results length

    let grpc_admin_server_addr = get_admin_grpc_server_connection_string().await.unwrap();
    let mut admin_api_service =
        CryptomailAdminApiServiceClient::connect(format!("http://{}", grpc_admin_server_addr))
            .await
            .unwrap();

    let response = admin_api_service
        .get_accounts(GetAccountsRequest {
            from: "".to_string(),
            max_results: 0,
        })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(response.accounts.len(), 5);

    info!("All accounts (including private");

    for account in response.accounts {
        info!("Account: {}", account);

        let response = admin_api_service
            .get_account_data(GetAccountDataRequest {
                data: Some(PublicKey(account.get_public_key().clone())),
            })
            .await
            .unwrap()
            .into_inner();
        assert!(response.account.is_some());
        assert_eq!(
            response.thread_boxes.len(),
            3,
            "expected thread boxes for each account"
        );
    }

    test_teardown().await.unwrap();
}
