// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::Result;
use base::snp::snp_blockchain::blockchain_service_client::BlockchainServiceClient;
use base::snp::snp_blockchain::SetBalanceRequest;
use base::snp::snp_core_types::DialupInfo;
use base::snp::snp_payments::{Amount, CoinType};
use xactor::*;

#[message(result = "Result<()>")]
pub(crate) struct SetBlockchainService {
    pub(crate) info: DialupInfo,
}

/// Set the blockchain service for this client
#[async_trait::async_trait]
impl Handler<SetBlockchainService> for SimpleClient {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetBlockchainService) -> Result<()> {
        let dialup_info = msg.info;

        info!(
            "connecting to blockchain service... {} {}",
            dialup_info.ip_address, dialup_info.port
        );

        self.blockchain_service_client = Some(
            BlockchainServiceClient::connect(format!(
                "http://{}:{}",
                dialup_info.ip_address, dialup_info.port
            ))
            .await?,
        );

        info!(
            "Blockchain service set to {} {}",
            dialup_info.ip_address, dialup_info.port
        );

        // Set genesis balance for this client
        let payment_address = self.get_payment_address()?;
        self.blockchain_service_client
            .as_mut()
            .unwrap()
            .set_balance(SetBalanceRequest {
                address: Some(payment_address.clone()),
                amount: Some(Amount {
                    value: 1000,
                    coin_type: CoinType::Core as i32,
                }),
            })
            .await?;

        Ok(())
    }
}
