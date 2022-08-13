//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::consts::{ETH_ADDRESS_LEN, MSG_ID_LEN};
use crate::model::crypto_mail_contract::CryptoMailCore;
use crate::model::types::{DepositConfirmation, MessageId};
use anyhow::{anyhow, bail, Result};
use base::hex_utils::hex_string;
use bytes::Bytes;
use ethers::prelude::*;
use ethers::providers::{Http, Middleware, Provider};
use std::convert::TryFrom;

pub const ETH_TEST_ACCOUNT_0: &str = "B3Ea742871cDEa6caC58Cc1010287Ed34bD20915"; // contract owner in tests
pub const ETH_TEST_ACCOUNT_1: &str = "7aF1c55866490D6f3822B445372f774e9045087b"; // depositor / paid message sender in tests
pub const ETH_TEST_ACCOUNT_2: &str = "12Fb44DB71Ebc0dBea86070ec75A035b4a96eefA"; // receiver in tests

pub const ETH_TEST_ACCOUNT_1_SIGNATURE : &str = "b3f8e9c2d9ab40dddd9f321d3ab0873f76646be54f644c975a7d119243c723643d1477d7782aa8c9f74d35af366ce7bef7fd433fd5357379bce11f29deab2d7b01";

pub const KOVAN_ADMIN_ETH_ACCOUNT: &str = "11849aEdC41Abd218BF1A58373688001099D417B";
pub const KOVAN_ADMIN_ETH_ACCOUNT_SIGNATURE: &str = "19a3bb424d036991524dab5d505dc0a1950d41f92ab127b04c6ef41cf82a1bf83e313c4ae4251c55aab9614c46f7f71688cc84bf8744511b4368c5f6f74e1a301c";

// Ganache
pub const LOCAL_DEVNET_CONTRACT_ADDRESS: &str = "BF3c7C482b8F93CAcBd7C49A50838216969CDAA2";

// ACCOUNT_1 deposit transaction
pub const LOCAL_DEVNET_DEPOSIT_TX_1: &str =
    "e537bf4da15ce1a6456a43f85f5fb0531c66cf2b5ac89c0c04c13f38323bf8a0";

pub const LOCAL_DEVNET_MNEMONIC: &str =
    "morning other service problem choose top original stadium weapon prepare fuel guitar";

pub const DEPOSIT_TX_1_AMOUNT: &str = "100000000000000000"; // 0.1 eth

// An eth api client implemented using Infura
#[allow(dead_code, unused_variables, unused_mut)]
pub struct EthApiClient {
    net_id: u32,
    provider: Provider<Http>,
    contract: CryptoMailCore<Provider<Http>>,
}

impl EthApiClient {
    pub(crate) fn new(net_id: u32) -> Result<EthApiClient> {
        let provider_url = Self::provider_url(net_id)?;
        info!("ethereum jsonrpc api provider url: {}", provider_url);

        //let res = Provider::<dyn JsonRpcClient>::try_from(provider_url);
        let res = Provider::<Http>::try_from(provider_url);
        if res.is_err() {
            bail!(
                "failed to create eth provider: {} at {}",
                res.err().unwrap(),
                provider_url
            )
        }

        let provider = res.unwrap();
        info!("created eth jsonrpc provider");

        let contract_address = Self::core_contract_address(net_id)?;
        info!(
            "cmail contract address: {}",
            hex_string(contract_address.as_bytes())
        );

        let contract = CryptoMailCore::new(contract_address, provider.clone().into());
        info!("created cmail contract instance.");

        Ok(EthApiClient {
            net_id,
            provider,
            contract,
        })
    }

    pub(crate) fn get_eth_net_id(&self) -> u32 {
        self.net_id
    }

