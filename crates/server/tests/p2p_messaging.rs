// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use base::snp::snp_server_api::provider_core_service_client::ProviderCoreServiceClient;
use base::snp::snp_server_api::{
    DrSessionHeader, GetIdentityBundleRequest, GetTermsOfServiceRequest, MessageRequest,
    MessageType, NewSessionRequest, TypedMessage,
};
use base::test_helpers::enable_logger;
use chrono::prelude::*;
use common::network_salt::NET_SALT;
use common::typed_msg_extensions::TypedMessageExtensions;
use crypto::utils::X25519PublicKeyWrapper;
use crypto::x2dh;
use crypto::x2dh::ProtocolInputAlice;
use double_ratchet::chain_key::ChainKey;
use double_ratchet::dr::DoubleRatchet;
use double_ratchet::session_key::SessionKey;
use log::*;
use prost::Message;
use rand_core::{OsRng, RngCore};
use server::server_service::{DestroyDb, ServerService, Startup, SNP_PROTOCOL_VERSION};
use std::time::Duration;
use tokio::time::sleep;
use xactor::Service;

use base::api_types_extensions::{Signed, SignedWithExternalVerifier};
use base::server_config_service::{
    ServerConfigService, GRPC_HOST_CONFIG_KEY, GRPC_SERVER_PORT_CONFIG_KEY,
};
use base::snp::snp_core_types::{EntityId, PublicKey};
use std::convert::TryFrom;

// new new session handling algorithm
///////////
// 1. Ensure sender used a valid provider id bundle and reject if not
// 2. Execute X2DH with caller using her ephemeral key
// 3. Init a dr session with sender using X2DH output
// 4. Decode the enc message in the dr session
// 5. Now we have sender public id - authenticate enc message with it
// 6. Authenticate whole top-level message with sender public id
// 7. Dispatch the message internally and get response
// 8. Store dr session by session id and by sender public id
// 9. Enc response in dr session and send it back to sender

// new message handling algo
////////////

// 1. Use unique dr session id to load session from storage and sender public id.
// 2. Update dr session with new dr public key
// 3. Decrypt internal message using the dr session
// 4. Authenticate internal message with sender public id and signature
// 5. Store dr session by session id and by sender public id
// 6. Dispatch the message internally and get response
// 7. Enc response in dr session and send it back to sender

