// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::features::grpc_service::BlockchainServerGrpc;
use crate::service::SimpleBlockchainService;
use anyhow::Result;
use base::snp::snp_blockchain::blockchain_service_server::BlockchainServiceServer;
use tonic::transport::Server;
use xactor::*;

#[message(result = "Result<()>")]
pub struct StartGrpcServer {
    pub grpc_port: u32,
    pub grpc_host: String,
    pub server_name: String,
}

/// Starts this blockchain service grpc server
#[async_trait::async_trait]
impl Handler<StartGrpcServer> for SimpleBlockchainService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: StartGrpcServer) -> Result<()> {
        let grpc_server_address = format!("{}:{}", msg.grpc_host, msg.grpc_port)
            .parse()
            .unwrap();

        info!(
            "starting {} api grpc server on: {}",
            msg.server_name, grpc_server_address
        );

        let service = BlockchainServiceServer::new(BlockchainServerGrpc::default())
            // accept compressed requests
            .accept_gzip()
            // compress responses, if supported by the client
            .send_gzip();

        // start health service and indicate we are serving MyMessagingService
        let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
        health_reporter
            .set_serving::<BlockchainServiceServer<BlockchainServerGrpc>>()
            .await;

        let service = tonic_web::config()
            //.allow_origins(vec!["127.0.0.1"]) // e.g. can limit to a reverse proxy that adds ssl!
            .enable(service);

        tokio::task::spawn(async move {
            let res = Server::builder()
                .accept_http1(true)
                .add_service(service)
                .add_service(health_service)
                .serve(grpc_server_address)
                .await;

            if res.is_err() {
                info!("grpc server stopped due to: {:?}", res.err().unwrap());
            } else {
                info!("grpc server stopped");
            }
        });

        Ok(())
    }
}
