// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::server_config_service::{
    DB_NAME_CONFIG_KEY, DROP_DB_CONFIG_KEY, GRPC_HOST_CONFIG_KEY, GRPC_SERVER_PORT_CONFIG_KEY,
};
use anyhow::{anyhow, Result};
use config::{Config, Environment};
use log::*;
use xactor::*;

pub const CLIENT_NAME_CONFIG_KEY: &str = "client_name";

pub struct ClientConfigService {
    config: Config,
}

#[async_trait::async_trait]
impl Actor for ClientConfigService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("Client ConfigService started");
        Ok(())
    }
}

impl Service for ClientConfigService {}

impl Default for ClientConfigService {
    fn default() -> Self {
        let mut config = Config::default();

        config
            .set_default(DROP_DB_CONFIG_KEY, true)
            .unwrap()
            .set_default(GRPC_SERVER_PORT_CONFIG_KEY, 8081)
            .unwrap()
            .set_default(GRPC_HOST_CONFIG_KEY, "[::1]")
            .unwrap()
            // we always want to have a peer name - even a generic one
            .set_default("client_name", "client_anon")
            .unwrap()
            .set_default(DB_NAME_CONFIG_KEY, "client_db")
            .unwrap()
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .merge(Environment::with_prefix("UPSETTER_CLIENT"))
            .unwrap();

        ClientConfigService { config }
    }
}

#[message(result = "Result<()>")]
pub struct SetConfigFile {
    pub config_file: String,
}

#[async_trait::async_trait]
impl Handler<SetConfigFile> for ClientConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetConfigFile) -> Result<()> {
        // todo: verify config file exists and is readable by this process
        self.config
            .merge(config::File::with_name(msg.config_file.as_str()).required(false))
            .unwrap();

        debug!(
            "Merging content of config file {:?}",
            msg.config_file.as_str()
        );

        Ok(())
    }
}

// helpers
impl ClientConfigService {
    pub async fn get(key: String) -> Result<Option<String>> {
        let config = ClientConfigService::from_registry().await?;
        let res = config.call(GetValue(key)).await?;
        Ok(res)
    }

    // helper
    pub async fn get_bool(key: String) -> Result<Option<bool>> {
        let config = ClientConfigService::from_registry().await?;
        let res = config.call(GetBool(key)).await?;
        Ok(res)
    }

    // helper
    pub async fn get_u64(key: String) -> Result<Option<u64>> {
        let config = ClientConfigService::from_registry().await?;
        let res = config.call(GetU64(key)).await?;
        Ok(res)
    }
}

#[message(result = "Option<bool>")]
pub struct GetBool(pub String);

/// Get bool value
#[async_trait::async_trait]
impl Handler<GetBool> for ClientConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: GetBool) -> Option<bool> {
        match self.config.get_bool(&msg.0.as_str()) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }
}

#[message(result = "Option<u64>")]
pub struct GetU64(pub String);

#[async_trait::async_trait]
impl Handler<GetU64> for ClientConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: GetU64) -> Option<u64> {
        match self.config.get_int(&msg.0.as_str()) {
            Ok(res) => Some(res as u64),
            Err(_) => None,
        }
    }
}

#[message(result = "Option<String>")]
pub struct GetValue(pub String);

#[async_trait::async_trait]
impl Handler<GetValue> for ClientConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: GetValue) -> Option<String> {
        match self.config.get_str(&msg.0.as_str()) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }
}

#[message(result = "Result<()>")]
pub struct SetValue {
    pub key: String,
    pub value: String,
}

#[async_trait::async_trait]
impl Handler<SetValue> for ClientConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetValue) -> Result<()> {
        match self.config.set(msg.key.as_str(), msg.value) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("{:?}", e)),
        }
    }
}
