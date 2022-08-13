//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::model::types::MessageServerData;
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for MessageServerData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Opened: {}", self.opened)?;
        writeln!(f, "Replied: {}", self.replied)?;

        if self.deposit_data.is_some() {
            writeln!(f, "Deposit data: {}", self.deposit_data.as_ref().unwrap())?;
        }

        Ok(())
    }
}
