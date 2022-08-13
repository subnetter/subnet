//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::commands::service::CryptoMailService;
use crate::model::types::{ThreadBox, ThreadBoxType};
use crate::tests::setup::{create_new_test_account, test_setup, test_teardown};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn account_model() {
    test_setup().await.unwrap();

    let account = create_new_test_account().await.unwrap();
    CryptoMailService::store_account(&account).await.unwrap();
    let account1 = CryptoMailService::load_account_from_store(account.get_public_key())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        account.public_account_info.as_ref().unwrap().name,
        account1.public_account_info.as_ref().unwrap().name
    );
    assert_eq!(account, account1);

    // Test account thread-boxes api

    let inbox = ThreadBox::new(ThreadBoxType::Inbox);

    account.save_thread_box(inbox).await.unwrap();

    let _inbox1 = account
        .load_thread_box(ThreadBoxType::Inbox)
        .await
        .unwrap()
        .unwrap();

    assert!(account
        .load_thread_box(ThreadBoxType::Sent)
        .await
        .unwrap()
        .is_none());

    test_teardown().await.unwrap();
}
