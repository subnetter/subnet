//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::model::types::Amount;

use std::fmt;
use std::fmt::{Display, Formatter};
impl Display for Amount {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // todo: parse to proper token name
        write!(f, "Token id: {}, ", self.token)?;
        write!(f, "Amount: {}", self.amount)
    }
}
