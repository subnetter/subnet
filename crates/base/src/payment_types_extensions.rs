// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

/*
use crate::snp::snp_payments::{Amount, Transaction, TransactionId};
use crate::snp::snp_blockchain::AddressData;
use anyhow::{anyhow, Result};

impl AddressData {
    pub fn get_balance(&mut self, coin_type: i32) -> Result<Option<Amount>> {
        if let Some(balance) = self.balances.iter().find(|b| b.coin_type == coin_type) {
            Ok(Some(balance.clone()))
        } else {
            Ok(None)
        }
    }

    pub fn add_to_balance(&mut self, amount: Amount) {
        if let Some(balance) = self.get_mut_balance(amount.coin_type) {
            balance.value += amount.value;
        } else {
            self.balances.push(amount);
        }
    }

    pub fn get_mut_balance(&mut self, coin_type: i32) -> Option<&mut Amount> {
        self.balances.iter_mut().find(|b| b.coin_type == coin_type)
    }
}

impl Transaction {
    pub fn get_tx_id(&self) -> Result<TransactionId> {
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;

        let digest = orion::hash::digest(buf.as_ref())
            .map_err(|e| anyhow!("failed to hash transaction: {}", e))?;

        Ok(TransactionId {
            data: digest.as_ref().to_vec(),
        })
    }
}
*/
