// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;
mod child_guard;

use base::api_types_extensions::{Signed, SignedWithExternalVerifier};
use base::server_config_service::{
    ServerConfigService, GRPC_HOST_CONFIG_KEY, GRPC_SERVER_PORT_CONFIG_KEY,
};
use base::snp::snp_blockchain::blockchain_service_client::BlockchainServiceClient;
use base::snp::snp_blockchain::SetBalanceRequest;

use base::snp::snp_core_types::{
    ApiEndPoint, ClientIdentityBundle, DialupInfo, EntityId, PreKey, PublicKey, Signature,
};
use base::snp::snp_payments::{Address, Amount, CoinType};
use base::snp::snp_server_api::provider_core_service_client::ProviderCoreServiceClient;
use base::snp::snp_server_api::{
    DrSessionHeader, GetIdentityBundleRequest, MessageType, NewSessionRequest, StartServiceRequest,
    TypedMessage,
};
use base::snp::upsetter_server_admin::server_admin_service_client::ServerAdminServiceClient;
use base::test_helpers::enable_logger;
use child_guard::ChildGuard;
use chrono::prelude::*;
use common::network_salt::NET_SALT;
use common::typed_msg_extensions::TypedMessageExtensions;
use crypto::utils::X25519PublicKeyWrapper;
use crypto::x2dh;
use crypto::x2dh::ProtocolInputAlice;
use double_ratchet::chain_key::ChainKey;
use double_ratchet::dr::DoubleRatchet;
use double_ratchet::session_key::SessionKey;
use ed25519_dalek::{Keypair, Signer};
use rand_core::{OsRng, RngCore};
use server::server_service::{DestroyDb, ServerService, Startup, SNP_PROTOCOL_VERSION};
use std::convert::TryFrom;
use std::env;
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;
use xactor::Service;

/// Server api test - get provider bundle public method test
#[tokio::test]
async fn serve_new_client() {
    enable_logger();

    let path = env::current_dir().unwrap();
    println!("Path: {:?}", path);

    let bc_conf_file = path.join("tests/blockchain_service1.json");
    let bc_app_path = "../../target/debug/blockchain-app";
    let spa_app = Command::new(bc_app_path)
        .args(&["-c", bc_conf_file.to_str().unwrap()])
        .spawn()
        .unwrap();

    let bc_guard = ChildGuard(spa_app);

    debug!("starting grpc services...");
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

    sleep(Duration::from_millis(2000)).await;

    debug!("Connecting...");

    // admin service client to configure blockchain service on the server

    let mut server_admin_client =
        ServerAdminServiceClient::connect(format!("http://{}:{}", grpc_host, grpc_port))
            .await
            .expect("failed to connect to provider admin service ");

    server_admin_client
        .set_blockchain_service(DialupInfo {
            end_point: ApiEndPoint::GrpcWeb2 as i32,
            api_version: "".to_string(),
            ip_address: "[::1]".to_string(),
            port: 5555,
            net_id: 0,
            name: "Blockchain Service".to_string(),
        })
        .await
        .expect("failed to set blockchain service");

    // client is alice. provider is bob

    let mut bc_client = BlockchainServiceClient::connect("http://[::1]:5555")
        .await
        .expect("failed to connect to grpc ping service");

    // generate 2 accounts key pairs here
    let alice_keypair = Keypair::generate(&mut rand_core::OsRng);

    let alice_payment_address = Address {
        data: alice_keypair.public.to_bytes()[12..].to_vec(),
    };

    bc_client
        .set_balance(SetBalanceRequest {
            address: Some(alice_payment_address.clone()),
            amount: Some(Amount {
                value: 1000,
                coin_type: CoinType::Core as i32,
            }),
        })
        .await
        .unwrap();

    let mut alice_client =
        ProviderCoreServiceClient::connect(format!("http://{}:{}", grpc_host, grpc_port))
            .await
            .expect("failed to connect to grpc provider core service");

    // Get server's provider current bundle
    let bob_provider_bundle = alice_client
        .get_identity_bundle(GetIdentityBundleRequest {
            protocol_version: SNP_PROTOCOL_VERSION.into(),
        })
        .await
        .unwrap()
        .into_inner()
        .bundle
        .unwrap();

    let alice_id_key_pair = ed25519_dalek::Keypair::generate(&mut rand_core::OsRng);
    let alice_id_pub_key = PublicKey {
        key: alice_id_key_pair.public.as_ref().to_vec(),
    };

    let alice_pre_key_private = x25519_dalek::StaticSecret::new(&mut rand_core::OsRng);
    let alice_pre_key_pub_data: x25519_dalek::PublicKey = (&alice_pre_key_private).into();
    let pre_key_public: PublicKey = PublicKey {
        key: alice_pre_key_pub_data.to_bytes().to_vec(),
    };
    let alice_entity = EntityId {
        public_key: Some(alice_id_pub_key.clone()),
        nickname: "".to_string(),
    };

    let time_stamp = Utc::now().timestamp_nanos() as u64;

    let mut alice_bundle = ClientIdentityBundle {
        time_stamp,
        client_id: Some(alice_entity.clone()),
        address: Some(alice_payment_address.clone()),
        provider_bundle: Some(bob_provider_bundle.clone()),
        pre_key: Some(PreKey {
            x2dh_version: "".to_string(),
            key: Some(pre_key_public),
            key_id: 0,
        }),
        one_time_keys: vec![],
        profile_image: None,
        signature: None,
        net_id: 0,
    };

    use prost::Message;

    // Sign the client bundle and add signature to the request
    let mut buf = Vec::with_capacity(alice_bundle.encoded_len());
    alice_bundle.encode(&mut buf).unwrap();
    alice_bundle.signature = Some(Signature {
        scheme_id: 0,
        signature: alice_id_key_pair.sign(&buf).as_ref().to_vec(),
    });

    let start_service_request = StartServiceRequest {
        bundle: Some(alice_bundle),
        payment: None,
        service_contract_id: 0,
        contract_options: 0,
    };

    // start session we provider and set StartServiceRequest as the message

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

    debug!("Alice X2DH output: {:?}", output_alice);

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
    let ikb_pub = bob_provider_bundle.get_provider_id_public_key().unwrap();
    let ikb_identity = EntityId {
        public_key: Some(ikb_pub.clone()),
        nickname: "".to_string(),
    };

    let mut buff = Vec::with_capacity(start_service_request.encoded_len());
    start_service_request.encode(&mut buff).unwrap();

    // this is the message alice sends to bob (a start service request)
    let mut typed_msg = TypedMessage {
        time_stamp: Utc::now().timestamp_nanos() as u64,
        msg_type: MessageType::StartServiceRequest as i32,
        message: buff,
        receiver: Some(ikb_identity.clone()),
        sender: Some(alice_entity.clone()),
        signature: None,
    };

    // Sign and add signature to typed_msg as alice
    typed_msg.sign(&alice_id_key_pair).unwrap();

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

    new_session_request.sign(&alice_id_key_pair).unwrap();

    //debug!("new session request: {:?}", new_session_request);

    let response = alice_client
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

    // delete the db created by the server for this test
    let _ = server.call(DestroyDb).await.unwrap();

    debug!("{}", bc_guard.0.id());
}
