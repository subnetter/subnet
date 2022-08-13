//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::consts::ETH_PRICE_CACHE_DUR_NANO_SECS;
use crate::model::api::{FiatCoinPrice, GetCoinPriceRequest, GetCoinPriceResponse};
use anyhow::Result;
use chrono::Utc;
use std::collections::HashMap;
use xactor::*;

impl CryptoMailService {
    pub(crate) async fn get_coin_price(
        request: GetCoinPriceRequest,
    ) -> Result<GetCoinPriceResponse> {
        let service = CryptoMailService::from_registry().await?;
        service.call(GetCoinPriceMessage { request }).await?
    }
}

#[message(result = "Result<GetCoinPriceResponse>")]
pub struct GetCoinPriceMessage {
    pub request: GetCoinPriceRequest,
}

#[async_trait::async_trait]
impl Handler<GetCoinPriceMessage> for CryptoMailService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetCoinPriceMessage,
    ) -> Result<GetCoinPriceResponse> {
        // for now we ignore all request params (symbol, currency) and just return ETH in USD

        let now = Utc::now().timestamp_nanos();

        if self.eth_price.is_none()
            || (now - self.last_eth_price_time as i64).abs() > ETH_PRICE_CACHE_DUR_NANO_SECS
        {
            // call the api
            let resp = reqwest::get("https://min-api.cryptocompare.com/data/price?fsym=ETH&tsyms=USD&api_key=1f314f2b94d63d8821beec5a67250129972cee13df5f5dedb572b95eeff0e41d")
                .await?
                .json::<HashMap<String, f32>>()
                .await?;

            // for now we assume USD result

            self.eth_price = Some(FiatCoinPrice {
                currency: "USD".to_string(),
                price: resp["USD"],
            });

            info!("Eth price USD: {}", self.eth_price.as_ref().unwrap().price);

            self.last_eth_price_time = now as u64
        }

        Ok(GetCoinPriceResponse {
            prices: vec![self.eth_price.as_ref().unwrap().clone()],
        })
    }
}
