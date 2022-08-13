// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

/*
use base::snp::snp_core_types::KeyPair;
use base::snp::snp_payments::{Address, Payment, Transaction};
use std::collections::HashMap;
use xactor::*;

#[derive(Clone, Debug)]
pub struct AddressData {
    description: String,
    address: Vec<u8>,
    key_pair: Option<KeyPair>,
}

/// WalletService is a system actor that manages cryptocurrency accounts and can sign transactions
/// Wallet should persist data to a db it is configured to use.
/// Both clients and providers use this actor.
/// In the final product, wallet data should be stored encrypted in a safe file
/// Wallet also supports monthly allowance spending budget per account to enable clients to sign
/// transactions within the budget w/o the user approval
#[derive(Debug)]
pub struct WalletService {
    /// Accounts managed by this wallet. Key is address, value is keyPair that generated it
    accounts: HashMap<Vec<u8>, AddressData>,
}

// public api
impl WalletService {
    // ListAccounts -> Address[]
    // CreateAccount(password, description) -> address
    // SignPayment(paymentData, address, password) - when password is empty - payments can only be done up to monthly spending quota
    // SignTransaction(txData, address, token)

    pub async fn new_wallet(&mut self, _password: String, _file_path: String) -> Result<()> {
        // create new wallet and new seed. Store it in _file_path
        // decrypt content with aes_key = kdf(password, salt)
        // wallet is a json file that stores a seed
        unimplemented!()
    }

    pub async fn open_wallet(&mut self, _password: String, _file_path: String) -> Result<()> {
        // load file
        // decrypt content with aes_key = kdf(password, salt)
        // wallet is a json file that stores a seed
        unimplemented!()
    }

    // return list of accounts derived from the seed
    pub async fn list_accounts(&self) -> Result<Vec<(u32, Address)>> {
        unimplemented!()
    }

    pub async fn sign_tx(&self, _account_id: u32, _transaction: &mut Transaction) -> Result<()> {
        unimplemented!()
    }

    pub async fn sign_payment(&self, _account_id: u32, _payment: &mut Payment) -> Result<()> {
        unimplemented!()
    }
}

impl Service for WalletService {}

impl Default for WalletService {
    fn default() -> Self {
        WalletService {
            accounts: HashMap::new(),
        }
    }
}

#[message(result = "Result<()>")]
pub struct Configure {
    /// user provider password
    pub password: String,
}

#[async_trait::async_trait]
impl Handler<Configure> for WalletService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: Configure) -> Result<()> {
        // todo: read encrypted accounts from db and decrypt it using kdf and provided password
        unimplemented!()
    }
}

// todo: implement  API

// TODO: add functionality to store and retrieve the key-pairs for public pre-keys
// so entities can create new pre-keys and use them in new identity bundles

#[async_trait::async_trait]
impl Actor for WalletService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("WalletService started");
        Ok(())
    }
}
*/
