//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use chrono::Utc;
use cryptomail::features::eth_api_client::*;
use cryptomail::model::api::cryptomail_api_service_client::CryptomailApiServiceClient;
use cryptomail::model::api::{
    CreateAccountRequest, CreateAccountResult, GetMessageDepositDataRequest, GetThreadBoxesRequest,
    NewThreadRequest, NewThreadResult, OpenMessageRequest, ReplyRequest,
};
use cryptomail::model::extensions::{Signed, Signer};
use cryptomail::model::types::{
    Amount, Message, MessageContent, MessageId, MessageUserdata, PaidActionType, Payment,
    PublicKey, ThreadBoxType, Token,
};
use cryptomail::tests::setup::{
    create_account_request, get_grpc_server_connection_string, test_setup, test_teardown,
};
use ed25519_dalek::{Keypair, Signer as EdSigner};
use log::*;
use rand_core::{OsRng, RngCore};
use tonic::transport::Channel;
use x25519_dalek::{EphemeralSecret, StaticSecret};

/// Basic interaction: send a new paid to open message to a recipient. Recipient opens it and replies.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn thread_paid_open_message() {
    test_setup().await.unwrap();

    let grpc_server_address = get_grpc_server_connection_string().await.unwrap();
    let mut api_service =
        CryptomailApiServiceClient::connect(format!("http://{}", grpc_server_address))
            .await
            .unwrap();

    // create account1: paid message recipient account as messages can only be sent to existing accounts
    // this should with eth localnet account[1]
    //
    let (mut create_account1_request, key_pair, recipient_pre_key_secret) =
        create_account_request(ETH_TEST_ACCOUNT_2.to_string(), "Account 1".to_string())
            .await
            .unwrap();

    create_account1_request.sign(&key_pair).unwrap();

    let response = api_service
        .create_account(create_account1_request.clone())
        .await
        .unwrap()
        .into_inner();

    assert_eq!(response.result, CreateAccountResult::Created as i32);
    assert!(response.account.is_some());

    // In the real product, sender knows the pre-key from the recipient account meta-data
    // which is available via the public directory or via querying a pre-key for account name
    let recipient_pub_pre_key = create_account1_request.get_x_pre_key().unwrap();
    let recipient_pub_key = create_account1_request.public_key.as_ref().unwrap().clone();

    // create paid message sender account - account2
    let (mut create_account2_request, account2_key_pair, _) =
        create_account_request(ETH_TEST_ACCOUNT_1.to_string(), "Account 2".to_string())
            .await
            .unwrap();

    // update account name
    let info = create_account2_request
        .public_account_info
        .as_mut()
        .unwrap();
    info.name = "zifton the cat".into();
    info.sign(&account2_key_pair).unwrap();

    create_account2_request.sign(&account2_key_pair).unwrap();
    let response = api_service
        .create_account(create_account2_request.clone())
        .await
        .unwrap()
        .into_inner();

    assert_eq!(response.result, CreateAccountResult::Created as i32);
    assert!(response.account.is_some());

    // create and encrypt message from sender to receiver - this happens in the sender's client
    //
    let simple_message = MessageContent::new_basic_message();
    let eph_secret = EphemeralSecret::new(OsRng);
    let x_eph_pub_key: x25519_dalek::PublicKey = x25519_dalek::PublicKey::from(&eph_secret);
    let eph_pub_key: PublicKey = x_eph_pub_key.into();
    let shared_secret = eph_secret.diffie_hellman(&recipient_pub_pre_key);
    let enc_message = simple_message.encrypt(shared_secret.as_bytes()).unwrap();

    // generate random unique thread id

    let mut thread_id = [0u8; 8];
    OsRng.fill_bytes(&mut thread_id);

    let mut message_thread_id = [0u8; 8];
    OsRng.fill_bytes(&mut message_thread_id);

    let message_id = MessageId {
        message_thread_id: message_thread_id.to_vec(),
        thread_id: thread_id.to_vec(),
    };
    let auth_pub_key = create_account2_request.public_key.as_ref().unwrap();

    let message_user_data = MessageUserdata {
        message_id: Some(message_id.clone()),
        sender_public_key: Some(auth_pub_key.clone()),
        created: Utc::now().timestamp_nanos() as u64,
        payment: Some(Payment {
            amount: Some(Amount {
                token: Token::Eth as i32,
                amount: DEPOSIT_TX_1_AMOUNT.to_string(),
            }),
            transaction_id: hex::decode(LOCAL_DEVNET_DEPOSIT_TX_1).unwrap(),
            paid_action_type: PaidActionType::Open as i32,
        }),
        reply_to: [0u8; 8].to_vec(),
        recipient_public_key: Some(recipient_pub_key.clone()),
        eph_pub_key: Some(eph_pub_key), // used for enc content of this message
        recipient_pre_key_id: 0,        // this should come from the receiver public account info
        content: enc_message,
    };

    use prost::Message;
    let mut data = Vec::with_capacity(message_user_data.encoded_len());
    message_user_data.encode(&mut data).unwrap();
    let message_user_data_signature = account2_key_pair.sign(data.as_slice()).as_ref().to_vec();

    let mut new_thread_request = NewThreadRequest {
        time_stamp: Utc::now().timestamp_nanos() as u64,
        public_key: Some(auth_pub_key.clone()),
        message_user_data: data,
        message_user_data_signature,
        message_id: Some(message_id.clone()),
        signature: vec![],
    };
    new_thread_request.sign(&account2_key_pair).unwrap();

    // extra verification here before sending the request...
    new_thread_request.verify_signature().unwrap();
    let enc_content = message_user_data.content.clone();

    // Send new message and new thread
    let response = api_service
        .new_thread(new_thread_request)
        .await
        .unwrap()
        .into_inner();
    assert_eq!(response.result, NewThreadResult::Created as i32);

    // quick test recipient decryption to verify the cryptosystem
    let shared_secret_1 = recipient_pre_key_secret.diffie_hellman(&x_eph_pub_key);
    assert_eq!(shared_secret.as_bytes(), shared_secret_1.as_bytes());
    let dec_content =
        MessageContent::decrypt(enc_content.as_slice(), shared_secret.as_bytes()).unwrap();
    dec_content.subject.unwrap();
    dec_content.body.unwrap();

    // test recipient message handling....
    test_new_message(
        api_service,
        create_account1_request,
        key_pair,
        recipient_pre_key_secret,
        message_id,
    )
    .await
    .unwrap();

    test_teardown().await.unwrap();
}

