// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use std::fmt;
use std::fmt::{Display, Formatter};

use crate::snp::snp_core_types::DialupInfo;

impl Display for DialupInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "name: {}, ", self.name)?;
        write!(f, "address: {}:{}, ", self.ip_address, self.port)?;
        write!(f, "api version: {}, ", self.api_version)?;
        write!(f, "net_id: {}", self.net_id)
    }
}

impl DialupInfo {
    pub fn new() -> Self {
        DialupInfo {
            end_point: 0,
            api_version: "".to_string(),
            ip_address: "".to_string(),
            port: 0,
            net_id: 0,
            name: "".to_string(),
        }
    }
}
