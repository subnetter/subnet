//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::model::types::DepositConfirmation;
use base::hex_utils::short_hex_string;
use base::time_utils;
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for DepositConfirmation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Message Id: {}", self.message_id.as_ref().unwrap())?;
        writeln!(f, "From: {:?}", self.from.as_ref().unwrap())?;
        writeln!(f, "To: {:?}", self.to.as_ref().unwrap())?;
        write!(f, "Amount: {}", self.amount.as_ref().unwrap())?;
        writeln!(
            f,
            "Block hash: {}",
            short_hex_string(self.block_hash.as_ref())
        )?;
        writeln!(f, "Block number: {}", self.block_num)?;
        writeln!(f, "Block time: {}", time_utils::local_date(self.block_time))?;

        write!(f, "Confirmations: {}", self.confirmations)
    }
}
