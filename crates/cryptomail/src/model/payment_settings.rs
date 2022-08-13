//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::features::eth_api_client::EthApiClient;
// use crate::model::extensions::Validatable;
use crate::model::types::{PaidActionType, PaymentSettings};
use anyhow::{bail, Result};
use base::hex_utils::{hex_string, short_hex_string};
use bytes::Bytes;
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for PaymentSettings {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // todo: go over paid actions and print them
        // write!(f, "Price open: {}, ", self.price_open.as_ref().unwrap())?;
        // write!(f, "Price reply: {}, ", self.price_reply.as_ref().unwrap())?;
        write!(
            f,
            "Eth address: {:?}, ",
            hex_string(&self.eth_address.as_ref().unwrap().bytes)
        )?;
        write!(
            f,
            "Eth signature: {}",
            short_hex_string(self.eth_signature.as_ref())
        )
    }
}

// todo: add eth sign

impl PaymentSettings {
    /// Sets the signature field to an eth signature on the eth public key
    pub(crate) async fn _eth_sign(&mut self, eth_client: &EthApiClient) -> Result<()> {
        self.eth_signature = eth_client
            ._sign(
                Bytes::from(self.eth_address.as_ref().unwrap().bytes.clone()),
                Bytes::from(self.eth_address.as_ref().unwrap().bytes.clone()),
            )
            .await?;

        Ok(())
    }
}

impl PaymentSettings {
    /// validates the payment settings using current account name
    pub(crate) fn validate(&self, account_name: &str) -> Result<()> {
        if self.eth_address.is_none() {
            bail!("missing eth signature")
        }

        if let Some(address) = &self.eth_address {
            info!(
                "validating signature for eth address: {} and account: {}",
                hex_string(address.bytes.as_ref()),
                account_name
            );

            let signature_slice = self.eth_signature.as_slice();
            info!(
                "Signature data: {}. Len: {}",
                hex_string(signature_slice),
                signature_slice.len()
            );

            let message = account_name.to_owned() + hex_string(address.bytes.as_ref()).as_str();

            // verify that the user eth-signed his eth address

            EthApiClient::verify_signature(
                message,
                self.eth_signature.as_slice(),
                address.bytes.as_ref(),
            )?;

            info!("😃 eth signature on address successfully verified!")
        } else {
            bail!("missing eth address")
        }

        if self.paid_actions.len() < 2 {
            // todo: check that the 2 actions are for open and reply
            bail!("expected open and reply paid actions")
        }

        let mut found_open_action = false;
        let mut found_reply_action = false;

        for a in self.paid_actions.iter() {
            if a.price.is_none() {
                bail!("missing action price")
            }

            if a.paid_action_type == 0 {
                bail!("missing action type")
            }

            if a.paid_action_type == PaidActionType::Open as i32 {
                found_open_action = true
            } else if a.paid_action_type == PaidActionType::Reply as i32 {
                found_reply_action = true
            }
        }

        if !(found_reply_action || found_open_action) {
            bail!("Open or reply payments must be specified")
        }

        Ok(())
    }
}
