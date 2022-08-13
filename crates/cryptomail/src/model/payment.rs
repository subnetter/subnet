//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::model::extensions::Validatable;
use crate::model::types::Payment;
use anyhow::{bail, Result};
use base::hex_utils::short_hex_string;
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for Payment {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "Tx id: {}",
            short_hex_string(self.transaction_id.as_ref())
        )?;
        writeln!(f, "Amount: {}", self.amount.as_ref().unwrap())
    }
}

impl Validatable for Payment {
    fn validate(&self) -> Result<()> {
        if self.amount.is_none() {
            bail!("missing amount")
        }

        // todo: verify eth tx id here

        Ok(())
    }
}
