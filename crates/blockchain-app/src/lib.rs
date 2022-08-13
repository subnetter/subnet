// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;
extern crate base;
extern crate clap;
extern crate db;

use base::blockchain_config_service::{
    BlockchainConfigService, SetConfigFile, GRPC_HOST_CONFIG_KEY, GRPC_SERVER_PORT_CONFIG_KEY,
    SERVICE_NAME_CONFIG_KEY,
};
use base::logging_service::{InitLogger, LoggingService};

use blockchain::configure::Configure;
use blockchain::service::SimpleBlockchainService;
use blockchain::start_grpc_server::StartGrpcServer;
use clap::{App, Arg};
use db::db_service::DatabaseService;
use tokio::signal;
use xactor::*;

// Start a client app - good for testability / integration testing
pub async fn start() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Upsetter Blockchain Service")
        .version("0.1.0")
        .author("Foo Bar. <foo@bar.goo>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let config_service = BlockchainConfigService::from_registry().await.unwrap();

    // merge values from config file over default config values and ones provided via process flags
    if let Some(conf_file) = matches.value_of("config") {
        info!("Using config from: {}", conf_file);
        config_service
            .call(SetConfigFile {
                config_file: conf_file.into(),
            })
            .await?
            .unwrap();
    }

    // init base services
    debug!("blockchain service starting...");

    // Start app logger
    let logging = LoggingService::from_registry().await?;
    let _ = logging
        .call(InitLogger {
            peer_name: "blockchain service".into(),
            brief: true, // todo: pick from config
        })
        .await?;

    SimpleBlockchainService::config(Configure {}).await?;

    let server_name = BlockchainConfigService::get(SERVICE_NAME_CONFIG_KEY.into())
        .await?
        .unwrap();

    let grpc_host = BlockchainConfigService::get(GRPC_HOST_CONFIG_KEY.into())
        .await?
        .unwrap();

    let grpc_port = BlockchainConfigService::get_u64(GRPC_SERVER_PORT_CONFIG_KEY.into())
        .await?
        .unwrap() as u32;

    SimpleBlockchainService::start_grpc_server(StartGrpcServer {
        grpc_port,
        grpc_host,
        server_name,
    })
    .await
    .unwrap();

    info!("blockchain service started");

    signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl-c signal");

    debug!("stopping client-app via ctrl-c signal...");

    tokio::task::spawn(async {
        // stop the db service so it has a chance to destroy itself if it is configured to destroy storage on stop...
        let mut db_service = DatabaseService::from_registry().await.unwrap();
        let _ = db_service.stop(None);
        debug!("resources cleanup completed");
    })
    .await
    .unwrap();

    Ok(())
}
