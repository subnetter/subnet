// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::consts::PROVIDERS_BUNDLES_CF;
use crate::service::SimpleBlockchainService;
use anyhow::{anyhow, bail, Result};
use base::api_types_extensions::Signed;
use base::snp::snp_blockchain::{
    Account, ProviderBundleTransactionData, Transaction, TransactionState,
};
use base::snp::snp_core_types::ProviderIdentityBundle;
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};

impl SimpleBlockchainService {
    /// Process a provider bundle transaction
    pub(crate) async fn process_provider_bundle_tx(
        sender_account: &mut Account,
        _tx: &Transaction,
        data: &ProviderBundleTransactionData,
    ) -> Result<(), TransactionState> {
        if data.provider_bundle.is_none() {
            return Err(TransactionState::RejectedInvalidData);
        }

        let bundle = data.provider_bundle.as_ref().unwrap();

        if bundle.verify_signature().is_err() {
            return Err(TransactionState::RejectedInvalidSignature);
        }

        // check that sender_account is the payment account in the bundle
        if bundle.address.as_ref().is_none() {
            return Err(TransactionState::RejectedInvalidData);
        }

        let bundle_payment_address = bundle.address.as_ref().unwrap();

        if sender_account.address.as_ref().is_none() {
            return Err(TransactionState::RejectedInvalidData);
        }

        if bundle_payment_address.data != sender_account.address.as_ref().unwrap().data {
            // bundle can only be updated with transaction from the bundle's payment account
            return Err(TransactionState::RejectedInvalidData);
        }

        if SimpleBlockchainService::store_provider_bundle(bundle)
            .await
            .is_err()
        {
            return Err(TransactionState::RejectedInternalError);
        }

        // update provider  bundle
        Ok(())
    }

    /// Load blockchain account from store
    pub(crate) async fn read_provider_bundle(
        public_key: &[u8],
    ) -> Result<Option<ProviderIdentityBundle>> {
        let key = public_key.to_vec();
        if let Some(data) = DatabaseService::read(ReadItem {
            key: Bytes::from(key),
            cf: PROVIDERS_BUNDLES_CF,
        })
        .await?
        {
            use prost::Message;
            let bundle = ProviderIdentityBundle::decode(data.0.as_ref())?;
            Ok(Some(bundle))
        } else {
            Ok(None)
        }
    }

    /// Save account in store
    pub(crate) async fn store_provider_bundle(bundle: &ProviderIdentityBundle) -> Result<()> {
        let key = bundle
            .provider_id
            .as_ref()
            .ok_or_else(|| anyhow!("missing provider id address"))?
            .public_key
            .as_ref()
            .ok_or_else(|| anyhow!("missing public key"))?;

        use prost::Message;
        let mut data = Vec::with_capacity(bundle.encoded_len());
        if bundle.encode(&mut data).is_err() {
            bail!("internal server error - failed to encode provider bundle")
        };

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(key.key.clone()),
                value: Bytes::from(data.to_vec()),
            },
            cf: PROVIDERS_BUNDLES_CF,
            ttl: 0,
        })
        .await
        .map_err(|e| {
            anyhow!(
                "internal server error - failed to save provider bundle: {}",
                e
            )
        })?;

        Ok(())
    }
}
