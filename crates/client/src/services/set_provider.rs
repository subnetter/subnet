// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::{SimpleClient, SNP_PROTOCOL_VERSION};
use anyhow::{anyhow, bail, Result};
use base::snp::snp_core_types::{
    ClientIdentityBundle, DialupInfo, EntityId, PreKey, ProviderSignedClientIdentityBundle,
    PublicKey,
};
use base::snp::snp_payments::{Address, Amount, CoinType};
use base::snp::snp_server_api::provider_core_service_client::ProviderCoreServiceClient;
use base::snp::snp_server_api::{
    GetIdentityBundleRequest, GetTermsOfServiceRequest, MessageType, StartServiceRequest,
    StartServiceResponse,
};

use base::api_types_extensions::Signed;
use base::snp::snp_blockchain::transaction::Data;
use base::snp::snp_blockchain::{
    ClientBundleTransactionData, SubmitTransactionRequest, Transaction, TransactionFee,
};
use chrono::prelude::*;
use xactor::*;

#[message(result = "Result<ProviderSignedClientIdentityBundle>")]
pub struct SetProvider {
    pub dialup_info: DialupInfo,
}

/// Request client to set its provider to a given provider.
/// This is a public client test api method.
#[async_trait::async_trait]
impl Handler<SetProvider> for SimpleClient {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: SetProvider,
    ) -> Result<ProviderSignedClientIdentityBundle> {
        // Step 1 - connect to provider. store connection and get provider bundle and store it

        let info = msg.dialup_info;
        info!(
            "Connecting to {}",
            format!("http://{}:{}", info.ip_address, info.port)
        );

        let mut provider_api_service =
            ProviderCoreServiceClient::connect(format!("http://{}:{}", info.ip_address, info.port))
                .await?;

        info!("Connected to {}", info.name);

        // get the provider terms of service for new users so client can use data in it to make future calls
        // e.g. pricing information
        let terms = provider_api_service
            .get_terms_of_service(GetTermsOfServiceRequest {
                promo_code: "".to_string(),
            })
            .await?
            .into_inner()
            .terms
            .ok_or_else(|| anyhow!("missing provider terms of service"))?;

        info!("got provider terms of service");

        // todo: implement me
        // verify provider signed on terms
        // terms.verify_signature()?;

        // get a ref to use before storing
        let contract = terms
            .service_terms
            .as_ref()
            .ok_or_else(|| anyhow!("missing contract data"))?
            .clone();

        // store terms for future use
        self.provider_terms = Some(terms);

        let provider_bundle = provider_api_service
            .get_identity_bundle(GetIdentityBundleRequest {
                protocol_version: SNP_PROTOCOL_VERSION.into(),
            })
            .await?
            .into_inner()
            .bundle
            .ok_or_else(|| anyhow!("missing provider bundle"))?;

        info!("got provider bundle");

        self.provider_bundle = Some(provider_bundle.clone());
        // Store the api client with our provider for later use
        self.provider_net_client = Some(provider_api_service);

        // Step 2 - create client bundle, sign it and send StartService to provider

        let client_pre_key_pub_data: x25519_dalek::PublicKey = (&self.pre_key).into();
        let client_pre_key_public: PublicKey = PublicKey {
            key: client_pre_key_pub_data.to_bytes().to_vec(),
        };
        let client_id_pub_key = PublicKey {
            key: self.client_id.public.as_ref().to_vec(),
        };

        // our entity with client name as nickname
        let client_entity = EntityId {
            public_key: Some(client_id_pub_key.clone()),
            nickname: self.client_name.clone(),
        };

        let mut client_bundle = ClientIdentityBundle {
            time_stamp: Utc::now().timestamp_nanos() as u64,
            client_id: Some(client_entity.clone()),
            // for now - we just create an address for pub key. This should come from wallet.
            address: Some(Address::new(&client_id_pub_key)),
            provider_bundle: Some(self.provider_bundle.as_ref().unwrap().clone()),
            pre_key: Some(PreKey {
                x2dh_version: "".to_string(),
                key: Some(client_pre_key_public),
                key_id: 0,
            }),
            one_time_keys: vec![],
            profile_image: None,
            signature: None,
            net_id: 0,
        };

        client_bundle.sign(&self.client_id)?;

        // Store our client bundle for future use
        self.client_bundle = Some(client_bundle.clone());

        info!("requesting start service...");

        // start new session with provider and set the startServe as the sessions message
        let start_service_request = StartServiceRequest {
            bundle: Some(client_bundle),
            payment: None,
            service_contract_id: contract.id,
            contract_options: 0, // monthly fixed or pay per usage
        };

        use prost::Message;
        let mut buff = Vec::with_capacity(start_service_request.encoded_len());
        start_service_request.encode(&mut buff).unwrap();

        let resp_message = self
            .send_new_session_to_provider(MessageType::StartServiceRequest, buff)
            .await?;

        info!("got service response");

        if resp_message.msg_type != (MessageType::StartServiceResponse as i32) {
            return Err(anyhow!(
                "unexpected response message type {}",
                resp_message.msg_type
            ));
        };

        // get the provider signed client bundle from the response and return it
        let start_service_resp: StartServiceResponse =
            StartServiceResponse::decode(resp_message.message.as_slice()).map_err(|e| {
                anyhow!("failed to decode client terms of service request: {:?}", e)
            })?;

        // subscribe to messages for this client on the provider
        self.subscribe_to_provider_messages().await?;

        let client_bundle = start_service_resp
            .bundle
            .ok_or_else(|| anyhow!("missing client bundle"))?;

        /*
        if let Some(_service) = self.blockchain_service_client.as_mut() {
            /*
            service
                .add_client(tonic::Request::new(client_bundle.clone()))
                .await?;*/
        } else {
            warn!("No blokchain service set on this client")
        }

        //self.publish_client_bundle(&client_bundle).await?;
        */

        info!("done");

        Ok(client_bundle)
    }
}

impl SimpleClient {
    async fn _publish_client_bundle(
        &mut self,
        bundle: &ProviderSignedClientIdentityBundle,
    ) -> Result<()> {
        if let Some(client) = self.blockchain_service_client.as_mut() {
            let bundle_tx_data = ClientBundleTransactionData {
                client_bundle: Some(bundle.clone()),
            };

            let tx_fee = TransactionFee {
                amount: Some(Amount {
                    value: 1,
                    coin_type: CoinType::Core as i32,
                }),
                payer_public_key: vec![], // sender pays fee
            };

            let mut tx = Transaction {
                sender_pub_key: self.client_id.public.to_bytes().to_vec(),
                fee: Some(tx_fee),
                counter: 1,
                entity_id: None,
                net_id: 0,
                signature: vec![],
                data: Some(Data::ClientBundle(bundle_tx_data)),
                fee_signature: vec![], // sender pays fee
            };

            tx.sign(&self.client_id).unwrap();

            let res = client
                .submit_transaction(SubmitTransactionRequest {
                    transaction: Some(tx),
                })
                .await
                .unwrap()
                .into_inner();

            let _tx_id = res.id.unwrap();

            Ok(())
        } else {
            bail!("No blockchain service set on this client")
        }
    }
}
