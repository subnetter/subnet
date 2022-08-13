// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::consts::{
    ACCOUNTS_CF, BLOCKCHAIN_CF, BLOCKS_CF, CLIENTS_BUNDLES_CF, PROVIDERS_BUNDLES_CF,
    SEALER_BLOCKS_CF, SYSTEM_COL_FAMILY, TRANSACTIONS_CF, VALIDATOR_BLOCKS_CF,
};
use crate::service::SimpleBlockchainService;
use anyhow::Result;
use base::blockchain_config_service::BlockchainConfigService;
use base::server_config_service::{DB_NAME_CONFIG_KEY, DROP_DB_CONFIG_KEY};
use db::db_service::DatabaseService;
use rocksdb::{ColumnFamilyDescriptor, Options};
use xactor::*;

#[message(result = "Result<()>")]
pub struct Configure {}

#[async_trait::async_trait]
impl Handler<Configure> for SimpleBlockchainService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: Configure) -> Result<()> {
        // read config params values
        let db_name = BlockchainConfigService::get(DB_NAME_CONFIG_KEY.into())
            .await?
            .unwrap();

        let drop_on_exit = BlockchainConfigService::get_bool(DROP_DB_CONFIG_KEY.into())
            .await?
            .unwrap();

        // todo: merge any config params into the config

        info!("db name: {}", db_name);
        info!("drop db on exit: {}", drop_on_exit);

        // configure the database service
        DatabaseService::config_db(db::db_service::Configure {
            drop_on_exit,
            db_name: db_name.to_string(),
            col_descriptors: vec![
                ColumnFamilyDescriptor::new(BLOCKCHAIN_CF, Options::default()),
                ColumnFamilyDescriptor::new(BLOCKS_CF, Options::default()),
                ColumnFamilyDescriptor::new(VALIDATOR_BLOCKS_CF, Options::default()),
                ColumnFamilyDescriptor::new(SEALER_BLOCKS_CF, Options::default()),
                ColumnFamilyDescriptor::new(TRANSACTIONS_CF, Options::default()),
                ColumnFamilyDescriptor::new(ACCOUNTS_CF, Options::default()),
                ColumnFamilyDescriptor::new(PROVIDERS_BUNDLES_CF, Options::default()),
                ColumnFamilyDescriptor::new(CLIENTS_BUNDLES_CF, Options::default()),
                ColumnFamilyDescriptor::new(SYSTEM_COL_FAMILY, Options::default()),
            ],
        })
        .await?;

        info!("config done");
        Ok(())
    }
}
