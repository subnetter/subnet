// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

extern crate log;
use base::test_helpers::enable_logger;
use bytes::Bytes;
use double_ratchet::chain_key::ChainKey;
use double_ratchet::dr::DoubleRatchet;
use double_ratchet::session_key::SessionKey;
use rand_core::{OsRng, RngCore};
use x25519_dalek::{PublicKey, StaticSecret};

#[test]
fn test_dr() {
    enable_logger();

    // Shared secret between Alice and Bob - output of execution of X2DH protocol between them
    let mut shared_secret = [0u8; 32];
    OsRng.fill_bytes(&mut shared_secret);
    let root_chain_key = ChainKey::from(shared_secret.as_ref());

    // Associated data known only to (output of DH key exchange protocol between parties such as DH)
    let mut ad_data = [0u8; 64];
    OsRng.fill_bytes(&mut ad_data);
    let ad = Bytes::from(ad_data.clone().to_vec());

    // Shared info between all nodes on network - salt
    let mut shared_info = [0u8; 32];
    OsRng.fill_bytes(&mut shared_info);
    let session_key = SessionKey::from(shared_info.as_ref());

    // Bob creates pkb - an x25519 prekey and publishes it in its bundle
    // Note that same pkb is used in both X2DH and dr (see notes on X2DH usage in dr protocol)
    // pkb will be bob's public ratchet key

    // bob's dr key pair is an x25519 key pair
    let bob_dr_private_key = StaticSecret::new(OsRng);
    let bob_dr_public_key: PublicKey = (&bob_dr_private_key).into();

    // Alice wants to send message to bob - she creates a dr ratchet using Bob's pkb
    // and the shared secret they established via X2DH

    let mut alice_dr = DoubleRatchet::new_with_peer(
        session_key,
        root_chain_key,
        &mut OsRng,
        &bob_dr_public_key,
        ad.clone(),
    )
    .unwrap();

    // Alice sends her current public dr key with a first message (enc w first send message key) to bob
    let alice_pub_dr_key = alice_dr.get_public_key().unwrap();

    // Bob inits his dr session w alice with the shared secret and his pkb private key
    let mut bob_dr = DoubleRatchet::new_with_keys(
        session_key,
        root_chain_key,
        bob_dr_private_key,
        ad,
        alice_dr.session_id,
    );

    // Bob performs a full ratchet step with Alice's shared public ratchet key
    bob_dr.ratchet(&mut OsRng, &alice_pub_dr_key, 0).unwrap();

    // both alice and bob should have the same message key for decrypting the first
    // message sent from alice to bob
    let alice_send_key = alice_dr.next_sending_key().unwrap();
    let bob_receive_key = bob_dr.get_receiving_key(alice_send_key.0).unwrap();

    assert_eq!(
        alice_send_key.1.as_bytes(),
        bob_receive_key.as_bytes(),
        "expected same message key"
    );

    // Bob now sends a message back to alice.
    // He computes a new sending key and gets his current public ratchet key
    // He also sends the sending chain count
    let bob_send_key = bob_dr.next_sending_key().unwrap();
    let bob_pub_ratchet_key = bob_dr.get_public_key().unwrap();

    // both of these are sent as header to a message encrypted with bob_send_key

    // Alice, gets the message and performs a ratchet step with bob_pub_ratchet_key.
    alice_dr
        .ratchet(&mut OsRng, &bob_pub_ratchet_key, 0)
        .unwrap();

    // Next, she calls next_receiving_key() to get bob's message enc key and decrypts the message from bob.
    let alice_receive_key = alice_dr.get_receiving_key(bob_send_key.0).unwrap();

    assert_eq!(
        bob_send_key.1.as_bytes(),
        alice_receive_key.as_bytes(),
        "expected same message key"
    );
}