/// test that a new message in a new thread appears in users inbox, opening it and replying to it
async fn test_new_message(
    mut api_service: CryptomailApiServiceClient<Channel>,
    account: CreateAccountRequest,
    key_pair: Keypair,
    pre_key_secret: StaticSecret,
    _expected_message_id: MessageId,
) -> Result<()> {
    // Get inbox from server
    let mut req = GetThreadBoxesRequest {
        time_stamp: Utc::now().timestamp_nanos() as u64,
        public_key: account.public_key.clone(),
        thread_boxes: ThreadBoxType::Inbox as u32,
        signature: vec![],
    };
    req.sign(&key_pair).unwrap();

    let response = api_service
        .get_thread_boxes(req.clone())
        .await
        .unwrap()
        .into_inner();

    // Verify results
    assert_eq!(response.threads_boxes.len(), 1);
    let inbox = response.threads_boxes.get(0).unwrap();
    assert_eq!(inbox.thread_box_type, ThreadBoxType::Inbox as i32);
    assert_eq!(inbox.thread_ids.len(), 2);
    assert_eq!(response.threads.len(), 2);
    let thread = response.threads.get(0).unwrap();
    assert_eq!(thread.msgs_ids.len(), 1);

    // Decrypt message and verify its content - we assume it is receiver inbox...
    let message_thread_id = thread.msgs_ids.get(0).unwrap();
    let message_id = thread.get_message_id(message_thread_id.as_ref());
    let message = crate::Message::load_message(&message_id)
        .await
        .unwrap()
        .unwrap();
    let message_user_data = message.get_message_user_data().unwrap();
    let x_eph_pub_key = message_user_data.get_eph_pub_key()?;

    // recipient account pre key was used by sender to encrypt the message
    let shared_secret = pre_key_secret.diffie_hellman(&x_eph_pub_key);
    let content: MessageContent = MessageContent::decrypt(
        message_user_data.content.as_slice(),
        shared_secret.as_bytes(),
    )
    .unwrap();
    content.verify()?;

    // instead of happening in background - we query here for the deposit transaction
    // In the full server - this happens in the background and the deposit data is added to the message
    // tokio::time::delay_for(std::time::Duration::from_millis(5000)).await;
    let data = api_service
        .get_message_deposit_data(GetMessageDepositDataRequest {
            message_id: Some(message_id.clone()),
        })
        .await
        .unwrap()
        .into_inner();

    if let Some(_confirmation) = data.deposit_confirmation {
        info!("got deposit confirmation :-)")
    } else {
        info!("no deposit confirmation")
    }

    // todo: verify confirmation members match the expected ones!!!

    // Report client opening a message to the server
    let mut open_req = OpenMessageRequest {
        time_stamp: Utc::now().timestamp_nanos() as u64,
        public_key: account.public_key.clone(),
        message_id: Some(message_id.clone()),
        signature: vec![],
    };
    open_req.sign(&key_pair).unwrap();
    api_service
        .open_message(open_req)
        .await
        .unwrap()
        .into_inner();

    // create reply message and encrypt it using the pre-key

    let mut message_thread_id = [0u8; 8];
    OsRng.fill_bytes(&mut message_thread_id);

    let reply_message_id = MessageId {
        message_thread_id: message_thread_id.to_vec(),
        thread_id: thread.id.clone(),
    };

    // new ephemeral key
    let eph_secret = EphemeralSecret::new(OsRng);
    let x_eph_pub_key: x25519_dalek::PublicKey = x25519_dalek::PublicKey::from(&eph_secret);
    let eph_pub_key: PublicKey = x_eph_pub_key.into();

    // We get the recipient public pre key to use to encrypt message from his account info
    let recipient_pub_pre_key = account.public_account_info.unwrap().pre_key.unwrap();
    let recipient_x_pub_pre_key = recipient_pub_pre_key.get_x25519_pub_key()?;
    let shared_secret = eph_secret.diffie_hellman(&recipient_x_pub_pre_key);
    let simple_message = MessageContent::new_basic_message();
    let enc_message = simple_message.encrypt(shared_secret.as_bytes()).unwrap();
    let recipient_pub_key = message_user_data.get_author_public_key().unwrap();

    let reply_message = MessageUserdata {
        message_id: Some(reply_message_id.clone()),
        created: Utc::now().timestamp_nanos() as u64,
        payment: None,
        reply_to: message_id.message_thread_id.clone(),
        sender_public_key: account.public_key.clone(),
        recipient_public_key: Some(recipient_pub_key.clone()), // reply recipient is the message sender
        eph_pub_key: Some(eph_pub_key),
        recipient_pre_key_id: recipient_pub_pre_key.id,
        content: enc_message,
    };

    use prost::Message;
    let mut data = Vec::with_capacity(reply_message.encoded_len());
    reply_message.encode(&mut data)?;
    let message_user_data_signature = key_pair.sign(data.as_slice()).as_ref().to_vec();

    let mut reply_request = ReplyRequest {
        time_stamp: Utc::now().timestamp_nanos() as u64,
        public_key: account.public_key,
        message_user_data: data,
        message_user_data_signature,
        message_id: Some(reply_message_id),
        signature: vec![],
    };

    reply_request.sign(&key_pair).unwrap();
    api_service.reply(reply_request).await.unwrap().into_inner();

    Ok(())
}
