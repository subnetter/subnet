//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::model::types::DepositData;
use base::time_utils;
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for DepositData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "Last verify attempt: {}",
            time_utils::local_date(self.last_verify_attempt)
        )?;

        writeln!(
            f,
            "Verify attempts: {}",
            time_utils::local_date(self.verify_attempts)
        )?;

        if let Some(confirmation) = self.deposit_confirmation.as_ref() {
            return writeln!(f, "Deposit Data: {}", confirmation);
        };

        Ok(())
    }
}
