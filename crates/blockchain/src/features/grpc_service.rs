// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::service::SimpleBlockchainService;
use anyhow::Result;
use base::snp::snp_blockchain::blockchain_service_server::BlockchainService;
use base::snp::snp_blockchain::{
    GetAccountRequest, GetAccountResponse, GetBlockRequest, GetBlockResponse,
    GetBlocksCountByEntityRequest, GetBlocksCountByEntityResponse, GetClientIdentityBundleRequest,
    GetClientIdentityBundleResponse, GetClientsRequest, GetClientsResponse, GetCurrentBlockRequest,
    GetProviderIdentityBundleRequest, GetProviderIdentityBundleResponse, GetProvidersRequest,
    GetProvidersResponse, GetTransactionRequest, GetTransactionResponse, SetBalanceRequest,
    SetBalanceResponse, SubmitTransactionRequest, SubmitTransactionResponse,
};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub(crate) struct BlockchainServerGrpc {}

impl Default for BlockchainServerGrpc {
    fn default() -> Self {
        debug!("Blockchain grpc started");
        BlockchainServerGrpc {}
    }
}

#[tonic::async_trait]
impl BlockchainService for BlockchainServerGrpc {
    /// Submit a transaction for processing
    async fn submit_transaction(
        &self,
        msg: Request<SubmitTransactionRequest>,
    ) -> Result<Response<SubmitTransactionResponse>, Status> {
        match SimpleBlockchainService::submit_transaction(msg.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("submit tx error: {:?}", e);
                Err(Status::internal(format!("submit tx error: {:?}", e)))
            }
        }
    }

    /// Sets the balance of an account - genesis config by consensus
    async fn set_balance(
        &self,
        request: Request<SetBalanceRequest>,
    ) -> Result<Response<SetBalanceResponse>, Status> {
        match SimpleBlockchainService::set_balance(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("set balance error: {:?}", e);
                Err(Status::internal(format!("set balance error: {:?}", e)))
            }
        }
    }

    /// Returns a blockchain transaction
    async fn get_transaction(
        &self,
        request: Request<GetTransactionRequest>,
    ) -> Result<Response<GetTransactionResponse>, Status> {
        match SimpleBlockchainService::get_transaction(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("get tx info error: {:?}", e);
                Err(Status::internal(format!("get tx info error: {:?}", e)))
            }
        }
    }

    /// Returns a blockchain account
    async fn get_account(
        &self,
        request: Request<GetAccountRequest>,
    ) -> Result<Response<GetAccountResponse>, Status> {
        match SimpleBlockchainService::get_account(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("get account error: {:?}", e);
                Err(Status::internal(format!("get account error: {:?}", e)))
            }
        }
    }

    async fn get_provider_identity_bundle(
        &self,
        request: Request<GetProviderIdentityBundleRequest>,
    ) -> Result<Response<GetProviderIdentityBundleResponse>, Status> {
        match SimpleBlockchainService::get_provider_bundle(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("get provider bundle error: {:?}", e);
                Err(Status::internal(format!(
                    "get provider bundle error: {:?}",
                    e
                )))
            }
        }
    }

    async fn get_client_identity_bundle(
        &self,
        request: Request<GetClientIdentityBundleRequest>,
    ) -> Result<Response<GetClientIdentityBundleResponse>, Status> {
        match SimpleBlockchainService::get_client_bundle(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("get provider bundle error: {:?}", e);
                Err(Status::internal(format!(
                    "get provider bundle error: {:?}",
                    e
                )))
            }
        }
    }

    async fn get_clients(
        &self,
        request: Request<GetClientsRequest>,
    ) -> Result<Response<GetClientsResponse>, Status> {
        match SimpleBlockchainService::get_clients(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("get clients error: {:?}", e);
                Err(Status::internal(format!("get clients error: {:?}", e)))
            }
        }
    }

    async fn get_providers(
        &self,
        request: Request<GetProvidersRequest>,
    ) -> Result<Response<GetProvidersResponse>, Status> {
        match SimpleBlockchainService::get_providers(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("get providers error: {:?}", e);
                Err(Status::internal(format!("get providers error: {:?}", e)))
            }
        }
    }

    async fn get_block(
        &self,
        request: Request<GetBlockRequest>,
    ) -> Result<Response<GetBlockResponse>, Status> {
        match SimpleBlockchainService::get_block(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("get block error: {:?}", e);
                Err(Status::internal(format!("get block error: {:?}", e)))
            }
        }
    }

    async fn get_current_block(
        &self,
        request: Request<GetCurrentBlockRequest>,
    ) -> Result<Response<GetBlockResponse>, Status> {
        match SimpleBlockchainService::get_current_block(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("get block error: {:?}", e);
                Err(Status::internal(format!("get block error: {:?}", e)))
            }
        }
    }

    async fn get_validated_blocks_count_by_entity(
        &self,
        _request: Request<GetBlocksCountByEntityRequest>,
    ) -> Result<Response<GetBlocksCountByEntityResponse>, Status> {
        todo!()
    }

    async fn get_sealed_blocks_count_by_entity(
        &self,
        _request: Request<GetBlocksCountByEntityRequest>,
    ) -> Result<Response<GetBlocksCountByEntityResponse>, Status> {
        todo!()
    }
}
