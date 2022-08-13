//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::provider_id::ProviderIdService;
use anyhow::{anyhow, Result};
use base::snp::snp_core_types::{PrivateProviderIdentityBundle, ProviderNetInfo};
use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use xactor::*;
impl Service for ProviderIdService {}

#[async_trait::async_trait]
impl Actor for ProviderIdService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("ProviderIdService started");
        Ok(self.init().await?)
    }
}

/// Get this provider public ed25519 id
#[message(result = "Result<PublicKey>")]
pub struct GetId;

/// GetId
#[async_trait::async_trait]
impl Handler<GetId> for ProviderIdService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetId,
    ) -> Result<ed25519_dalek::PublicKey> {
        Ok(self
            .provider_id
            .as_ref()
            .ok_or_else(|| anyhow!("internal error - missing provider id"))?
            .public)
    }
}

/////////////////////

/// Get a provider's identity bundle identified by an id.
/// Returns PrivateProviderIdentityBundle or None if no bundle is saved with this id by this provider
#[message(result = "Result<Option<PrivateProviderIdentityBundle>>")]
pub struct GetIdentityBundle(pub u64);

#[async_trait::async_trait]
impl Handler<GetIdentityBundle> for ProviderIdService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetIdentityBundle,
    ) -> Result<Option<PrivateProviderIdentityBundle>> {
        Ok(self.get_bundle(msg.0).await?)
    }
}
/////////////////////

/// Get provider's payment account key-pair.
#[message(result = "Result<Keypair>")]
pub struct GetPaymentAccountKeypair;

#[async_trait::async_trait]
impl Handler<GetPaymentAccountKeypair> for ProviderIdService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetPaymentAccountKeypair,
    ) -> Result<Keypair> {
        // We currently assume keypair exists in state and so we unwrap
        let key_pair = self.payments_account_id.as_ref().unwrap();

        Ok(Keypair {
            public: key_pair.public.clone(),
            secret: SecretKey::from_bytes(key_pair.secret.as_ref())?,
        })
    }
}

/////////////////////

/// Get provider's current signed net info
#[message(result = "Result<ProviderNetInfo>")]
pub struct GetProviderNetInfo;

#[async_trait::async_trait]
impl Handler<GetProviderNetInfo> for ProviderIdService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetProviderNetInfo,
    ) -> Result<ProviderNetInfo> {
        Ok(self.get_provider_net_info().await?)
    }
}

/////////////////////

/// Get the provider's current identity bundle
#[message(result = "Result<PrivateProviderIdentityBundle>")]
pub struct GetCurrentIdentityBundle();

#[async_trait::async_trait]
impl Handler<GetCurrentIdentityBundle> for ProviderIdService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetCurrentIdentityBundle,
    ) -> Result<PrivateProviderIdentityBundle> {
        self.get_identity_bundle(false).await
    }
}

/////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use base::test_helpers::enable_logger;
    use db::db_service::{DatabaseService, Destroy};

    #[tokio::test]
    async fn test_provider_init_first_run() {
        enable_logger();

        let db = DatabaseService::from_registry().await.unwrap();
        let provider = ProviderIdService::from_registry().await.unwrap();
        let bundle: PrivateProviderIdentityBundle = provider
            .call(GetCurrentIdentityBundle {})
            .await
            .unwrap()
            .unwrap();

        debug!("bundle id: {}", bundle.public_bundle.unwrap().time_stamp);

        let _ = db.call(Destroy).await.expect("failed to delete the db");
    }
}
