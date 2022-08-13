// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::features::admin_api_service::CryptoMailAdminGrpcService;
use crate::features::api_service::CryptoMailGrpcService;
use crate::model::api::cryptomail_admin_api_service_server::CryptomailAdminApiServiceServer;
use crate::model::api::cryptomail_api_service_server::CryptomailApiServiceServer;
use anyhow::Result;
use base::server_config_service::{
    ServerConfigService, GRPC_ADMIN_PORT_CONFIG_KEY, GRPC_HOST_CONFIG_KEY,
    GRPC_SERVER_PORT_CONFIG_KEY, PEER_NAME_CONFIG_KEY, START_GRPC_SERVER_ADMIN_SERVICE_CONFIG_KEY,
};
use tonic::transport::Server;
use xactor::*;

#[message(result = "Result<()>")]
pub struct StartGrpcServerMessage {}

/// Configure and start the api and and the admin api grpc services
#[async_trait::async_trait]
impl Handler<StartGrpcServerMessage> for CryptoMailService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: StartGrpcServerMessage,
    ) -> Result<()> {
        // setup api grpc server and services
        let server_name = ServerConfigService::get(PEER_NAME_CONFIG_KEY.into())
            .await?
            .unwrap();

        let grpc_host = ServerConfigService::get(GRPC_HOST_CONFIG_KEY.into())
            .await?
            .unwrap();

        let grpc_port = ServerConfigService::get_u64(GRPC_SERVER_PORT_CONFIG_KEY.into())
            .await?
            .unwrap();

        let grpc_server_address = format!("{}:{}", grpc_host, grpc_port).parse().unwrap();
        info!(
            "starting {} api grpc server on: {}",
            server_name, grpc_server_address
        );

        // todo: add health service

        tokio::task::spawn(async move {
            let service = CryptomailApiServiceServer::new(CryptoMailGrpcService::default())
                // accept compressed requests
                .accept_gzip()
                // compress responses, if supported by the client
                .send_gzip();

            // start health service and indicate we are serving MyMessagingService
            let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
            health_reporter
                .set_serving::<CryptomailApiServiceServer<CryptoMailGrpcService>>()
                .await;

            let service = tonic_web::config()
                //.allow_origins(vec!["127.0.0.1"]) // e.g. can limit to a reverse proxy that adds ssl!
                .enable(service);

            let res = Server::builder()
                .accept_http1(true)
                .add_service(service)
                .add_service(health_service)
                .serve(grpc_server_address)
                .await;

            if res.is_err() {
                info!("api grpc server stopped due to: {:?}", res.err().unwrap());
            } else {
                info!("api grpc server stopped");
            }
        });

        // setup admin grpc server and services

        let start_admin =
            ServerConfigService::get_bool(START_GRPC_SERVER_ADMIN_SERVICE_CONFIG_KEY.into())
                .await?
                .unwrap();

        if start_admin {
            let grpc_port = ServerConfigService::get_u64(GRPC_ADMIN_PORT_CONFIG_KEY.into())
                .await?
                .unwrap();

            let grpc_admin_address = format!("{}:{}", grpc_host, grpc_port).parse().unwrap();
            info!(
                "starting {} admin api grpc server on: {}",
                server_name, grpc_admin_address
            );

            // todo: add health service

            tokio::task::spawn(async move {
                let service =
                    CryptomailAdminApiServiceServer::new(CryptoMailAdminGrpcService::default())
                        // accept compressed requests
                        .accept_gzip()
                        // compress responses, if supported by the client
                        .send_gzip();

                let service = tonic_web::config()
                    .allow_origins(vec!["127.0.0.1"]) // allow only local origin for admin api over http
                    .enable(service);

                let res = Server::builder()
                    .accept_http1(true)
                    .add_service(service)
                    .serve(grpc_admin_address)
                    .await;

                if res.is_err() {
                    info!("admin grpc server stopped due to: {:?}", res.err().unwrap());
                } else {
                    info!("admin grpc server stopped");
                }
            });
            info!("admin grpc server starting...")
        } else {
            info!("admin grpc server not started")
        }

        Ok(())
    }
}
