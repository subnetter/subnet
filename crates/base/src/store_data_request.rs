// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::snp::snp_blockchain::transaction::Data::ProviderBundle;
use crate::snp::snp_core_types::{ClientIdentityBundle, ProviderIdentityBundle};
use crate::snp::snp_decentralized_storage::{StoreDataRequest, StoredDataType};
use anyhow::{anyhow, bail, Result};

impl StoreDataRequest {
    /// Validate the data stored in this request.
    /// Returns Ok iff provided data is valid.
    /// todo: consider testing against binary hard-limit of message size
    pub fn validate_data(&self) -> Result<()> {
        match self.data_type {
            x if x == StoredDataType::ClientBundle as i32 => {
                let _ = self.get_client_bundle()?;
                Ok(())
            }
            x if x == StoredDataType::ProviderBundle as i32 => {
                let _ = self.get_provider_bundle()?;
                Ok(())
            }
            _ => {
                bail!("unexpected data type")
            }
        }
    }

    pub fn get_client_bundle(&self) -> Result<ClientIdentityBundle> {
        if self.data_type != StoredDataType::ClientBundle as i32 {
            bail!("invalid data type")
        }
        use prost::Message;
        let bundle: ClientIdentityBundle = ClientIdentityBundle::decode(self.value.as_ref())
            .map_err(|_| anyhow!("invalid data"))?;

        Ok(bundle)
    }

    pub fn get_provider_bundle(&self) -> Result<ProviderIdentityBundle> {
        if self.data_type != ProviderBundle as i32 {
            bail!("invalid data type")
        }
        use prost::Message;
        let bundle: ProviderIdentityBundle = ProviderIdentityBundle::decode(self.value.as_ref())
            .map_err(|_| anyhow!("invalid data"))?;

        Ok(bundle)
    }
}
