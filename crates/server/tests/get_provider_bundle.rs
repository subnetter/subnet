// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;
use base::server_config_service::{
    ServerConfigService, GRPC_HOST_CONFIG_KEY, GRPC_SERVER_PORT_CONFIG_KEY,
};
use base::snp::snp_server_api::provider_core_service_client::ProviderCoreServiceClient;
use base::snp::snp_server_api::GetIdentityBundleRequest;
use base::test_helpers::enable_logger;
use server::server_service;
use server::server_service::{DestroyDb, ServerService, Startup};
use std::time::Duration;
use tokio::time::sleep;
use xactor::Service;

/// Server api test - get provider bundle public method test
#[tokio::test]
async fn get_provider_bundle() {
    enable_logger();

    debug!("starting grpc services...");
    let server = ServerService::from_registry().await.unwrap();
    let _ = server.call(Startup {}).await.unwrap();

    sleep(Duration::from_millis(1000)).await; // Wait for the grpc service to startup

    debug!("Connecting...");

    let grpc_host = ServerConfigService::get(GRPC_HOST_CONFIG_KEY.into())
        .await
        .unwrap()
        .unwrap();
    let grpc_port = ServerConfigService::get_u64(GRPC_SERVER_PORT_CONFIG_KEY.into())
        .await
        .unwrap()
        .unwrap();

    let mut client =
        ProviderCoreServiceClient::connect(format!("http://{}:{}", grpc_host, grpc_port))
            .await
            .expect("failed to connect to grpc ping service");

    // Get server's provider current bundle
    let opt_bundle = client
        .get_identity_bundle(GetIdentityBundleRequest {
            protocol_version: server_service::SNP_PROTOCOL_VERSION.into(),
        })
        .await
        .unwrap()
        .into_inner()
        .bundle;

    assert!(opt_bundle.is_some(), "expected provider bundle");

    // delete the db created by the server for this test
    let _ = server.call(DestroyDb).await.unwrap();
}
