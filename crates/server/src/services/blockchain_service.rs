//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::provider_id::ProviderIdService;
use crate::services::provider_id_service::{GetCurrentIdentityBundle, GetPaymentAccountKeypair};
use anyhow::Result;
use base::api_types_extensions::Signed;
use base::hex_utils::hex_string;
use base::snp::snp_blockchain::blockchain_service_client::BlockchainServiceClient;
use base::snp::snp_blockchain::transaction::Data;
use base::snp::snp_blockchain::{
    ClientBundleTransactionData, ProviderBundleTransactionData, SetBalanceRequest,
    SubmitTransactionRequest, Transaction, TransactionFee,
};
use base::snp::snp_core_types::{
    DialupInfo, PrivateProviderIdentityBundle, ProviderSignedClientIdentityBundle,
};
use base::snp::snp_payments::{Amount, CoinType};
use ed25519_dalek::Keypair;
use tonic::transport::Channel;
use xactor::*;

// Blockchain service client api
impl BlockchainService {
    /// Publish this provider current bundle to the blockchain. Call me when provider bundle changes
    pub(crate) async fn _publish_provider_bundle() -> Result<()> {
        let service = BlockchainService::from_registry().await?;
        service.call(PublishProviderBundleMessage {}).await?
    }

    /// Publish a serviced client bundle to the blockchain service
    pub(crate) async fn publish_client_bundle(msg: PublishClientBundleMessage) -> Result<()> {
        let service = BlockchainService::from_registry().await?;
        service.call(msg).await?
    }

    /// Set the remote blockchain service for this server
    pub(crate) async fn setup_blockchain_service(dialup_info: DialupInfo) -> Result<()> {
        let service = BlockchainService::from_registry().await?;
        service.call(SetBlockchainService { dialup_info }).await?
    }
}

//////////

#[message(result = "Result<()>")]
pub(crate) struct SetBlockchainService {
    pub(crate) dialup_info: DialupInfo,
}

/// Set a blockchain service and publish our current id bundle to the service.
#[async_trait::async_trait]
impl Handler<SetBlockchainService> for BlockchainService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetBlockchainService) -> Result<()> {
        let dialup_info = msg.dialup_info;
        info!(
            "Connecting to blockchain-service set to {} {}",
            dialup_info.ip_address, dialup_info.port
        );

        let mut client = BlockchainServiceClient::connect(format!(
            "http://{}:{}",
            dialup_info.ip_address, dialup_info.port
        ))
        .await?;

        info!(
            "Blockchain service set to {} {}",
            dialup_info.ip_address, dialup_info.port
        );

        // Give provider genesis coins so it can perform transactions
        info!("set provider genesis coins...");

        let provider_id_service = ProviderIdService::from_registry().await?;

        let bundle: PrivateProviderIdentityBundle = provider_id_service
            .call(GetCurrentIdentityBundle {})
            .await??;

        let payment_address = bundle.get_payment_address()?;

        // Set genesis amount so we can pay transactions fee
        client
            .set_balance(SetBalanceRequest {
                address: Some(payment_address.clone()),
                amount: Some(Amount {
                    value: 1000,
                    coin_type: CoinType::Core as i32,
                }),
            })
            .await?;

        self.blockchain_service_client = Some(client);

        info!("publishing provider bundle to blockchain...");

        self.publish_provider_bundle_to_blockchain().await
    }
}

////////////

#[message(result = "Result<()>")]
pub(crate) struct PublishProviderBundleMessage {}

/// Publish provider bundle to the blockchain service
#[async_trait::async_trait]
impl Handler<PublishProviderBundleMessage> for BlockchainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: PublishProviderBundleMessage,
    ) -> Result<()> {
        self.publish_provider_bundle_to_blockchain().await
    }
}

