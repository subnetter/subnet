// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;

use base::server_config_service::{GetValue, ServerConfigService, SetValue};
use base::test_helpers::enable_logger;
use xactor::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_read_write() {
    enable_logger();

    let addr = ServerConfigService::from_registry()
        .await
        .expect("failed to get config service");

    let _ = addr
        .call(SetValue {
            key: "key1".into(),
            value: "value1".into(),
        })
        .await
        .expect("failed to write value");

    let data = addr
        .call(GetValue("key1".into()))
        .await
        .expect("failed to call get value")
        .expect("get value returned an error");

    assert_eq!(data, "value1");
    debug!("foo");
}
