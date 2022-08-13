// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::features::eth_api_client::EthApiClient;
use crate::model::api::FiatCoinPrice;
use anyhow::Result;
use xactor::*;

// CryptoMailService provides crypto email services for clients
pub struct CryptoMailService {
    pub(crate) eth_api_client: Option<EthApiClient>,
    pub(crate) eth_price: Option<FiatCoinPrice>,
    pub(crate) last_eth_price_time: u64, // timestamp
}

#[async_trait::async_trait]
impl Actor for CryptoMailService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("CryptoMailService starting...");
        Ok(())
    }

    async fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("CryptoMailService stopped");
    }
}

impl Service for CryptoMailService {}

impl Default for CryptoMailService {
    fn default() -> Self {
        CryptoMailService {
            eth_api_client: None,
            eth_price: None,
            last_eth_price_time: 0,
        }
    }
}