impl BlockchainService {
    /// Private helper function
    async fn publish_provider_bundle_to_blockchain(&mut self) -> Result<()> {
        let client = self.blockchain_service_client.as_mut().unwrap();

        let provider_id_service = ProviderIdService::from_registry().await?;
        let bundle: PrivateProviderIdentityBundle = provider_id_service
            .call(GetCurrentIdentityBundle {})
            .await??;

        let payment_keypair: Keypair = provider_id_service
            .call(GetPaymentAccountKeypair {})
            .await??;

        let bundle_data = ProviderBundleTransactionData {
            provider_bundle: Some(bundle.public_bundle.as_ref().unwrap().clone()),
        };

        let tx_fee = TransactionFee {
            amount: Some(Amount {
                value: 1,
                coin_type: CoinType::Core as i32,
            }),
            payer_public_key: vec![], // sender pays fee
        };

        let counter = self.counter + 1;

        // provider 1 bundle submission
        let mut tx = Transaction {
            sender_pub_key: payment_keypair.public.as_ref().to_vec(),
            fee: Some(tx_fee),
            counter,
            entity_id: None,
            net_id: 0,
            signature: vec![],
            data: Some(Data::ProviderBundle(bundle_data)),
            fee_signature: vec![], // sender pays fee
        };

        tx.sign(&payment_keypair).unwrap();

        let res = client
            .submit_transaction(SubmitTransactionRequest {
                transaction: Some(tx),
            })
            .await
            .unwrap()
            .into_inner();

        let tx_id = res.id.unwrap();
        info!("tx id: {}", hex_string(tx_id.id.as_ref()));
        self.counter = counter;

        Ok(())
    }
}

/// BlockchainService is a...
#[derive(Debug, Clone)]
pub(crate) struct BlockchainService {
    blockchain_service_client: Option<BlockchainServiceClient<Channel>>,
    /// transactions counter (todo: move to wallet)
    counter: u64,
}

impl Default for BlockchainService {
    fn default() -> Self {
        debug!("BlockchainService started");
        BlockchainService {
            blockchain_service_client: None,
            counter: 0,
        }
    }
}

#[async_trait::async_trait]
impl Actor for BlockchainService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("BlockchainService started");
        Ok(())
    }
}

impl Service for BlockchainService {}

////////////////

#[message(result = "Result<()>")]
pub(crate) struct PublishClientBundleMessage {
    pub(crate) client_bundle: ProviderSignedClientIdentityBundle,
}

/// Publish provider bundle to the blockchain service
#[async_trait::async_trait]
impl Handler<PublishClientBundleMessage> for BlockchainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: PublishClientBundleMessage,
    ) -> Result<()> {
        let provider_id_service = ProviderIdService::from_registry().await.unwrap();

        let bundle_tx_data = ClientBundleTransactionData {
            client_bundle: Some(msg.client_bundle.clone()),
        };

        let tx_fee = TransactionFee {
            amount: Some(Amount {
                value: 1,
                coin_type: CoinType::Core as i32,
            }),
            payer_public_key: vec![], // sender pays fee
        };

        let payment_keypair: Keypair = provider_id_service
            .call(GetPaymentAccountKeypair {})
            .await??;

        let counter = self.counter + 1;

        // client 1 bundle submission by provider 1
        let mut tx = Transaction {
            sender_pub_key: payment_keypair.public.as_ref().to_vec(),
            fee: Some(tx_fee),
            counter,
            entity_id: None,
            net_id: 0,
            signature: vec![],
            data: Some(Data::ClientBundle(bundle_tx_data)),
            fee_signature: vec![], // sender pays fee
        };

        tx.sign(&payment_keypair).unwrap();

        let res = self
            .blockchain_service_client
            .as_mut()
            .unwrap()
            .submit_transaction(SubmitTransactionRequest {
                transaction: Some(tx),
            })
            .await
            .unwrap()
            .into_inner();

        let tx_id = res.id.unwrap();
        self.counter = counter;

        info!("tx id: {}", hex_string(tx_id.id.as_ref()));

        Ok(())
    }
}
