// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::playground::Playground;
use anyhow::{anyhow, Result};
use base::snp::snp_core_types::DialupInfo;

impl Playground {
    /// Set service provider blockchain service
    pub(crate) async fn set_provider_blockchain_service(
        &mut self,
        provider_name: &str,
        name_server_info: &DialupInfo,
    ) -> Result<()> {
        let client = self.providers_admin_clients.get_mut(provider_name);
        if client.is_none() {
            return Err(anyhow!("unknown provider"));
        }

        let client_api = client.unwrap();
        client_api
            .set_blockchain_service(name_server_info.clone())
            .await
            .map_err(|_| anyhow!("failed to set name server for provider"))?
            .into_inner();

        // set provider payment amount balance on chain

        println!("🖖 blockchain server set for {}", provider_name);
        Ok(())
    }
}
