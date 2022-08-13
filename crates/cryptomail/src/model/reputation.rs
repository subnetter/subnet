//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::model::types::Reputation;
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for Reputation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Reputation {
    pub fn new_account_reputation(og_rank: u64) -> Self {
        Reputation {
            open_paid_messages_received: 0,
            open_paid_message_opened: 0,
            messages_reply_paid_received: 0,
            messages_reply_paid_opened: 0,
            payment_redeemed_no_open: 0,
            payment_redeemed_no_reply: 0,
            reputation_score: 0.0,
            og_rank,
            cmail_token_balance_cur_period: 0,
            last_drop_cmail_tokens: 0,
            cmail_token_balance_total_earned: 0,
        }
    }

    /// New paid to open message received which is above the account's open price
    pub fn handle_new_paid_open_message_received(&mut self) {
        self.open_paid_messages_received += 1;
    }

    /// New paid to reply message received which is about the account's reply price.
    pub fn handle_new_paid_reply_messages_received(&mut self) {
        self.messages_reply_paid_received += 1;
    }

    pub fn add_cmail_tokens(&mut self, amount: u64) {
        self.cmail_token_balance_cur_period += amount;
        self.cmail_token_balance_total_earned += amount;
    }
}