    // Returns the provider api url for an eth net id
    fn provider_url(net_id: u32) -> Result<&'static str> {
        match net_id {
            0 => Ok("https://mainnet.infura.io/v3/f35e500f9a9949539ba4b3cf375bb1d8"),
            42 => Ok("https://kovan.infura.io/v3/fed00d94fe7f40c0a0453f54274a464d"),
            8545 => Ok("http://127.0.0.1:8545"), // ganache local
            _ => bail!("unsupported eth net id"),
        }
    }

    fn core_contract_address_string(net_id: u32) -> Result<&'static str> {
        match net_id {
            0 => Ok("0"), // todo: mainnet contract address here
            42 => Ok("26f6104F1a81fA6D6Cd9D947C6753F67927d568a"),
            8545 => Ok(LOCAL_DEVNET_CONTRACT_ADDRESS), // gananche address should go here
            _ => bail!("missing contract address"),
        }
    }

    fn core_contract_address(net_id: u32) -> Result<Address> {
        let address_str = Self::core_contract_address_string(net_id)?;
        address_str
            .parse::<Address>()
            .map_err(|e| anyhow!("failed to create provider: {}", e))
    }

    pub(crate) async fn _get_tx_by_hash(
        &self,
        tx_hash: Bytes,
    ) -> Result<Option<Transaction>, ProviderError> {
        let hash = ethers::types::TxHash::from_slice(tx_hash.as_ref());
        self.provider.get_transaction(hash).await
    }

    pub(crate) async fn _sign(&self, message: Bytes, address_bytes: Bytes) -> Result<Vec<u8>> {
        let mut address_slice: [u8; ETH_ADDRESS_LEN] = [0x0; ETH_ADDRESS_LEN];
        address_slice.copy_from_slice(address_bytes.as_ref());
        let address = Address::try_from(address_slice)?;
        let signature = self.provider.sign(message, &address).await?;
        let signature_bytes = signature.to_vec();
        info!(
            "Eth signature: {}. Length: {}",
            hex_string(signature_bytes.as_slice()),
            signature_bytes.len()
        );
        Ok(signature_bytes)
    }
    /// Verify an eth provider signature on a binary message based on an eth signature provided as bytes and eth address
    #[allow(dead_code)]
    pub(crate) fn verify_signature(
        message: String,        // arbitrary message bytes
        signature_bytes: &[u8], // signature
        address_bytes: &[u8],   // eth address
    ) -> Result<()> {
        if address_bytes.len() != ETH_ADDRESS_LEN {
            // todo: take constant
            bail!("address must be 20 bytes")
        }
        let mut address_slice: [u8; ETH_ADDRESS_LEN] = [0x0; ETH_ADDRESS_LEN];
        address_slice.copy_from_slice(address_bytes);
        let address = Address::try_from(address_slice)?;

        info!(
            "Eth input signature bytes: {}, len: {}",
            hex_string(signature_bytes),
            signature_bytes.len()
        );

        let signature = Signature::try_from(signature_bytes)?;

        info!("signature input string: {}", message);
        signature
            .verify(message, address)
            .map_err(|e| anyhow!("Signature verification error: {:?}", e))
    }

    /// Returns on-chain deposit data for a message identified by id or none if doesn't exist on the chin.
    pub(crate) async fn get_deposit_data(
        &self,
        message_id: &MessageId,
    ) -> Result<Option<DepositConfirmation>> {
        // use deposit contract abi to read storage of contract and parse it into DepositConfirmation

        let mut msg_id_bytes: [u8; MSG_ID_LEN] = [0x0; MSG_ID_LEN];
        msg_id_bytes.copy_from_slice(message_id.get_message_id_bytes().as_ref());
        info!("message id bytes: {}", hex_string(msg_id_bytes.as_ref()));

        use crate::model::types::Amount;
        use crate::model::types::EthAddress;
        use crate::model::types::Token;

        let res = self.provider.get_block_number().await;
        if res.is_err() {
            error!("failed to get block num from provider: {:?}", res.err());
            bail!("failed to get current block num from provider")
        }

        let current_block_num = res.unwrap().as_u64();
        info!("Current block number: {}", current_block_num);

        // Get deposit for the message id from the CryptoMail smart-contract
        match self.contract.get_deposit(msg_id_bytes).call().await {
            Ok(data) => {
                // todo: id data is all 0s - return Ok(None)

                // get the deposit block data
                let deposit_block_num = data.3.as_u64();

                if deposit_block_num == 0 {
                    // mappings with 0 values
                    info!(
                        "0 values deposit data for deposit for message id: {}",
                        hex_string(msg_id_bytes.as_ref())
                    );
                    return Ok(None);
                }

                let deposit_block = self
                    .provider
                    .get_block(deposit_block_num)
                    .await
                    .map_err(|e| anyhow!("failed to get current block num: {:?}", e))?
                    .ok_or_else(|| anyhow!("failed to get deposit block data"))?;

                info!(
                    "got block data for deposit block number: {}",
                    deposit_block_num
                );

                info!(
                    "got deposit non-zero data for contract from chain: {:?}",
                    data
                );

                // data layout from abi: amount, depositor, recipient, block
                Ok(Some(DepositConfirmation {
                    message_id: Some(message_id.clone()),
                    amount: Some(Amount {
                        token: Token::Eth as i32, // only eth deposits are supported for now
                        amount: data.0.to_string(),
                    }),
                    from: Some(EthAddress {
                        bytes: data.1.as_bytes().to_vec(),
                    }),
                    to: Some(EthAddress {
                        bytes: data.2.as_bytes().to_vec(),
                    }),
                    block_num: data.3.as_u64(), // block this deposit was made on
                    block_hash: deposit_block.hash.unwrap().as_bytes().to_vec(),
                    confirmations: current_block_num - deposit_block_num,
                    block_time: deposit_block.timestamp.as_u64(),
                }))
            }
            Err(e) => {
                info!(
                    "failed to get deposit data for contract due to error: {:?}",
                    e
                );
                Err(anyhow!("failed to get deposit for message id: {:?}", e))
            }
        }
    }
}
