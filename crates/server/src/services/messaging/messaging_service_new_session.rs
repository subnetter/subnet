//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::messaging::messaging_service::ServerMessagingService;
use crate::services::messaging::messaging_service_new_msg::IncomingMessageContext;
use anyhow::Result;
use base::api_types_extensions::SignedWithExternalVerifier;
use base::hex_utils::short_hex_string;
use base::snp::snp_server_api::{NewSessionRequest, NewSessionResponse};
use bytes::Bytes;
use common::network_salt;
use common::typed_msg_extensions::TypedMessageExtensions;
use common::x2dh_service::{ExecuteProtocolAsBob, X2DHService};
use crypto::utils::StaticSecretWrapper;
use crypto::x2dh::{ProtocolInputBob, ProtocolOutputBob};
use double_ratchet::chain_key::ChainKey;
use double_ratchet::dr::DoubleRatchet;
use double_ratchet::session_key::SessionKey;
use rand_core::OsRng;
use std::convert::TryInto;
use tonic::{Request, Response, Status};
use xactor::Service;

/// MyMessagingService new_session api method implementation
impl ServerMessagingService {
    /// Handles a remote request for a new DR session with this provider
    pub async fn new_session_handler(
        &self,
        request: Request<NewSessionRequest>,
    ) -> Result<Response<NewSessionResponse>, Status> {
        //
        // Step 1 - Check that the sender used a valid receiver provider data (provider id, pre-key)
        let req_data = request.into_inner();
        let bundle =
            ServerMessagingService::get_provider_id_bundle(req_data.receiver_bundle_id).await?;

        debug!(
            "Using bundle id: {}",
            bundle.public_bundle.as_ref().unwrap().time_stamp
        );

        let provider_id_private_key = bundle
            .get_provider_private_key()
            .map_err(|_| Status::internal("internal error"))?;
        let provider_id_pub_key: ed25519_dalek::PublicKey = (&provider_id_private_key).into();
        let receiver_pub_key = req_data
            .get_receiver()
            .map_err(|_| Status::invalid_argument("missing receiver id"))?;
        if receiver_pub_key.as_bytes() != provider_id_pub_key.as_bytes() {
            // Receiver id in the request doesn't match our own provider id
            return Err(Status::not_found("unrecognized receiver id"));
        }

        // step 2 - create X2DH ProtocolInputBob - this provider is Bob. Caller is Alice.
        // Prepare data needed for X2DH protocol execution.
        let eka = req_data
            .get_eka()
            .map_err(|_| Status::invalid_argument("invalid eka"))?;

        debug!(
            "eka public: {}",
            short_hex_string(eka.as_bytes().to_vec().as_ref())
        );

        let ikb_pair_source = bundle.provider_id_keypair.as_ref().unwrap();
        let ikb_pair = ikb_pair_source.to_ed2559_kaypair();
        let ikb_pair_clone = ikb_pair_source.to_ed2559_kaypair();

        let pkb_private_wrapped: Result<StaticSecretWrapper> =
            bundle.pre_key.as_ref().unwrap().key.as_slice().try_into();
        if pkb_private_wrapped.is_err() {
            return Err(Status::invalid_argument("invalid prekey data in bundle"));
        }

        let pkb_private = pkb_private_wrapped.unwrap().0;
        debug!(
            "pkb private: {}",
            short_hex_string(pkb_private.to_bytes().as_ref())
        );
        debug!("ikb public: {}", short_hex_string(ikb_pair.public.as_ref()));
        debug!(
            "Receiver bundle id from request: {}",
            req_data.receiver_bundle_id
        );

        // debug!("Bundle from local: {:?}", bundle.public_bundle);

        let input_bob = ProtocolInputBob {
            eka,
            ikb_pair,
            pkb_private: pkb_private.clone(),
            b_bundle_id: req_data.receiver_bundle_id,
        };

        // step 3 - call X2DHService to execute x2dh with Alice (obtain AD and shared secret)
        let x2dh_output_bob: ProtocolOutputBob = X2DHService::from_registry()
            .await
            .map_err(|_| Status::internal("failed to use X2DH service"))?
            .call(ExecuteProtocolAsBob(input_bob))
            .await
            .map_err(|_| Status::internal("failed to execute X2DH protocol"))?
            .map_err(|_| Status::internal("failed to execute X2DH protocol"))?;

        /*
        debug!(
            ">>> x2dh output shared_secret: {:?}",
            x2dh_output_bob.shared_secret.as_ref()
        );
        debug!(
            ">>> x2dh output ad: {:?}",
            x2dh_output_bob.ad.as_ref().to_vec()
        );*/

        // step 4 - Start a new DR ratchet with Alice using (shared secret, AD) from X2DH output

        // Shared secret between Alice (other party) and Bob (receiver, this provider) - output of execution of X2DH protocol between them
        let root_chain_key = ChainKey::from(x2dh_output_bob.shared_secret);

        // Shared info between all nodes on the same p2p network - salt
        let session_key = SessionKey::from(network_salt::NET_SALT.to_vec().as_slice());

        let session_id = req_data
            .get_dr_session_id()
            .map_err(|e| Status::invalid_argument(format!("missing session id: {:?}", e)))?;

        // Bob inits his dr session w Alice with the shared secret and his pkb private key
        let mut dr = DoubleRatchet::new_with_keys(
            session_key,
            root_chain_key,
            pkb_private,
            Bytes::from(x2dh_output_bob.ad.as_ref().to_vec()),
            session_id,
        );

        // decrypt TypedMessage using the dr session (same logic below for incoming msg in dr session)
        let alice_pub_dr_key = req_data
            .get_sender_dr_pub_key()
            .map_err(|_| Status::invalid_argument("invalid sender dr pub key"))?;

        // Bob performs a full ratchet step with Alice's pub dr key per the protocol
        dr.ratchet(&mut OsRng, &alice_pub_dr_key, 0)
            .map_err(|_| Status::invalid_argument("invalid dr data"))?;

        // The requested message decryption key (compare counter with message)
        let bob_receive_key = dr.get_receiving_key(0).unwrap();

        // debug!(">>> bob receiver key: {:?}", bob_receive_key.1.as_bytes());

        // The key to encrypt response message to Alice
        // let bob_send_key = dr.next_sending_key().unwrap();

        // Bob's current public dr key - to be sent in the message header to alice
        //let bob_pub_ratchet_key = dr.get_public_key().unwrap();

        // debug!(">>> bob ratchet key: {:?}", bob_pub_ratchet_key.as_bytes());

        // decrypt and authenticate the inner message
        let typed_message = TypedMessageExtensions::decrypt_msg(
            req_data.message.as_ref().unwrap().enc_typed_msg.as_slice(),
            &bob_receive_key,
            x2dh_output_bob.ad.as_ref(),
        )
        .map_err(|e| Status::invalid_argument(format!("invalid enc message: {:?}", e)))?;

        // get ika from message and authenticate the whole request using it and signature
        let ika = typed_message
            .get_ika()
            .map_err(|_| Status::invalid_argument("invalid sender key"))?;

        debug!("Called public id: {}", short_hex_string(ika.as_ref()));

        // verify the whole request
        req_data
            .verify_signature(&ika)
            .map_err(|_| Status::invalid_argument("Failed to authenticate message"))?;

        // step 5 - process message
        // this step is common with how we handle message in an exiting dr session
        // so we use a helper function to generate the response
        let context = IncomingMessageContext {
            dr,
            msg: typed_message,
            ikb_pair: ikb_pair_clone,
            ika,
        };

        // standard incoming message processing - returns a response Message
        let resp_msg = ServerMessagingService::process_incoming_msg(context)
            .await
            .map_err(|e| Status::internal(format!("failed to process message: {:?}", e)))?;

        Ok(Response::new(NewSessionResponse {
            message: Some(resp_msg),
        }))
    }
}
