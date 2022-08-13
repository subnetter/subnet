//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use chrono::Utc;
use cryptomail::features::eth_api_client::ETH_TEST_ACCOUNT_1;
use cryptomail::model::api::cryptomail_api_service_client::CryptomailApiServiceClient;
use cryptomail::model::api::{CreateAccountResult, GetAccountRequest, GetThreadBoxesRequest};
use cryptomail::model::extensions::{Signed, Signer};
use cryptomail::model::types::ThreadBoxType;
use cryptomail::tests::setup::{
    create_account_request, get_grpc_server_connection_string, test_setup, test_teardown,
};

/// Basic account test - create a new account
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn new_account() {
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

    let account_pub_key = request.public_key.as_ref().unwrap().clone();
    let account_name = request.public_account_info.as_ref().unwrap().name.clone();

    let response = api_service
        .create_account(request)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(response.result, CreateAccountResult::Created as i32);
    assert!(response.account.is_some());

    // thread_boxes bitmask
    let thread_boxes =
        ThreadBoxType::Inbox as u32 | ThreadBoxType::Sent as u32 | ThreadBoxType::Archive as u32;

    // test getting account thread-boxes by account owner
    let mut request = GetThreadBoxesRequest {
        time_stamp: Utc::now().timestamp_nanos() as u64,
        public_key: Some(account_pub_key.clone()),
        thread_boxes,
        signature: vec![],
    };
    request.sign(&key_pair).unwrap();

    let response = api_service
        .get_thread_boxes(request)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(response.threads_boxes.len(), 3);

    // test getting account public info by anyone

    use cryptomail::model::api::get_account_request::Data;
    let resp = api_service
        .get_account(GetAccountRequest {
            data: Some(Data::PublicKey(account_pub_key.clone())),
        })
        .await
        .unwrap()
        .into_inner();

    let info = resp.account.unwrap().public_account_info.unwrap();
    info.validate().await.unwrap();
    info.verify_signature().unwrap();

    assert_eq!(
        info.public_key.unwrap().key,
        account_pub_key.key,
        "unexpected account info"
    );
    assert_eq!(info.name, account_name, "unexpected account name");

    // test getting account info by name

    let resp = api_service
        .get_account(GetAccountRequest {
            data: Some(Data::Name(account_name.clone())),
        })
        .await
        .unwrap()
        .into_inner();

    let info = resp.account.unwrap().public_account_info.unwrap();
    info.validate().await.unwrap();
    info.verify_signature().unwrap();

    assert_eq!(
        info.public_key.unwrap().key,
        account_pub_key.key,
        "unexpected account info"
    );
    assert_eq!(info.name, account_name, "unexpected account name");

    test_teardown().await.unwrap();
}
