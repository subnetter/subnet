// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::playground::Playground;
use anyhow::{anyhow, Result};
use base::snp::snp_core_types::DialupInfo;
use base::snp::upsetter_simple_client::UserSetProviderRequest;

impl Playground {
    pub(crate) async fn set_client_provider(
        &mut self,
        client_name: &str,
        provider_name: &str,
    ) -> Result<()> {
        let client = self.clients.get_mut(client_name);
        if client.is_none() {
            return Err(anyhow!("unknown client"));
        }

        let config = self.providers_config.get(provider_name);
        if config.is_none() {
            return Err(anyhow!("unknown provider"));
        }

        let data = config.unwrap();
        let client_api = client.unwrap();
        let info = DialupInfo {
            end_point: 0,
            api_version: "0.1.0".into(),
            ip_address: data["host_name"].as_str().unwrap().into(),
            port: data["grpc_server_port"].as_u64().unwrap() as u32,
            name: data["peer_name"].as_str().unwrap().into(),
            net_id: data["net_id"].as_u64().unwrap() as u32,
        };

        println!("Provider dialup info: {}", info);

        let res = client_api
            .user_set_provider(UserSetProviderRequest {
                dialup_info: Some(info),
            })
            .await
            .map_err(|e| anyhow!(format!("failed to set provider. {}", e)))?
            .into_inner();
        let client_bundle = res
            .client_bundle
            .ok_or_else(|| anyhow!("missing client bundle"))?;

        // let the new client know about all existing clients on the network (simulating bundles discovery)
        for other_client_bundle in self.clients_bundles.values() {
            client_api
                .user_add_other_client_bundle(other_client_bundle.clone())
                .await?;
        }

        // this mocks name service
        self.clients_bundles
            .insert(client_name.to_string(), client_bundle.clone());

        // let other clients know about this new client
        for other_client in self.clients.values_mut() {
            other_client
                .user_add_other_client_bundle(client_bundle.clone())
                .await?;
        }

        println!(
            "🖖 provider {} set for client {}",
            provider_name, client_name
        );
        Ok(())
    }
}
