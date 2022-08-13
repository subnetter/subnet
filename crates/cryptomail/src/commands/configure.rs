// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::consts::{
    ACCOUNTS_COL_FAMILY, ACCOUNTS_NAMES_COL_FAMILY, ALLOW_HTTP_USER_MEDIA_KEY, BOXES_COL_FAMILY,
    DEPOSIT_CONFIRMATIONS_CONFIG_KEY, DEPOSIT_CONFIRMATION_PERIOD_BLOCKS, ETH_NET_ID_CONFIG_KEY,
    KOVAN_NET_ID, MESSAGES_COL_FAMILY, PUB_ACCOUNTS_COL_FAMILY, SYSTEM_COL_FAMILY,
    THREADS_COL_FAMILY,
};
use crate::features::eth_api_client::EthApiClient;
use anyhow::Result;
use base::server_config_service::{
    ServerConfigService, DB_NAME_CONFIG_KEY, DROP_DB_CONFIG_KEY, GRPC_HOST_CONFIG_KEY,
};
use db::db_service::DatabaseService;
use rocksdb::{ColumnFamilyDescriptor, Options};
use xactor::*;

#[message(result = "Result<()>")]
pub struct ConfigureMessage {}

impl CryptoMailService {
    pub async fn config(config: ConfigureMessage) -> Result<()> {
        let service = CryptoMailService::from_registry().await?;
        service.call(config).await?
    }
}

/// Configure the service
#[async_trait::async_trait]
impl Handler<ConfigureMessage> for CryptoMailService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: ConfigureMessage) -> Result<()> {
        info!("configuring service...");

        // config defaults are set here
        // for develop builds we need 2 as block doesn't advance in Ganache after a transaction automatically
        ServerConfigService::set_u64(
            DEPOSIT_CONFIRMATIONS_CONFIG_KEY.into(),
            DEPOSIT_CONFIRMATION_PERIOD_BLOCKS,
        )
        .await?;

        ServerConfigService::set_bool(ALLOW_HTTP_USER_MEDIA_KEY.into(), true).await?;

        // Switch to ipv4 from ipv6
        ServerConfigService::set(GRPC_HOST_CONFIG_KEY.into(), "0.0.0.0".into()).await?;

        // local dev
        //ServerConfigService::set_u64(ETH_NET_ID_CONFIG_KEY.into(), LOCAL_DEVNET_ID).await?;

        // kovan
        ServerConfigService::set_u64(ETH_NET_ID_CONFIG_KEY.into(), KOVAN_NET_ID).await?;

        // read config params values
        let db_name = ServerConfigService::get(DB_NAME_CONFIG_KEY.into())
            .await?
            .unwrap();

        let drop_on_exit = ServerConfigService::get_bool(DROP_DB_CONFIG_KEY.into())
            .await?
            .unwrap();

        let net_id = ServerConfigService::get_u64(ETH_NET_ID_CONFIG_KEY.into())
            .await?
            .unwrap();

        info!("db name: {}", db_name);
        info!("drop db on exit: {}", drop_on_exit);
        info!("eth network-id: {}", net_id);

        // configure eth api client
        info!("starting eth api client...");
        let eth_api_client = EthApiClient::new(net_id as u32)?;
        self.eth_api_client = Some(eth_api_client);

        // configure the database service
        DatabaseService::config_db(db::db_service::Configure {
            drop_on_exit,
            db_name,
            col_descriptors: vec![
                ColumnFamilyDescriptor::new(THREADS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(ACCOUNTS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(BOXES_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(ACCOUNTS_NAMES_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(PUB_ACCOUNTS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(MESSAGES_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(SYSTEM_COL_FAMILY, Options::default()),
            ],
        })
        .await?;

        info!("bootstrapping....");
        // bootstrap the service - store any service related entities in the db
        CryptoMailService::boostrap().await?;

        // start the deposits verifier - run it every 5 minutes
        let _ = CryptoMailService::start_verify_deposits_background_task(60).await;

        info!("config done");
        Ok(())
    }
}
