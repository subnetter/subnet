// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::Result;
use base::api_types_extensions::Signed;
use base::snp::snp_core_types::ProviderSignedClientIdentityBundle;
use xactor::*;

#[message(result = "Result<()>")]
pub struct AddOtherClientBundle(pub ProviderSignedClientIdentityBundle);

#[async_trait::async_trait]
impl Handler<AddOtherClientBundle> for SimpleClient {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: AddOtherClientBundle) -> Result<()> {
        msg.0.verify_signature()?;
        let key = msg.0.get_client_id()?;
        self.other_clients.insert(key, msg.0);
        Ok(())
    }
}
