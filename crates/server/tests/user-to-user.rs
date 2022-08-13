// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;
extern crate nix;

mod child_guard;

use base::snp::snp_core_types::{ApiEndPoint, DialupInfo};
use base::snp::upsetter_server_admin::server_admin_service_client::ServerAdminServiceClient;
use base::snp::upsetter_simple_client::simple_client_user_service_client::SimpleClientUserServiceClient;
use base::snp::upsetter_simple_client::*;
use base::test_helpers::enable_logger;
use child_guard::ChildGuard;
use std::env;
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;

/*
In this test we show how user A sends a text message to user B using the network via service providers SA and SB
when SA is user's A service provider and SB is user's B service provider.
- We assume A and B didn't communicate directly over the network previously.
- We assume SA obtained X2DH(B, SB) - in practice this will involve execution of the modified KAD algorithm.
- We assume A obtained B's public id via a published bundle.

Steps

Setup phase
1. Start 4 processes - A, B, SA and B - Generate identities for all entities. A, B, SA and SB and Start SA and SB servers.
2. A and B get SA's and SB's service bundle via its public api.
2. B generates X2DH(B, SB) and sends it to SB. SB accepts B as provided client.
3. SA also have access to X2DH(B, SB) - shortcut to Kad algo execution
5. A asks SA for X2DH(B) and gets back X2DH(B, SB).

Execution phase
1. A creates M1: NewSession(B, UserSimpleMessage(A->B, msg-text)). TextMessage is higher-level protocol message.
2. A creates M2: ForwardMessage(SB, B, Payload(M1))
3. A creates M2: NewSession(SA, RoutMessage(SB, M2)) and sends it to SA.
4. SA sends M2 to SB (over a new session between them)
5. SB decrypts M2 (it doesn't include A's id only an ephemeral one)
6. SB forward payload to B (it is a NewSession message with UserSimpleMessage as inner message payload)

Notes
- SA only knows that A sends a message to SB but not that A is talking with B.
- SB doesn't know A's public identity only that there's a message to B forwarded via SA on behalf of another entity.

1. B decrypts M1 to read the message.
2. B creates R1 := UserSimpleMessage(B->A, msg-text) encrypts it in DR session to A.
3. B creates ForwardMessage(SA, A, R1) message and send it back via SB.
4. SB forwards to SA.
5. SA decrypt and forwards to A.

*/

#[tokio::test]

