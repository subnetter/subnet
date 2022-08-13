// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::configure::Configure;
use crate::start_grpc_server::StartGrpcServer;
use anyhow::Result;
use xactor::*;

/// A simple SNP blockchain service mock.
/// This api should be provided by Cryptocurrency Nodes.
pub struct SimpleBlockchainService {
    // todo: add tx pool here
}

// Public service convenience wrappers
impl SimpleBlockchainService {
    pub async fn config(config: Configure) -> Result<()> {
        let service = SimpleBlockchainService::from_registry().await?;
        service.call(config).await?
    }

    pub async fn start_grpc_server(params: StartGrpcServer) -> Result<()> {
        let service = SimpleBlockchainService::from_registry().await?;
        service.call(params).await?
    }
}

#[async_trait::async_trait]
impl Actor for SimpleBlockchainService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("BlockchainService starting...");
        Ok(())
    }

    async fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("BlockchainService stopped");
    }
}

impl Service for SimpleBlockchainService {}
impl Default for SimpleBlockchainService {
    fn default() -> Self {
        SimpleBlockchainService {}
    }
}
