//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::new_account::CreateAccountMessage;
use crate::commands::new_thread::NewThreadMessage;
use crate::commands::service::CryptoMailService;
use crate::model::api::cryptomail_api_service_server::CryptomailApiService;
use crate::model::api::*;
use anyhow::Result;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub(crate) struct CryptoMailGrpcService {}

impl Default for CryptoMailGrpcService {
    fn default() -> Self {
        debug!("CryptoMailGrpcService started");
        CryptoMailGrpcService {}
    }
}

/// Public api
#[tonic::async_trait]
impl CryptomailApiService for CryptoMailGrpcService {
    /// Create a new user account
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<CreateAccountResponse>, Status> {
        match CryptoMailService::create_account(CreateAccountMessage {
            request: request.into_inner(),
        })
        .await
        {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("Can't create account: {}", e);
                Err(Status::internal(format!("internal server error: {}", e)))
            }
        }
    }

    /// Update account settings
    async fn update_settings(
        &self,
        request: Request<UpdateSettingsRequest>,
    ) -> Result<Response<UpdateSettingsResponse>, Status> {
        match CryptoMailService::update_settings(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("Can't update setting: {}", e);
                Err(Status::internal(format!("internal server error: {}", e)))
            }
        }
    }

    /// Delete an account and all related account data from store. Note that messages sent by the account
    /// are still available for their recipients
    async fn delete_account(
        &self,
        request: Request<DeleteAccountRequest>,
    ) -> Result<Response<DeleteAccountResponse>, Status> {
        match CryptoMailService::delete_account(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("Can't delete account: {}", e);
                Err(Status::internal(format!("internal server error: {}", e)))
            }
        }
    }

    /// Get thread boxes for an account
    async fn get_thread_boxes(
        &self,
        request: Request<GetThreadBoxesRequest>,
    ) -> Result<Response<GetThreadBoxesResponse>, Status> {
        match CryptoMailService::get_thread_boxes(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("Can't get thread-boxes: {}", e);
                Err(Status::internal(format!("internal server error: {}", e)))
            }
        }
    }

    /// Open a message gets its full content by an account and updates its and the account open count
    async fn open_message(
        &self,
        request: Request<OpenMessageRequest>,
    ) -> Result<Response<OpenMessageResponse>, Status> {
        match CryptoMailService::open_message_handler(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("Can't open message: {}", e);
                Err(Status::internal(format!("internal server error: {}", e)))
            }
        }
    }

    /// Reply to a message in a thread
    async fn reply(
        &self,
        request: Request<ReplyRequest>,
    ) -> Result<Response<ReplyResponse>, Status> {
        match CryptoMailService::reply(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("Can't reply to a message: {}", e);
                Err(Status::internal(format!("internal server error: {}", e)))
            }
        }
    }

    /// Move a thread for an account from inbox to archive-box.
    async fn archive_thread(
        &self,
        request: Request<ArchiveThreadRequest>,
    ) -> Result<Response<ArchiveThreadResponse>, Status> {
        match CryptoMailService::archive_thread(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("Can't archive thread: {}", e);
                Err(Status::internal(format!("internal server error: {}", e)))
            }
        }
    }

    /// Delete a thread - note that this is just deleted for the account which deleted it. Not the other account.
    /// the messages are not deleted from store and the thread is available for the other party.
    async fn delete_thread(
        &self,
        request: Request<DeleteThreadRequest>,
    ) -> Result<Response<DeleteThreadResponse>, Status> {
        match CryptoMailService::delete_thread(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("Can't delete thread: {}", e);
                Err(Status::internal(format!("internal server error: {}", e)))
            }
        }
    }

    /// Start a new thread with recipient account with first paid message to recipient
    async fn new_thread(
        &self,
        request: Request<NewThreadRequest>,
    ) -> Result<Response<NewThreadResponse>, Status> {
        let req = request.into_inner();
        let message_id = req.message_id.as_ref().unwrap().clone();

        match CryptoMailService::new_thread(NewThreadMessage { request: req }).await {
            Ok(result) => Ok(Response::new(NewThreadResponse {
                result: result as i32,
                message_id: Some(message_id),
            })),
            Err(e) => {
                error!("Can't create a new thread. Error: {}", e);
                Err(Status::internal(format!("internal server error: {}", e)))
            }
        }
    }

    /// Returns basic public account info based on name or public key input
    async fn get_account(
        &self,
        request: Request<GetAccountRequest>,
    ) -> Result<Response<GetAccountResponse>, Status> {
        match CryptoMailService::get_account(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("Can't get public account: {}", e);
                Err(Status::internal(format!("internal server error: {}", e)))
            }
        }
    }

    async fn get_public_accounts(
        &self,
        request: Request<GetPublicAccountsRequest>,
    ) -> Result<Response<GetPublicAccountsResponse>, Status> {
        match CryptoMailService::public_accounts(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("Can't get public accounts: {}", e);
                Err(Status::internal(format!("internal server error: {}", e)))
            }
        }
    }

    async fn get_message_deposit_data(
        &self,
        request: Request<GetMessageDepositDataRequest>,
    ) -> Result<Response<GetMessageDepositDataResponse>, Status> {
        match CryptoMailService::message_deposit_data(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("Can't get message deposit data: {}", e);
                Err(Status::internal(format!("internal server error: {}", e)))
            }
        }
    }

    async fn get_coin_price(
        &self,
        request: Request<GetCoinPriceRequest>,
    ) -> Result<Response<GetCoinPriceResponse>, Status> {
        match CryptoMailService::get_coin_price(request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(e) => {
                error!("Can't get coin fiat price: {}", e);
                Err(Status::internal(format!("internal server error: {}", e)))
            }
        }
    }
}