async fn user_to_user() {
    enable_logger();

    // todo: ALL config vars should come from the config file and not hard-coded.
    // todo: Find available tcp ports via server api, update the config files and use them.

    // step 1 - entities setup
    // note: we assume path is upsetter-core/crates/server. Looks like this is not the case when running the debugger.
    let path = env::current_dir().unwrap();
    info!("Path: {:?}", path);

    let bc_conf_file = path.join("tests/blockchain_service1.json");
    let bc_app_path = "../../target/debug/blockchain-app";
    let spa_app = Command::new(bc_app_path)
        .args(&["-c", bc_conf_file.to_str().unwrap()])
        .spawn()
        .unwrap();

    let bc_guard = ChildGuard(spa_app);

    let spa_conf_path = path.join("tests/spa_conf.json");
    let spb_conf_path = path.join("tests/spb_conf.json");
    let server_app_path = "../../target/debug/server-app";
    let spa_app = Command::new(server_app_path)
        .args(&["-c", spa_conf_path.to_str().unwrap()])
        .spawn()
        .unwrap();

    let spa_guard = ChildGuard(spa_app);
    let spb_app = Command::new(server_app_path)
        .args(&["-c", spb_conf_path.to_str().unwrap()])
        .spawn()
        .unwrap();

    let spb_guard = ChildGuard(spb_app);
    let a_conf_path = path.join("tests/client_a_conf.json");
    let b_conf_path = path.join("tests/client_b_conf.json");
    let client_app_path = "../../target/debug/client-app";
    let a_app = Command::new(client_app_path)
        .args(&["-c", a_conf_path.to_str().unwrap()])
        .spawn()
        .unwrap();
    let a_guard = ChildGuard(a_app);

    let b_app = Command::new(client_app_path)
        .args(&["-c", b_conf_path.to_str().unwrap()])
        .spawn()
        .unwrap();

    let b_guard = ChildGuard(b_app);

    // todo: figure out why this is needed - we get errors if we remove this....
    sleep(Duration::from_millis(3000)).await; // Wait for the grpc service to start

    let mut spa_admin_client = ServerAdminServiceClient::connect("http://[::1]:8082")
        .await
        .expect("failed to connect to spa admin service ");

    spa_admin_client
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

    let mut spb_admin_client = ServerAdminServiceClient::connect("http://[::1]:8083")
        .await
        .expect("failed to connect to spa admin service ");

    spb_admin_client
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

    info!("started all clients and providers...");

    // client a connects to spa and starts to get service from it
    let mut client_a = SimpleClientUserServiceClient::connect("http://[::1]:3033")
        .await
        .expect("failed to connect to client a");

    client_a
        .set_blockchain_service(SetBlockchainServiceRequest {
            dialup_info: Some(DialupInfo {
                end_point: 0,
                api_version: "0.1.0".to_string(),
                ip_address: "[::1]".to_string(),
                port: 5555,
                net_id: 0,
                name: "Blockchain Service".to_string(),
            }),
        })
        .await
        .unwrap();

    let res = client_a
        .user_set_provider(UserSetProviderRequest {
            dialup_info: Some(DialupInfo {
                end_point: 0,
                api_version: "0.1.0".into(),
                ip_address: "[::1]".into(),
                port: 8082,
                net_id: 0,
                name: "ServiceProviderA".to_string(),
            }),
        })
        .await
        .map_err(|e| error!("error connecting to client: {}", e))
        .unwrap();

    let client_a_provider_signed_bundle = res.into_inner().client_bundle.unwrap();

    info!("client a - provider set");

    // client b connects to spb and starts to get service from it
    let mut client_b = SimpleClientUserServiceClient::connect("http://[::1]:3034")
        .await
        .expect("failed to connect to client b");

    client_b
        .set_blockchain_service(SetBlockchainServiceRequest {
            dialup_info: Some(DialupInfo {
                end_point: 0,
                api_version: "0.1.0".to_string(),
                ip_address: "[::1]".to_string(),
                port: 5555,
                net_id: 0,
                name: "Blockchain Service".to_string(),
            }),
        })
        .await
        .unwrap();

    let res = client_b
        .user_set_provider(UserSetProviderRequest {
            dialup_info: Some(DialupInfo {
                end_point: 0,
                api_version: "0.1.0".into(),
                ip_address: "[::1]".into(),
                port: 8083,
                net_id: 0,
                name: "ServiceProviderB".to_string(),
            }),
        })
        .await
        .unwrap();

    info!("providers set on clients");

    let client_b_provider_signed_bundle = res.into_inner().client_bundle.unwrap();
    let client_a_entity = client_a_provider_signed_bundle.get_client_entity().unwrap();
    let client_b_entity = client_b_provider_signed_bundle.get_client_entity().unwrap();

    // Let the clients have each other's public bundles
    let _res = client_a
        .user_add_other_client_bundle(client_b_provider_signed_bundle)
        .await
        .unwrap();

    let _res = client_b
        .user_add_other_client_bundle(client_a_provider_signed_bundle)
        .await
        .unwrap();

    info!("testing a to b messaging...");

    // Tell client a to send a message to b - we send b's provider-signed bundle.
    // In the real-world we will obtain this from its provider via the search kad capabilities.
    match client_a
        .user_send_text_message(UserSendTextMessageRequest {
            other_client_id: Some(client_b_entity.clone()),
            user_text: "Hi Bob, this is Alice sending you an upsetter instant message".into(),
            reply_to: 0,
        })
        .await
    {
        Ok(resp) => {
            let message_id = resp.into_inner().message_id;

            debug!("a >> b done. Message id: {} ", message_id);

            // send a response to b from a
            match client_b
                .user_send_text_message(UserSendTextMessageRequest {
                    other_client_id: Some(client_a_entity.clone()),
                    user_text: "Hi Alice, this is Bob. Got your message!".into(),
                    reply_to: message_id,
                })
                .await
            {
                Ok(resp) => debug!("b >> a done. Message id: {}", resp.into_inner().message_id),
                Err(e) => error!("got error response: {}", e),
            }
        }
        Err(e) => error!("got error response: {}", e),
    }

    // test status updates

    info!("creating new status update channel for a...");
    // Create a new status update channel for A
    let resp = client_a
        .user_create_status_update_channel(UserCreateStatusUpdateChannelRequest {
            channel_name: "A Status Updates Channel".to_string(),
        })
        .await
        .unwrap()
        .into_inner();

    let channel_bundle = resp.channel_bundle.unwrap();
    let channel_id = channel_bundle.channel_id.as_ref().unwrap().clone();

    // subscribe B to A's status updates (on behalf of its user)

    client_b
        .user_subscribe_to_status_updates(UserSubscribeRequest {
            channel_bundle: Some(channel_bundle.clone()),
        })
        .await
        .unwrap()
        .into_inner();

    client_a
        .user_new_post(UserNewPostRequest {
            channel_id: Some(channel_id.clone()),
            reply_to: 0,
            text: "This is my first upsetter status update! -xxxx A".to_string(),
        })
        .await
        .unwrap()
        .into_inner();

    let resp = client_a
        .user_new_post(UserNewPostRequest {
            channel_id: Some(channel_id.clone()),
            reply_to: 0,
            text: "This is my 2nd upsetter status update! -xxxx A".to_string(),
        })
        .await
        .unwrap()
        .into_inner();

    // post a reply to this post by client_b....

    client_b
        .user_new_post(UserNewPostRequest {
            channel_id: Some(channel_id.clone()),
            reply_to: resp.post_id,
            text: "This is a reply to a status update by a subscriber!".to_string(),
        })
        .await
        .unwrap()
        .into_inner();

    client_b
        .user_unsubscribe_from_status_updates(UserUnsubscribeRequest {
            channel_bundle: Some(channel_bundle.clone()),
        })
        .await
        .unwrap()
        .into_inner();

    ////////////////////////////////////////////

    // groups

    let resp = client_a
        .user_create_group(UserCreateGroupRequest {
            group_name: "My Upsetter Group".to_string(),
        })
        .await
        .unwrap()
        .into_inner();

    let channel_bundle = resp.channel_bundle.unwrap();
    let channel_id = channel_bundle.channel_id.as_ref().unwrap().clone();

    client_b
        .user_join_group(UserJoinGroupRequest {
            channel_bundle: Some(channel_bundle.clone()),
        })
        .await
        .unwrap()
        .into_inner();

    let post_res = client_a
        .user_new_post(UserNewPostRequest {
            channel_id: Some(channel_id.clone()),
            reply_to: 0,
            text: "This is my first upsetter group message! -xxxx A".to_string(),
        })
        .await
        .unwrap()
        .into_inner();

    client_b
        .user_new_post(UserNewPostRequest {
            channel_id: Some(channel_id.clone()),
            reply_to: 0,
            text: "This is a post from group member B !!!!".to_string(),
        })
        .await
        .unwrap()
        .into_inner();

    client_b
        .user_new_post(UserNewPostRequest {
            channel_id: Some(channel_id.clone()),
            reply_to: post_res.post_id,
            text: "This is a reply from B on A grou post !!!!".to_string(),
        })
        .await
        .unwrap()
        .into_inner();

    client_b
        .user_leave_group(UserLeaveGroupRequest {
            channel_bundle: Some(channel_bundle),
        })
        .await
        .unwrap()
        .into_inner();

    // we need to keep a ref to the guards so they are not dropped before we get here in case there's no panic
    debug!("{}", spa_guard.0.id());
    debug!("{}", spb_guard.0.id());
    debug!("{}", a_guard.0.id());
    debug!("{}", b_guard.0.id());
    debug!("{}", bc_guard.0.id());
}
