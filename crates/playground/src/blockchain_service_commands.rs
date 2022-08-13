// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::playground::{ChildGuard, Playground};
use anyhow::{anyhow, Result};
use base::hex_utils::short_hex_string;
use base::snp::snp_blockchain::blockchain_service_client::BlockchainServiceClient;
use base::snp::snp_blockchain::{GetClientsRequest, GetProvidersRequest};
use base::snp::snp_core_types::DialupInfo;
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;
// use std::time::Duration;
// use tokio::time::sleep;

impl Playground {
    /// Start a blockchain server with provided config file
    pub(crate) async fn start_blockchain_server(&mut self, config_file: String) -> Result<()> {
        let blockchain_server_app = Command::new("./blockchain-app")
            .args(&["-c", config_file.as_str()])
            .spawn()
            .unwrap();

        self.proc_guards.push(ChildGuard(blockchain_server_app));

        let config = self.read_json_from_file(config_file).await?;

        let port = config["grpc_server_port"].as_u64().unwrap() as u32;
        let net_id = config["net_id"].as_u64().unwrap() as u32;
        let host_name = config["host_name"].as_str().unwrap();
        let name = config["peer_name"].as_str().unwrap();

        let info = DialupInfo {
            end_point: 0,
            api_version: "0.1.0".into(),
            ip_address: host_name.into(),
            port,
            net_id,
            name: name.into(),
        };

        sleep(Duration::from_millis(2000)).await; // Wait for the grpc service to startup

        println!("connecting to blockchain server... {}:{}", host_name, port);

        self.blockchain_server_client =
            Some(BlockchainServiceClient::connect(format!("http://{}:{}", host_name, port)).await?);

        self.blockchain_service_info = Some(info);

        Ok(())
    }

    pub(crate) async fn exec_blockchain_service_cmd(&mut self, tokens: Vec<&str>) -> Result<()> {
        if let Some(client) = self.blockchain_server_client.as_mut() {
            match tokens[0].to_lowercase().as_str() {
                "list-clients" => {
                    let res = client.get_clients(GetClientsRequest {}).await?.into_inner();
                    println!("known clients:");
                    for client_bundle in res.clients_bundles {
                        let client = client_bundle.get_client_entity()?;
                        let id = client.get_ed_pub_key()?;
                        println!(
                            "Client {}: {}",
                            client.nickname,
                            short_hex_string(id.as_ref())
                        );
                    }
                }
                "list-providers" => {
                    let res = client
                        .get_providers(GetProvidersRequest {})
                        .await?
                        .into_inner();
                    println!("known providers:");
                    for bundle in res.providers_bundles {
                        let id = bundle.get_provider_id_ed25519_public_key()?;
                        let nickname = bundle.provider_id.unwrap().nickname;
                        println!("Provider {}: {}", nickname, short_hex_string(id.as_ref()));
                    }
                }
                _ => return Err(anyhow!("unrecognized command")),
            }
            Ok(())
        } else {
            Err(anyhow!("blockchain service not setup"))
        }
    }
}
