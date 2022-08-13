//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use cryptomail::features::eth_api_client::{ETH_TEST_ACCOUNT_1, ETH_TEST_ACCOUNT_2};
use cryptomail::model::api::cryptomail_api_service_client::CryptomailApiServiceClient;
use cryptomail::model::api::{CreateAccountResult, GetPublicAccountsRequest};
use cryptomail::model::extensions::Signer;
use cryptomail::tests::setup::{
    create_account_request, get_grpc_server_connection_string, test_setup, test_teardown,
};
use log::*;

/// Public account listing api test for anyone
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn get_public_accounts() {
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

    ////// Get public accounts and check expected results length

    let response = api_service
        .get_public_accounts(GetPublicAccountsRequest {
            from: "".to_string(),
            max_results: 0,
        })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(response.accounts.len(), 4);

    // todo: change a public account to private and count again public listed accounts...

    for account in response.accounts {
        info!("Account: {}", account)
    }

    test_teardown().await.unwrap();
}
