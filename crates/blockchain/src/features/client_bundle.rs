// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::consts::CLIENTS_BUNDLES_CF;
use crate::service::SimpleBlockchainService;
use anyhow::{anyhow, bail, Result};
use base::api_types_extensions::Signed;
use base::snp::snp_blockchain::{
    Account, ClientBundleTransactionData, Transaction, TransactionState,
};
use base::snp::snp_core_types::ProviderSignedClientIdentityBundle;
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};

impl SimpleBlockchainService {
    /// Process a provider bundle transaction
    pub(crate) async fn process_client_bundle_tx(
        sender_account: &mut Account,
        _tx: &Transaction,
        data: &ClientBundleTransactionData,
    ) -> Result<(), TransactionState> {
        if data.client_bundle.is_none() {
            return Err(TransactionState::RejectedInvalidData);
        }

        let bundle = data.client_bundle.as_ref().unwrap();
        if bundle.verify_signature().is_err() {
            return Err(TransactionState::RejectedInvalidSignature);
        }

        if bundle.client_bundle.is_none() {
            return Err(TransactionState::RejectedInvalidData);
        }

        let client_bundle = bundle.client_bundle.as_ref().unwrap();

        if client_bundle.address.as_ref().is_none() {
            return Err(TransactionState::RejectedInvalidData);
        }

        if sender_account.address.as_ref().is_none() {
            return Err(TransactionState::RejectedInvalidData);
        }

        if client_bundle.provider_bundle.is_none() {
            return Err(TransactionState::RejectedInvalidData);
        }

        let provider_bundle = client_bundle.provider_bundle.as_ref().unwrap();
        if provider_bundle.address.is_none() {
            return Err(TransactionState::RejectedInvalidData);
        }

        let provider_payment_address = provider_bundle.address.as_ref().unwrap();

        // check that tx sender payment account is the provided published coin account
        // only providers can publish service bundles for their clients
        // Providers must publish their client bundles to the blockchain for clients to be able to receive messages and providers earn inome

        if provider_payment_address.data != sender_account.address.as_ref().unwrap().data {
            return Err(TransactionState::RejectedInvalidData);
        }

        // todo: check that if there's an already client bundle stored in global state.
        // the time-stamp of the new one is greater than the one before so only newer updates are accepted
        // also check that very old bundles can't be published. e.g. created more than few months ago

        if SimpleBlockchainService::store_client_bundle(bundle)
            .await
            .is_err()
        {
            return Err(TransactionState::RejectedInternalError);
        }

        Ok(())
    }

    /// Load blockchain account from store
    pub(crate) async fn read_client_bundle(
        public_key: &[u8],
    ) -> Result<Option<ProviderSignedClientIdentityBundle>> {
        let key = public_key.to_vec();
        if let Some(data) = DatabaseService::read(ReadItem {
            key: Bytes::from(key),
            cf: CLIENTS_BUNDLES_CF,
        })
        .await?
        {
            use prost::Message;
            let bundle = ProviderSignedClientIdentityBundle::decode(data.0.as_ref())?;
            Ok(Some(bundle))
        } else {
            Ok(None)
        }
    }

    /// Save account in store
    pub(crate) async fn store_client_bundle(
        bundle: &ProviderSignedClientIdentityBundle,
    ) -> Result<()> {
        let client_bundle = bundle
            .client_bundle
            .as_ref()
            .ok_or_else(|| anyhow!("missing client bundle"))?;

        let client_pub_key = client_bundle
            .client_id
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
                key: Bytes::from(client_pub_key.key.clone()),
                value: Bytes::from(data.to_vec()),
            },
            cf: CLIENTS_BUNDLES_CF,
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
