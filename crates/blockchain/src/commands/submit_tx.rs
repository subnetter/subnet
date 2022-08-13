// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::service::SimpleBlockchainService;
use anyhow::Result;
use base::api_types_extensions::Signed;
use base::hex_utils::hex_string;
use base::snp::snp_blockchain::transaction::Data::{
    ClientBundle, PaymentTransaction, ProviderBundle,
};
use base::snp::snp_blockchain::{
    Account, Block, SubmitTransactionRequest, SubmitTransactionResponse, TransactionInfo,
    TransactionState, TransactionType,
};
use base::snp::snp_payments::{Amount, TransactionId};
use xactor::*;

impl SimpleBlockchainService {
    /// Update account settings
    pub(crate) async fn submit_transaction(
        request: SubmitTransactionRequest,
    ) -> Result<SubmitTransactionResponse, TransactionState> {
        let service_res = SimpleBlockchainService::from_registry().await;

        if service_res.is_err() {
            return Err(TransactionState::RejectedInternalError);
        }

        let res = service_res
            .unwrap()
            .call(SubmitTransactionMessage { request })
            .await;

        if res.is_err() {
            return Err(TransactionState::RejectedInternalError);
        }

        match res.unwrap() {
            Ok(tx_id) => Ok(SubmitTransactionResponse {
                id: Some(TransactionId { id: tx_id }),
            }),
            Err(e) => Err(e),
        }
    }
}

#[message(result = "Result<Vec<u8>, TransactionState>")]
struct SubmitTransactionMessage {
    request: SubmitTransactionRequest,
}

/// Submit tx for processing
#[async_trait::async_trait]
impl Handler<SubmitTransactionMessage> for SimpleBlockchainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: SubmitTransactionMessage,
    ) -> Result<Vec<u8>, TransactionState> {
        let req = msg.request;
        if req.transaction.as_ref().is_none() {
            return Err(TransactionState::RejectedInvalidData);
        }

        let tx = req.transaction.as_ref().unwrap();

        if tx.validate_fee().is_err() {
            return Err(TransactionState::RejectedInvalidData);
        }

        if tx.verify_signature().is_err() {
            return Err(TransactionState::RejectedInvalidSignature);
        }

        let third_party_fee_payer = tx.third_party_fee_payer();

        if third_party_fee_payer && tx.verify_fee_signature().is_err() {
            // third party tx fee signature verification failure
            return Err(TransactionState::RejectedInvalidSignature);
        }

        let tx_id_res = tx.get_tx_id();
        if tx_id_res.is_err() {
            return Err(TransactionState::RejectedInvalidData);
        }

        let tx_id = tx_id_res.unwrap();
        if tx.data.as_ref().is_none() {
            return Err(TransactionState::RejectedInvalidData);
        }

        let data = tx.data.as_ref().unwrap();

        if tx.fee.is_none() {
            return Err(TransactionState::RejectedInvalidData);
        }

        let fee = tx.fee.as_ref().unwrap();

        let sender_address = tx.get_sender_address();
        info!("tx sender address: {}", hex_string(sender_address.as_ref()));

        let res_account = SimpleBlockchainService::read_account(&sender_address).await;
        if res_account.is_err() || res_account.as_ref().unwrap().is_none() {
            return Err(TransactionState::RejectedUnknownSender);
        }

        // check counter
        let mut sender_account = res_account.unwrap().unwrap();
        if tx.counter != sender_account.nonce + 1 {
            return Err(TransactionState::RejectedInvalidCounter);
        }

        // Apply fee

        let fee_amount = fee.amount.as_ref().unwrap();

        // figure out which account is paying the fee for this tx
        let mut fee_payer_account: Account;
        match third_party_fee_payer {
            true => {
                let payer_address = tx.get_fee_payer_address();
                let payer_account_res = SimpleBlockchainService::read_account(&payer_address).await;
                if payer_account_res.is_err() || payer_account_res.as_ref().unwrap().is_none() {
                    return Err(TransactionState::RejectedUnknownSender);
                }

                fee_payer_account = payer_account_res
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .clone()
            }
            false => fee_payer_account = sender_account.clone(),
        };

        let balance = fee_payer_account.get_balance(fee_amount.coin_type);
        if balance < fee_amount.value {
            return Err(TransactionState::RejectedInsufficientFunds);
        }

        fee_payer_account.set_balance(&Amount {
            value: balance - fee_amount.value,
            coin_type: fee_amount.coin_type,
        });

        // store account pre attempted tx execution (nonce and gas
        if SimpleBlockchainService::store_account(&fee_payer_account)
            .await
            .is_err()
        {
            return Err(TransactionState::RejectedInternalError);
        }

        if !third_party_fee_payer {
            // update sender account balance as it is used below
            sender_account.set_balance(&Amount {
                value: fee_payer_account.get_balance(fee_amount.coin_type),
                coin_type: fee_amount.coin_type,
            })
        }

        let tx_type: TransactionType;

        // apply the transaction
        match data {
            PaymentTransaction(payment) => {
                if let Err(e) =
                    SimpleBlockchainService::process_payment_tx(&mut sender_account, tx, payment)
                        .await
                {
                    return Err(e);
                }
                tx_type = TransactionType::SendCoin;
            }

            ProviderBundle(provider_bundle) => {
                if let Err(e) = SimpleBlockchainService::process_provider_bundle_tx(
                    &mut sender_account,
                    tx,
                    provider_bundle,
                )
                .await
                {
                    return Err(e);
                }
                tx_type = TransactionType::SetProviderBundle;
            }

            ClientBundle(client_bundle) => {
                if let Err(e) = SimpleBlockchainService::process_client_bundle_tx(
                    &mut sender_account,
                    tx,
                    client_bundle,
                )
                .await
                {
                    return Err(e);
                }
                tx_type = TransactionType::SetClientBundle;
            }
        };

        let block_id = match SimpleBlockchainService::read_current_block_id().await {
            Ok(id) => id + 1,
            Err(_) => return Err(TransactionState::RejectedInternalError),
        };

        // Store transaction info

        let tx_info = TransactionInfo {
            id: Some(TransactionId { id: tx_id.clone() }),
            state: TransactionState::Confirmed as i32,
            transaction: Some(tx.clone()),
            transaction_type: tx_type as i32,
            block_id,
        };

        if SimpleBlockchainService::store_transaction(&tx_info)
            .await
            .is_err()
        {
            return Err(TransactionState::RejectedInternalError);
        }

        // Add the transaction to a new block if it was applied w/o an error

        let block = Block {
            id: block_id,
            transactions: vec![tx.clone()],
            sealer: None,
            signature: vec![],
        };

        // store block and update current block id
        if SimpleBlockchainService::store_block(&block).await.is_err() {
            return Err(TransactionState::RejectedInternalError);
        }

        if SimpleBlockchainService::write_current_block_id(block_id)
            .await
            .is_err()
        {
            return Err(TransactionState::RejectedInternalError);
        }

        // update sender nonce and store it
        sender_account.nonce += 1;
        if SimpleBlockchainService::store_account(&sender_account)
            .await
            .is_err()
        {
            return Err(TransactionState::RejectedInternalError);
        }

        Ok(tx_id)
    }
}
