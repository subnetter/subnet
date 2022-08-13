//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::messaging::messaging_service::ServerMessagingService;
use anyhow::{anyhow, Result};
use base::api_types_extensions::Signed;
use base::snp::snp_core_types::ServiceTermsBundle;
use base::snp::snp_payments::ServiceTerms;
use base::snp::snp_server_api::{
    GetTermsOfServiceRequest, GetTermsOfServiceResponse, MessageType, TypedMessage,
};
use base::typed_msgs_dispatcher::{
    Subscribe, TypedMessageHandler, TypedMessagesDispatcher, Unsubscribe,
};
use chrono::prelude::*;
use prost::Message;
use xactor::*;

/// TermsService handles terms of service related requests
#[derive(Debug, Default)]
pub(crate) struct TermsService {}
impl Service for TermsService {}

#[async_trait::async_trait]
impl Actor for TermsService {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        // subscribe to terms of service messages
        let subscribe_msg = Subscribe {
            message_type: MessageType::ServiceTermsRequest as i32,
            subscriber: ctx.address().caller(),
        };

        let dispatcher = TypedMessagesDispatcher::from_registry().await.unwrap();
        dispatcher.call(subscribe_msg).await??;
        debug!("TermsService started and subscribed to handle ServiceTermsRequests");
        Ok(())
    }

    async fn stopped(&mut self, _ctx: &mut Context<Self>) {
        // Unsubscribe from the dispatcher
        let dispatcher = TypedMessagesDispatcher::from_registry().await.unwrap();
        let _res = dispatcher
            .call(Unsubscribe {
                id: MessageType::ServiceTermsRequest as i32,
            })
            .await;
    }
}

impl TermsService {
    // async fn get_new_user_service_contract(&mut self) -> Result

    /// Returns current provider terms of service
    async fn get_current_terms(&mut self) -> Result<ServiceTermsBundle> {
        let private_bundle = ServerMessagingService::get_curr_provider_id_bundle().await?;
        let public_bundle = private_bundle
            .public_bundle
            .ok_or_else(|| anyhow!("missing public bundle data"))?;
        let provider_id = public_bundle
            .provider_id
            .ok_or_else(|| anyhow!("missing provider id"))?;

        // todo: generate ServiceTermsBundle when new provider identity is created, store it in db and return stored terms and don't generate and sign new terms per request.

        let mut terms = ServiceTermsBundle {
            provider_id: Some(provider_id),
            signature: None,
            service_terms: Some(ServiceTerms {
                id: 0,
                created: 0,
                valid_until: 0,
                pricing_model: 0,
                user_id: vec![],
                free_trial_period: 0,
                min_balance: None,
                max_balance: None,
                balance: None,
                routing_msg_base_cost: None,
                routing_msg_cost_per_byte: None,
                data_store_per_byte: None,
                registration_fee: None,
                monthly_fixed_fee: None,
                max_user_storage_space: 0,
                max_file_size: 0,
                payable_account: None,
            }),
        };
        let key_pair = private_bundle
            .provider_id_keypair
            .ok_or_else(|| anyhow!("missing key pair"))?
            .to_ed2559_kaypair();

        terms.sign(&key_pair)?;

        Ok(terms)
    }
}

#[message(result = "Result<ServiceTermsBundle>")]
pub(crate) struct GetCurrentTerms;

/// Direct access to current terms of service for new users
#[async_trait::async_trait]
impl Handler<GetCurrentTerms> for TermsService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetCurrentTerms,
    ) -> Result<ServiceTermsBundle> {
        self.get_current_terms().await
    }
}

// Callback for our registered typed message handling - handle a ServiceTermsRequest
#[async_trait::async_trait]
impl Handler<TypedMessageHandler> for TermsService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: TypedMessageHandler,
    ) -> Result<TypedMessage> {
        // step 1 - verify we know how to handle the message
        if msg.0.msg_type != (MessageType::ServiceTermsRequest as i32) {
            return Err(anyhow!("Unexpected message type {}", msg.0.msg_type));
        };

        // step 2 - decode the request into the expected object
        let _req: GetTermsOfServiceRequest =
            GetTermsOfServiceRequest::decode(msg.0.message.as_slice()).map_err(|e| {
                anyhow!("failed to decode client terms of service request: {:?}", e)
            })?;

        // step 3 - process the request and prepare response data

        // todo: if caller is an existing user then need to return his current terms of service and terms of service for next period (when current terms expire).

        let terms = self.get_current_terms().await?;
        let resp = GetTermsOfServiceResponse { terms: Some(terms) };

        debug!("created terms of service response to client request");

        // step 4 - create result typed message and return it
        let mut buff = Vec::with_capacity(resp.encoded_len());
        let _res = resp.encode(&mut buff);

        Ok(TypedMessage {
            time_stamp: Utc::now().timestamp_nanos() as u64,
            msg_type: MessageType::ServiceTermsResponse as i32,
            message: buff,
            receiver: None,
            sender: None,
            signature: None,
        })
    }
}
