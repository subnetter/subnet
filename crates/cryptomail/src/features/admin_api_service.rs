//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::model::api::cryptomail_admin_api_service_server::CryptomailAdminApiService;
use crate::model::api::*;
use anyhow::Result;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub(crate) struct CryptoMailAdminGrpcService {}

impl Default for CryptoMailAdminGrpcService {
    fn default() -> Self {
        debug!("CryptoMailAdminGrpcService started");
        CryptoMailAdminGrpcService {}
    }
}

/// Public api
#[tonic::async_trait]
impl CryptomailAdminApiService for CryptoMailAdminGrpcService {
    /// Return all accounts
    async fn get_accounts(
        &self,
        request: Request<GetAccountsRequest>,
    ) -> Result<Response<GetAccountsResponse>, Status> {
        let resp = CryptoMailService::all_accounts(request.into_inner())
            .await
            .map_err(|e| Status::internal(format!("server error: {}", e)))?;

        Ok(Response::new(resp))
    }

    /// Get data about an account
    async fn get_account_data(
        &self,
        request: Request<GetAccountDataRequest>,
    ) -> Result<Response<GetAccountDataResponse>, Status> {
        let resp = CryptoMailService::admin_account_info(request.into_inner())
            .await
            .map_err(|e| Status::internal(format!("server error: {}", e)))?;

        Ok(Response::new(resp))
    }
}
