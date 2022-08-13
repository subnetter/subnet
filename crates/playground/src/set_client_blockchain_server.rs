// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::playground::Playground;
use anyhow::{anyhow, Result};
use base::snp::snp_core_types::DialupInfo;
use base::snp::upsetter_simple_client::SetBlockchainServiceRequest;

impl Playground {
    pub(crate) async fn set_client_blockchain_service(
        &mut self,
        client_name: &str,
        name_server_info: &DialupInfo,
    ) -> Result<()> {
        let client = self.clients.get_mut(client_name);
        if client.is_none() {
            return Err(anyhow!("unknown client"));
        }

        let client_api = client.unwrap();
        let _ = client_api
            .set_blockchain_service(SetBlockchainServiceRequest {
                dialup_info: Some(name_server_info.clone()),
            })
            .await
            .map_err(|e| anyhow!(format!("failed to set blockchain server for client. {}", e)))?
            .into_inner();

        println!("🖖 blockchain server set for client {}", client_name);
        Ok(())
    }
}
