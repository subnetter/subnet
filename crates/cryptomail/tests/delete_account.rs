//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use chrono::Utc;
use cryptomail::features::eth_api_client::ETH_TEST_ACCOUNT_1;
use cryptomail::model::api::cryptomail_api_service_client::CryptomailApiServiceClient;
use cryptomail::model::api::{
    CreateAccountResult, DeleteAccountRequest, GetAccountRequest, GetThreadBoxesRequest,
};
use cryptomail::model::extensions::Signer;
use cryptomail::model::types::ThreadBoxType;
use cryptomail::tests::setup::{
    create_account_request, get_grpc_server_connection_string, test_setup, test_teardown,
};

/// Basic account test - create a new account
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn delete_account() {
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
        .create_account(request.clone())
        .await
        .unwrap()
        .into_inner();

    assert_eq!(response.result, CreateAccountResult::Created as i32);
    assert!(response.account.is_some());

    // delete the account
    let mut delete_account_request = DeleteAccountRequest {
        time_stamp: Utc::now().timestamp_nanos() as u64,
        public_key: Some(account_pub_key.clone()),
        signature: vec![],
    };
    delete_account_request.sign(&key_pair).unwrap();

    api_service
        .delete_account(delete_account_request)
        .await
        .unwrap()
        .into_inner();

    // try getting thread boxes after deleting the account

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
    let response = api_service.get_thread_boxes(request).await;
    assert!(response.is_err());

    // test getting account public info by anyone by address
    use cryptomail::model::api::get_account_request::Data;
    let response = api_service
        .get_account(GetAccountRequest {
            data: Some(Data::PublicKey(account_pub_key.clone())),
        })
        .await
        .unwrap();

    assert!(response.into_inner().account.is_none());

    // test getting account info by name
    let response = api_service
        .get_account(GetAccountRequest {
            data: Some(Data::Name(account_name.clone())),
        })
        .await
        .unwrap();

    assert!(response.into_inner().account.is_none());

    test_teardown().await.unwrap();
}
