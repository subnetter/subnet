// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::snp::snp_blockchain::Account;
use crate::snp::snp_payments::Amount;

impl Account {
    pub fn get_balance(&self, coin_type: i32) -> u64 {
        for a in &self.balances {
            if a.coin_type == coin_type {
                return a.value;
            }
        }
        0
    }

    pub fn set_balance(&mut self, amount: &Amount) {
        for a in self.balances.iter_mut() {
            if a.coin_type == amount.coin_type {
                a.value = amount.value;
                return;
            }
        }
        self.balances.push(amount.clone())
    }
}
