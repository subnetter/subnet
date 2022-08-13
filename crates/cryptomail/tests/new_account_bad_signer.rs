//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use cryptomail::features::eth_api_client::ETH_TEST_ACCOUNT_1;
use cryptomail::model::api::cryptomail_api_service_client::CryptomailApiServiceClient;
use cryptomail::tests::setup::{
    create_account_request, get_grpc_server_connection_string, test_setup, test_teardown,
};

/// Fail to create an account with a badly signed request
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn new_account_bad_signature() {
    test_setup().await.unwrap();

    let grpc_server_addr = get_grpc_server_connection_string().await.unwrap();
    let mut api_service =
        CryptomailApiServiceClient::connect(format!("http://{}", grpc_server_addr))
            .await
            .unwrap();

    let (mut request, _, _) =
        create_account_request(ETH_TEST_ACCOUNT_1.to_string(), "account 1".to_string())
            .await
            .unwrap();

    let mut buf = [0u8; 32];
    getrandom::getrandom(&mut buf).unwrap();
    request.signature = buf.to_vec();

    let result = api_service.create_account(request).await;

    assert!(result.is_err());

    test_teardown().await.unwrap();
}
