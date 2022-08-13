// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::{anyhow, bail, Result};
use base::hex_utils::short_hex_string;
use base::snp::snp_core_types::{EntityId, PublicKey};
use base::snp::snp_server_api::dr_message::Data;
use base::snp::snp_server_api::{
    DrMessage, ForwardMessagePayload, ForwardMessageRequest, MessageType, RouteMessageRequest,
    TypedMessage,
};
use bytes::Bytes;
use common::aead::AEAD;
use common::dr_service::DRService;
use rand_core::OsRng;

impl SimpleClient {
    /// Send a typed message to another client
    pub(crate) async fn send_typed_message(
        &mut self,
        msg: TypedMessage,
        receiver_id: Bytes,
    ) -> Result<()> {
        // In this flow, we are A, SA is our service provider. We send a message to B where SB is its service provider.
        // Note that we assume to B is not server by SA. In the case it does, we need to execute a different simpler flow
        // where we simply send SA the text-message in a NewSessionRequest (or in a Message) to B.

        if self.provider_bundle.is_none() {
            bail!("missing provider bundle")
        }

        let sb_bundle = self
            .other_clients
            .get(receiver_id.as_ref())
            .ok_or_else(|| anyhow!("I don't know about this client"))?
            .clone();

        let b_bundle = sb_bundle.client_bundle.as_ref().unwrap();
        let b_pub_key = b_bundle.get_client_id_public_key().unwrap();
        let b_entity = EntityId {
            public_key: Some(b_pub_key.clone()),
            nickname: "Bob".to_string(),
        };

        // Check if we have a DR session with B. If we do - message to it should be Message otherwise it is NewSession
        let ikb = b_bundle.get_client_id_ed25519_public_key()?;

        // create NewSessionRequest or Message based on existence of DR message with receiver
        let data = match DRService::get_dr_session(ikb).await? {
            Some(_) => {
                debug!("existing dr session with receiver client - using it");
                Data::Message(self.create_message_to_receiver(ikb, msg).await?)
            }
            None => {
                debug!("no existing dr session with receiver client - starting a new one...");
                let new_session_request =
                    self.new_session_message_to_client(&b_bundle, msg).await?;

                Data::NewSessionRequest(new_session_request)
            }
        };

        // The forward request payload we need to send to SB (via SA)
        let forward_message_payload = ForwardMessagePayload {
            receiver: Some(b_entity),
            dr_message: Some(DrMessage { data: Some(data) }),
        };

        // now we perform an EDH with SB. We use its published pre-key and a new ephemeral key we generate here
        // the results key is used to encrypt payload so only SB can decrypt it

        let spb_bundle = b_bundle.provider_bundle.as_ref().unwrap();
        let spb_pre_key = spb_bundle.get_provider_pre_key()?.as_x25519_pub_key()?;
        let eph_key = x25519_dalek::EphemeralSecret::new(&mut OsRng);
        let eph_pub = x25519_dalek::PublicKey::from(&eph_key);
        let shared_secret = eph_key.diffie_hellman(&spb_pre_key);
        let ad = common::edh::compute_ad(&eph_pub, &spb_pre_key);

        use prost::Message;
        let mut buf = Vec::with_capacity(forward_message_payload.encoded_len());
        forward_message_payload.encode(&mut buf)?;

        debug!(
            "%%%% spb pub pre key: {}",
            short_hex_string(spb_pre_key.to_bytes().as_ref())
        );
        debug!("%%%% ad: {}", short_hex_string(ad.as_ref()));
        debug!(
            "%%%% shared secret: {}",
            short_hex_string(shared_secret.as_bytes().to_vec().as_ref())
        );

        let enc_payload = AEAD::encrypt(bytes::Bytes::from(buf), &shared_secret.as_bytes(), &ad)?;

        // Message to SB
        let forward_req = ForwardMessageRequest {
            receiver: Some(spb_bundle.provider_id.clone().unwrap()), // SB id - router sees that and can route
            receiver_bundle_id: spb_bundle.time_stamp,
            sender_ephemeral_key: Some(PublicKey {
                key: eph_pub.as_bytes().to_vec(),
            }),
            enc_payload: enc_payload.to_vec(), // encrypted ForwardMessagePayload object (to SB)
        };

        // Message to SA
        // M2:= RouteMessage(SA, ForwardMessageRequest)
        // Send M2 to SA (in current dr session we already have with it)
        // include SB dialup info for now in the request.
        // todo: SA can find SB dialup info by running Kad with SB's public id. Do this once Kad is implemented.

        if spb_bundle.dial_up_info.is_empty() {
            bail!("missing dialup info from bundle")
        }

        let route_req = RouteMessageRequest {
            forward_message: Some(forward_req),
            dialup_info: Some(spb_bundle.dial_up_info[0].clone()),
        };

        let mut buff: Vec<u8> = Vec::with_capacity(route_req.encoded_len());
        route_req.encode(&mut buff)?;

        //  The flow continues on SA and SB after we send the message to SA below:
        //  step 1. SA creates M3: NewSession(SB, M2)
        //  step 2. SA sends M3 to SB (over a new session between them)
        //  step 3. SB decrypts M2 (it doesn't include A's id only an ephemeral one)
        //  step 4. SB forward payload to B (it is a NewSession or a Message)

        debug!("sending route message request to our provider...");

        let _resp_msg = self
            .send_message_to_provider(MessageType::RouteMessageRequest, buff)
            .await?;

        debug!("got RouteMessageRequest response from provider :-)");

        Ok(())
    }
}
