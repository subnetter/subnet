// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::playground::{ChildGuard, Playground};
use anyhow::{anyhow, Result};
use base::snp::upsetter_simple_client::simple_client_user_service_client::SimpleClientUserServiceClient;
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;

impl Playground {
    pub(crate) async fn exec_client_cmd(&mut self, tokens: Vec<&str>) -> Result<()> {
        let cmd = tokens[0].to_lowercase();
        match cmd.as_str() {
            "new" => {
                let conf_file = tokens[1].to_string();
                match self.new_client(conf_file).await {
                    Ok(name) => {
                        println!("🖖 created new client {}.", name);
                        Ok(())
                    }
                    Err(e) => Err(anyhow!(format!("{:?}", e))),
                }
            }
            _ => Err(anyhow!("unrecognized command")),
        }
    }

    async fn new_client(&mut self, conf_file_path: String) -> Result<String> {
        let config_data = self.read_json_from_file(conf_file_path.clone()).await?;
        let client_app = Command::new("./client-app")
            .args(&["-c", conf_file_path.as_str()])
            .spawn()
            .unwrap();

        self.proc_guards.push(ChildGuard(client_app));
        let client_name = config_data["client_name"]
            .as_str()
            .ok_or_else(|| anyhow!("missing data"))?
            .to_string();

        let port = config_data["grpc_server_port"].as_u64().unwrap() as i32;
        let host_name = config_data["host_name"].as_str().unwrap();

        sleep(Duration::from_millis(2000)).await; // Wait for the grpc service to startup

        println!("connecting to client...");

        let client =
            SimpleClientUserServiceClient::connect(format!("http://{}:{}", host_name, port))
                .await?;
        self.clients.insert(client_name.clone(), client);
        self.clients_config
            .insert(client_name.clone(), config_data.clone());
        Ok(client_name)
    }
}
