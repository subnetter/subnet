//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use cryptomail::features::eth_api_client::{ETH_TEST_ACCOUNT_1, ETH_TEST_ACCOUNT_2};
use cryptomail::model::api::cryptomail_api_service_client::CryptomailApiServiceClient;
use cryptomail::model::api::CreateAccountResult;
use cryptomail::model::extensions::Signer;
use cryptomail::tests::setup::{
    create_account_request, get_grpc_server_connection_string, test_setup, test_teardown,
};

/// Fail to create an account with a pub key of an existing account
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn new_account_existing_name() {
    test_setup().await.unwrap();

    let grpc_server_addr = get_grpc_server_connection_string().await.unwrap();
    let mut api_service =
        CryptomailApiServiceClient::connect(format!("http://{}", grpc_server_addr))
            .await
            .unwrap();

    let (mut request, key_pair, _) =
        create_account_request(ETH_TEST_ACCOUNT_1.to_string(), "account 1".to_string())
            .await
            .unwrap();
    request.sign(&key_pair).unwrap();

    let response = api_service
        .create_account(request.clone())
        .await
        .unwrap()
        .into_inner();

    assert_eq!(response.result, CreateAccountResult::Created as i32);
    assert!(response.account.is_some());

    let (mut request1, key_pair1, _) =
        create_account_request(ETH_TEST_ACCOUNT_2.to_string(), "account 2".to_string())
            .await
            .unwrap();

    let public_account_info = request1.public_account_info.as_mut().unwrap();

    // copy an existing account name into new account
    public_account_info.name = request.public_account_info.as_ref().unwrap().name.clone();
    public_account_info.sign(&key_pair1).unwrap();
    request1.sign(&key_pair1).unwrap();

    let response1 = api_service
        .create_account(request1)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(response1.result, CreateAccountResult::NameTaken as i32);
    assert!(response1.account.is_none());

    test_teardown().await.unwrap();
}
