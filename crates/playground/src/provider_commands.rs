// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::playground::{ChildGuard, Playground};
use anyhow::{anyhow, Result};
use base::snp::upsetter_server_admin::server_admin_service_client::ServerAdminServiceClient;
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;

impl Playground {
    pub(crate) async fn exec_provider_cmd(&mut self, tokens: Vec<&str>) -> Result<()> {
        let cmd = tokens[0].to_lowercase();
        match cmd.as_str() {
            "new" => {
                let conf_file = tokens[1].to_string();
                match self.new_provider(conf_file).await {
                    Ok(name) => {
                        println!("🖖 created new provider {}", name);
                        Ok(())
                    }
                    Err(e) => Err(anyhow!(format!("{:?}", e))),
                }
            }
            _ => Err(anyhow!("unrecognized command")),
        }
    }

    pub(crate) async fn new_provider(&mut self, conf_file_path: String) -> Result<String> {
        let config_data = self.read_json_from_file(conf_file_path.clone()).await?;

        let provider_app = Command::new("./server-app")
            .args(&["-c", conf_file_path.as_str()])
            .spawn()
            .unwrap();

        let provider_name = config_data["peer_name"]
            .as_str()
            .ok_or_else(|| anyhow!("missing name"))?
            .to_string();

        self.providers_config
            .insert(provider_name.clone(), config_data.clone());
        self.proc_guards.push(ChildGuard(provider_app));

        sleep(Duration::from_millis(1000)).await;

        let port = config_data["grpc_server_port"].as_u64().unwrap() as i32;
        let host_name = config_data["host_name"].as_str().unwrap();

        // get and store admin client to these providers
        let admin_client =
            ServerAdminServiceClient::connect(format!("http://{}:{}", host_name, port)).await?;

        self.providers_admin_clients
            .insert(provider_name.clone(), admin_client);

        Ok(provider_name)
    }
}
