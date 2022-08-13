// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use chrono::prelude::*;

const A_BILLY: u64 = 1_000_000_000;

/// Returns local date for a unix epoch timestamp in nano
pub fn local_date(time_stamp_nano: u64) -> DateTime<Local> {
    let naive = NaiveDateTime::from_timestamp(
        (time_stamp_nano / A_BILLY) as i64,
        (time_stamp_nano % A_BILLY) as u32,
    );

    Local.from_local_datetime(&naive).unwrap()
}
