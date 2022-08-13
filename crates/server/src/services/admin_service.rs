//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain_service::BlockchainService;
use anyhow::Result;
use base::snp::snp_core_types::DialupInfo;
use base::snp::upsetter_server_admin::server_admin_service_server::ServerAdminService;
use base::snp::upsetter_server_admin::GetClientsResponse;
use tonic::{Request, Response, Status};
use xactor::*;

/// AdminService is a system service that provides access to provider server persisted data as well as an interface to admin the provider's server. It provides a GRPC admin service defined in ServerAdminService. This service is designed to be used by provider admin clients.
#[derive(Debug, Clone)]
pub(crate) struct AdminService {}

impl Default for AdminService {
    fn default() -> Self {
        debug!("AdminService started");
        AdminService {}
    }
}

#[async_trait::async_trait]
impl Actor for AdminService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("AdminService started");
        Ok(())
    }
}

impl Service for AdminService {}

/// AdminService implements the ServerAdminService trait which defines the grpc methods
/// it provides for clients over the network
#[tonic::async_trait]
impl ServerAdminService for AdminService {
    async fn set_blockchain_service(
        &self,
        request: Request<DialupInfo>,
    ) -> Result<Response<()>, Status> {
        BlockchainService::setup_blockchain_service(request.into_inner())
            .await
            .map_err(|_| Status::internal("internal error"))?;

        Ok(Response::new(()))
    }

    async fn get_clients(
        &self,
        _request: Request<()>,
    ) -> Result<Response<GetClientsResponse>, Status> {
        todo!()
    }
}
