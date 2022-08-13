// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::playground::Playground;
use anyhow::Result;

impl Playground {
    pub(crate) async fn exec_abc_magic_command(&mut self) -> Result<()> {
        // create 4 clients and 3 providers
        self.exec_provider_cmd(vec!["new", "spa.json"]).await?;
        self.exec_provider_cmd(vec!["new", "spb.json"]).await?;
        self.exec_provider_cmd(vec!["new", "spc.json"]).await?;

        let blockchain_service_info = self.blockchain_service_info.as_ref().unwrap().clone();

        self.set_provider_blockchain_service("SPA", &blockchain_service_info)
            .await?;
        self.set_provider_blockchain_service("SPB", &blockchain_service_info)
            .await?;
        self.set_provider_blockchain_service("SPC", &blockchain_service_info)
            .await?;

        println!("creating clients...");

        self.exec_client_cmd(vec!["new", "a.json"]).await?;
        self.exec_client_cmd(vec!["new", "b.json"]).await?;
        self.exec_client_cmd(vec!["new", "c.json"]).await?;
        self.exec_client_cmd(vec!["new", "d.json"]).await?;

        println!("setting blockchain service for clients...");

        self.set_client_blockchain_service("A", &blockchain_service_info)
            .await?;

        self.set_client_blockchain_service("B", &blockchain_service_info)
            .await?;

        self.set_client_blockchain_service("C", &blockchain_service_info)
            .await?;

        println!("setting client providers...");

        self.set_client_provider("A", "SPA").await?;
        self.set_client_provider("B", "SPB").await?;
        self.set_client_provider("C", "SPC").await?;
        self.set_client_provider("D", "SPA").await?;

        self.exec_blockchain_service_cmd(Vec::from(["list-providers"]))
            .await?;

        self.exec_blockchain_service_cmd(Vec::from(["list-clients"]))
            .await?;

        Ok(())
    }
}
