//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::model::extensions::Validatable;
use crate::model::types::Settings;
use anyhow::Result;
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for Settings {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Public list: {}, ", self.public_list_account)?;
        write!(f, "Active: {}", self.active)
    }
}

impl Validatable for Settings {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
