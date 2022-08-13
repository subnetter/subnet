// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;
extern crate base;
extern crate clap;
extern crate db;

use base::logging_service::{InitLogger, LoggingService};
use base::server_config_service::{ServerConfigService, SetConfigFile};
use clap::{App, Arg};
use cryptomail::commands::configure::ConfigureMessage;
use cryptomail::commands::service::CryptoMailService;
use cryptomail::commands::start_grpc_server::StartGrpcServerMessage;
use tokio::signal;
use xactor::*;

// Start a client app - good for testability / integration testing
pub async fn start() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Cryptomail Server")
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

    let config = ServerConfigService::from_registry().await.unwrap();

    // merge values from config file over default config values and ones provided via process flags
    if let Some(conf_file) = matches.value_of("config") {
        config
            .call(SetConfigFile {
                config_file: conf_file.into(),
            })
            .await?
            .unwrap();
    }

    debug!("CryptoMail service starting...");

    // Start app logging
    let logging = LoggingService::from_registry().await?;
    let _ = logging
        .call(InitLogger {
            peer_name: "CryptoMail Service".into(),
            brief: false, // todo: take from config
        })
        .await?;

    // Start and configure the CryptoMail service
    let service = CryptoMailService::from_registry().await?;
    CryptoMailService::config(ConfigureMessage {}).await?;

    // start the grpc server
    service.call(StartGrpcServerMessage {}).await??;

    debug!("CryptoMail started");

    signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl-c signal");

    debug!("stopping process via a ctrl-c signal...");

    Ok(())
}
