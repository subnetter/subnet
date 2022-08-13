// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::grpc_api_service::SimpleClientGrpcService;
use anyhow::{anyhow, Result};
use base::client_config_service::ClientConfigService;
use base::hex_utils::short_hex_string;
use base::server_config_service::{DB_NAME_CONFIG_KEY, DROP_DB_CONFIG_KEY};
use base::snp::snp_blockchain::blockchain_service_client::BlockchainServiceClient;
use base::snp::snp_core_types::{
    ChannelBundle, ClientIdentityBundle, ContentItem, EntityId, ProviderIdentityBundle,
    ProviderSignedClientIdentityBundle, ServiceTermsBundle,
};
use base::snp::snp_payments::Address;
use base::snp::snp_server_api::provider_core_service_client::ProviderCoreServiceClient;
use base::snp::upsetter_simple_client::simple_client_user_service_server::SimpleClientUserServiceServer;
use db::db_service::{Configure, DatabaseService, PROVIDER_COL_FAMILY, TESTS_COL_FAMILY};
use ed25519_dalek::Keypair;
use rand_core::OsRng;
use rocksdb::{ColumnFamilyDescriptor, Options};
use std::collections::HashMap;
use tonic::transport::{Channel, Server};
use x25519_dalek::StaticSecret;
use xactor::*;

pub const SNP_PROTOCOL_VERSION: &str = "0.1.0";

/// A simple client creates a new id when it is running
/// and has only one pre_key it uses
/// Currently doesn't use a db or x2dh or dr services for more robust functionality
/// todo: use a DR service to store DR sessions w provider and w other clients instead of hard-coded ones
pub struct SimpleClient {
    pub(crate) client_name: String,
    /// client long term ed25519 id
    pub(crate) client_id: Keypair,
    /// for now we assume only 1 pre-key for the client and we don't create new ones yet
    pub(crate) pre_key: StaticSecret,
    /// our provider bundle
    pub(crate) provider_bundle: Option<ProviderIdentityBundle>,
    /// our provider terms of service
    pub(crate) provider_terms: Option<ServiceTermsBundle>,
    /// a connection with our provider
    pub(crate) provider_net_client: Option<ProviderCoreServiceClient<Channel>>,
    /// our client bundle with our provider
    pub(crate) client_bundle: Option<ClientIdentityBundle>,
    /// other clients indexed by pub key
    pub(crate) other_clients: HashMap<Vec<u8>, ProviderSignedClientIdentityBundle>,
    /// channels this client is subscribed to (groups and status updates)
    pub(crate) channels_subscriptions: HashMap<Vec<u8>, ChannelBundle>,
    /// channels client requested to subscribe to but subscription not confirmed yet
    pub(crate) channels_subscriptions_requests: HashMap<Vec<u8>, ChannelBundle>,
    /// paid content items published by this client
    pub(crate) paid_items: HashMap<u64, ContentItem>,
    /// A name server client used to communicate with a name service
    pub(crate) blockchain_service_client: Option<BlockchainServiceClient<Channel>>,
}

impl SimpleClient {
    pub(crate) fn get_client_entity(&self) -> Result<EntityId> {
        Ok(self
            .client_bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing client bundle"))?
            .client_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing client id"))?
            .clone())
    }

    /// Client's payment address - derived from public id for now
    pub(crate) fn get_payment_address(&self) -> Result<Address> {
        Ok(Address {
            data: self.client_id.public.to_bytes()[12..].to_vec(),
        })
    }
}

impl Default for SimpleClient {
    fn default() -> Self {
        let client_id = Keypair::generate(&mut OsRng);
        info!(
            "New client pub id: {}",
            short_hex_string(client_id.public.as_ref())
        );

        SimpleClient {
            client_name: "client???".into(),
            client_id,
            pre_key: StaticSecret::new(&mut OsRng),
            provider_bundle: None,
            provider_net_client: None,
            client_bundle: None,
            channels_subscriptions: HashMap::new(),
            channels_subscriptions_requests: HashMap::new(),
            other_clients: HashMap::new(),
            paid_items: HashMap::new(),
            blockchain_service_client: None,
            provider_terms: None,
        }
    }
}

impl Service for SimpleClient {}

#[async_trait::async_trait]
impl Actor for SimpleClient {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        // init here system services used by this client

        info!("initializing client db...");
        let db_name = ClientConfigService::get(DB_NAME_CONFIG_KEY.into())
            .await?
            .unwrap();
        let drop_on_exit = ClientConfigService::get_bool(DROP_DB_CONFIG_KEY.into())
            .await?
            .unwrap();

        // no column descriptors ????
        DatabaseService::config_db(Configure {
            drop_on_exit,
            db_name,
            col_descriptors: vec![
                ColumnFamilyDescriptor::new(PROVIDER_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(TESTS_COL_FAMILY, Options::default()),
            ],
        })
        .await?;

        info!("SimpleClient started");
        Ok(())
    }
}

#[message(result = "Result<()>")]
pub struct StartGrpcServer {
    pub grpc_port: u32,
    pub grpc_host: String,
    pub client_name: String,
}

/// Starts this client grpc server
#[async_trait::async_trait]
impl Handler<StartGrpcServer> for SimpleClient {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: StartGrpcServer) -> Result<()> {
        // setup grpc server and services
        let grpc_server_addr = format!("{}:{}", msg.grpc_host, msg.grpc_port)
            .parse()
            .unwrap();
        info!(
            "starting {} client grpc server on: {}",
            msg.client_name, grpc_server_addr
        );

        self.client_name = msg.client_name;
        let client_grpc_service = SimpleClientGrpcService::default();

        tokio::task::spawn(async move {
            let res = Server::builder()
                .add_service(SimpleClientUserServiceServer::new(client_grpc_service))
                .serve(grpc_server_addr)
                .await;

            if res.is_err() {
                debug!("client grpc server stopping due to: {:?}", res);
            } else {
                debug!("client grpc Server stopped.");
            }
        });

        Ok(())
    }
}