/// In this test, a new client gets the server current bundle id via its public grpc service
/// an initiates a new session with a server and sends a message with the new session request.
/// The server processes the message and returns an encrypted response which the client authenticates an decrypts it
/// This is an end-to-end client-server communication flow
#[tokio::test]
async fn p2p_messaging() {
    enable_logger();

    debug!("starting grpc services...");

    // in this test Bob is the server and Alice is the client
    let server = ServerService::from_registry().await.unwrap();
    let _ = server.call(Startup {}).await.unwrap();

    let grpc_host = ServerConfigService::get(GRPC_HOST_CONFIG_KEY.into())
        .await
        .unwrap()
        .unwrap();
    let grpc_port = ServerConfigService::get_u64(GRPC_SERVER_PORT_CONFIG_KEY.into())
        .await
        .unwrap()
        .unwrap();

    sleep(Duration::from_millis(1000)).await; // Wait for the grpc service to start

    let mut client =
        ProviderCoreServiceClient::connect(format!("http://{}:{}", grpc_host, grpc_port))
            .await
            .expect("failed to connect to grpc server");

    // Step 1 - Alice gets Bob's current identity bundle via its public api and uses it to execute X2DH and dr with him....
    let bob_provider_bundle = client
        .get_identity_bundle(GetIdentityBundleRequest {
            protocol_version: SNP_PROTOCOL_VERSION.into(),
        })
        .await
        .unwrap()
        .into_inner()
        .bundle
        .unwrap();

    debug!("Bobs bundle id: {}", bob_provider_bundle.time_stamp);

    // Step 2 - create Alice identity and initiate a dr session with bob
    let alice_id_key_pair = ed25519_dalek::Keypair::generate(&mut rand_core::OsRng);
    let pair_bytes = alice_id_key_pair.to_bytes();
    // we need a clone to sign request to bob and one pair as input to dr algo
    let alice_id_key_pair_clone = ed25519_dalek::Keypair::from_bytes(pair_bytes.as_ref()).unwrap();
    let alice_id_pub_key = PublicKey {
        key: alice_id_key_pair.public.as_ref().to_vec(),
    };

    let ikb = bob_provider_bundle
        .get_provider_id_ed25519_public_key()
        .unwrap();

    let pkb = bob_provider_bundle.get_provider_x25519_pre_key().unwrap();

    // Alice X2DH protocol input
    let input_alice = ProtocolInputAlice {
        ikb,
        pkb,
        b_bundle_id: bob_provider_bundle.time_stamp,
    };

    // Alice executes x2dh with bob and get the output
    let output_alice = x2dh::execute_alice(&input_alice);

    // debug!("Alice X2DH output: {:?}", output_alice);

    // Alice creates a DR session with bob and using it to get the enc key for her first message with bob
    let input = SessionKey::from(NET_SALT.as_ref());
    let dr_root_chain_key = ChainKey::from(output_alice.shared_secret.as_ref());

    let mut alice_dr = DoubleRatchet::new_with_peer(
        input,
        dr_root_chain_key,
        &mut OsRng,
        &pkb,
        output_alice.ad.clone(),
    )
    .unwrap();

    // Alice sends her current public dr key with a first message (enc w first send message key) to bob
    let alice_pub_dr_key = alice_dr.get_public_key().unwrap();

    let alice_send_key = alice_dr.next_sending_key().unwrap();

    let inner_msg = GetTermsOfServiceRequest {
        promo_code: "".to_string(),
    };
    let mut buff = Vec::with_capacity(inner_msg.encoded_len());
    inner_msg.encode(&mut buff).unwrap();

    let alice_entity = EntityId {
        public_key: Some(alice_id_pub_key.clone()),
        nickname: "".to_string(),
    };

    let ikb_pub = bob_provider_bundle.get_provider_id_public_key().unwrap();
    let ikb_identity = EntityId {
        public_key: Some(ikb_pub.clone()),
        nickname: "".to_string(),
    };

    // this is the message alice sends to bob
    let mut typed_msg = TypedMessage {
        time_stamp: Utc::now().timestamp_nanos() as u64,
        msg_type: MessageType::ServiceTermsRequest as i32,
        message: buff,
        receiver: Some(ikb_identity.clone()),
        sender: Some(alice_entity.clone()),
        signature: None,
    };

    typed_msg.sign(&alice_id_key_pair_clone).unwrap();

    let enc_msg = TypedMessageExtensions::encrypt_msg(
        typed_msg.clone(),
        &alice_send_key.1,
        output_alice.ad.as_ref(),
    )
    .unwrap();

    let alice_dr_session_id = OsRng.next_u64();

    let message = base::snp::snp_server_api::Message {
        header: Some(DrSessionHeader {
            session_id: alice_dr_session_id,
            dr_pub_key: Some(PublicKey {
                key: alice_pub_dr_key.as_bytes().to_vec(),
            }),
            prev_count: 0,
            count: alice_send_key.0,
        }),
        enc_typed_msg: enc_msg.to_vec(),
    };

    let eka = PublicKey {
        // Alice ephemeral pub key for X2DH protocol
        key: output_alice.eka.as_bytes().to_vec(),
    };

    // Note that alice id is not exposed in this message clear-text !!!
    let mut new_session_request = NewSessionRequest {
        time_stamp: Utc::now().timestamp_nanos() as u64,
        receiver: Some(ikb_identity),
        sender_ephemeral_key: Some(eka),
        receiver_one_time_prekey_id: 0,
        message: Some(message),
        sender_signature: None,
        receiver_bundle_id: bob_provider_bundle.time_stamp,
        net_id: 0,
        protocol_version: SNP_PROTOCOL_VERSION.into(),
    };

    //debug!("new session request: {:?}", new_session_request);

    // Sign and add signature to the request
    new_session_request.sign(&alice_id_key_pair_clone).unwrap();

    let response = client
        .new_session(tonic::Request::new(new_session_request))
        .await
        .expect("failed to send NewSession request to service")
        .into_inner();

    // Alice decrypts the response message using the dr session with the server bob
    // Validate it is the expected response to the original request message (get service terms)...

    let message = response.message.unwrap();
    let resp_dr_header = message.header.unwrap();
    let key_data = resp_dr_header.dr_pub_key.unwrap();
    let bob_dr_key_wrapper = X25519PublicKeyWrapper::try_from(key_data.key.as_slice()).unwrap();
    assert!(alice_dr
        .ratchet(
            &mut OsRng,
            &bob_dr_key_wrapper.0.clone(),
            resp_dr_header.prev_count
        )
        .is_ok());
    let alice_receive_key = alice_dr.get_receiving_key(0).unwrap();

    let dec_res = TypedMessageExtensions::decrypt_msg(
        message.enc_typed_msg.as_slice(),
        &alice_receive_key,
        output_alice.ad.as_ref(),
    )
    .unwrap();

    // Alice checks that bob signed the typed message
    dec_res
        .verify_signature()
        .expect("failed to verify signature on response message");

    let sender = dec_res.sender.unwrap().public_key.unwrap();

    assert_eq!(ikb_pub.key, sender.key, "expected message from bob");

    // New sending key
    let alice_send_key_1 = alice_dr.next_sending_key().unwrap();

    // We create a new encrypted msg from the typed message we used for the previous message (get service terms)
    let enc_msg = TypedMessageExtensions::encrypt_msg(
        typed_msg,
        &alice_send_key_1.1,
        output_alice.ad.as_ref(),
    )
    .unwrap();

    let message1 = base::snp::snp_server_api::Message {
        header: Some(DrSessionHeader {
            session_id: alice_dr_session_id, // same session id as before
            dr_pub_key: Some(PublicKey {
                key: alice_dr.get_public_key().unwrap().as_bytes().to_vec(),
            }),
            prev_count: 0,
            count: alice_send_key_1.0,
        }),
        enc_typed_msg: enc_msg.to_vec(),
    };

    let response1 = client
        .message(tonic::Request::new(MessageRequest {
            message: Some(message1),
        }))
        .await
        .expect("failed to send message request to service")
        .into_inner();

    // Alice decrypts the response and processes it

    let message3 = response1.message.unwrap();
    let resp_dr_header = message3.header.unwrap();
    let key_data = resp_dr_header.dr_pub_key.unwrap();
    let bob_dr_key_wrapper = X25519PublicKeyWrapper::try_from(key_data.key.as_slice()).unwrap();
    assert!(alice_dr
        .ratchet(
            &mut OsRng,
            &bob_dr_key_wrapper.0.clone(),
            resp_dr_header.prev_count
        )
        .is_ok());
    let alice_receive_key = alice_dr.get_receiving_key(0).unwrap();

    let dec_res = TypedMessageExtensions::decrypt_msg(
        message3.enc_typed_msg.as_slice(),
        &alice_receive_key,
        output_alice.ad.as_ref(),
    )
    .unwrap();

    // Alice checks that bob signed the typed message
    dec_res
        .verify_signature()
        .expect("failed to verify signature on response message");

    let sender = dec_res.sender.unwrap().public_key.unwrap();

    assert_eq!(ikb_pub.key, sender.key, "expected message from bob");

    // delete the db created by the server for this test
    let _ = server.call(DestroyDb).await.unwrap();
}
