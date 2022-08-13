// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::service::SimpleBlockchainService;
use base::snp::snp_blockchain::{Account, PaymentTransactionData, Transaction, TransactionState};
use base::snp::snp_payments::Amount;

impl SimpleBlockchainService {
    pub(crate) async fn process_payment_tx(
        sender_account: &mut Account,
        _tx: &Transaction,
        data: &PaymentTransactionData,
    ) -> Result<(), TransactionState> {
        if data.receiver.is_none() {
            return Err(TransactionState::RejectedInternalError);
        }

        if data.coins.is_none() {
            return Err(TransactionState::RejectedInvalidData);
        }

        if data.receiver.is_none() {
            // missing receiver address in tx data
            return Err(TransactionState::RejectedInvalidData);
        }

        let transfer_amount = data.coins.as_ref().unwrap();
        let balance = sender_account.get_balance(transfer_amount.coin_type);
        if balance < transfer_amount.value {
            return Err(TransactionState::RejectedInsufficientFunds);
        }

        let receiver_address = data.receiver.as_ref().unwrap();
        let res = SimpleBlockchainService::read_account(&receiver_address.data).await;
        if res.is_err() {
            // db error
            return Err(TransactionState::RejectedInternalError);
        }

        let mut receiver = match res.unwrap() {
            None => Account {
                address: Some(data.receiver.as_ref().unwrap().clone()),
                nonce: 0,
                balances: vec![],
            },
            Some(a) => a,
        };

        // end of validation - update state below
        //

        sender_account.set_balance(&Amount {
            value: balance - transfer_amount.value,
            coin_type: transfer_amount.coin_type,
        });

        receiver.set_balance(&Amount {
            value: receiver.get_balance(transfer_amount.coin_type) + transfer_amount.value,
            coin_type: transfer_amount.coin_type,
        });

        if SimpleBlockchainService::store_account(sender_account)
            .await
            .is_err()
        {
            return Err(TransactionState::RejectedInternalError);
        }

        if SimpleBlockchainService::store_account(&receiver)
            .await
            .is_err()
        {
            return Err(TransactionState::RejectedInternalError);
        }

        Ok(())
    }
}
