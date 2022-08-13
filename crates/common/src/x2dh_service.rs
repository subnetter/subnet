// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;
use crypto::x2dh;
use crypto::x2dh::{ProtocolInputAlice, ProtocolInputBob, ProtocolOutputAlice, ProtocolOutputBob};
use db::db_service;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use sha2::{Digest, Sha512};
use xactor::*;

/// X2DHService provides a system service for the X2DH protocol
/// Clients use this to can create a shared secret with another party using the X2DH protocol
/// and to get shared secrets already created by another party previously based on entity id.
pub struct X2DHService {}
impl Service for X2DHService {}

#[async_trait::async_trait]
impl Actor for X2DHService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("X2DHService started");
        Ok(())
    }
}

#[message(result = "Result<ProtocolOutputAlice>")]
pub struct ExecuteProtocolAsAlice(pub ProtocolInputAlice);

/// ExecuteProtocolAsAlice
#[async_trait::async_trait]
impl Handler<ExecuteProtocolAsAlice> for X2DHService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: ExecuteProtocolAsAlice,
    ) -> Result<ProtocolOutputAlice> {
        // step 1 - check if we already executed the protocol and return stored result
        let db_key = X2DHService::compute_alice_output_db_key(&msg.0);
        let read_item = ReadItem {
            key: db_key.clone(),
            cf: db_service::PROVIDER_COL_FAMILY,
        };

        let res: Option<(Bytes, u64)> = DatabaseService::read(read_item).await?;
        if res.is_some() {
            let res: ProtocolOutputAlice = bincode::deserialize(res.unwrap().0.as_ref()).unwrap();
            debug!("found bob and alice protocol output in db of alice");
            return Ok(res);
        }

        debug!("bob and alice protocol output not in db for alice - executing");

        // step 2 - execute the X2DH protocol
        let output = x2dh::execute_alice(&msg.0);

        // step 3 - store the output in the db
        let data: Bytes = Bytes::from(bincode::serialize(&output).unwrap());
        let write_item = WriteItem {
            data: DataItem {
                key: db_key,
                value: data,
            },
            cf: db_service::PROVIDER_COL_FAMILY,
            ttl: 0, // todo: set ttl for few weeks here
        };

        DatabaseService::write(write_item).await?;

        Ok(output)
    }
}

#[message(result = "Result<ProtocolOutputBob>")]
pub struct ExecuteProtocolAsBob(pub ProtocolInputBob);

/// ExecuteProtocolAsBob
#[async_trait::async_trait]
impl Handler<ExecuteProtocolAsBob> for X2DHService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: ExecuteProtocolAsBob,
    ) -> Result<ProtocolOutputBob> {
        // step 1 - return stored output if it exists in the db...
        let db_key = X2DHService::compute_bob_output_db_key(&msg.0);
        let read_item = ReadItem {
            key: db_key.clone(),
            cf: db_service::PROVIDER_COL_FAMILY,
        };

        let res: Option<(Bytes, u64)> = DatabaseService::read(read_item).await?;
        if res.is_some() {
            let res: ProtocolOutputBob = bincode::deserialize(res.unwrap().0.as_ref()).unwrap();
            debug!("found bob and alice protocol output in db of bob");
            return Ok(res);
        }

        debug!("No stored X2DH session output for parties - executing X2DH...");

        // step 2 - execute the x2dh protocol
        let output = x2dh::execute_bob(&msg.0);

        // step 3 - store the output in the db with ttl of 2 weeks or so
        let data: Bytes = Bytes::from(bincode::serialize(&output).unwrap());
        let write_item = WriteItem {
            data: DataItem {
                key: db_key,
                value: data,
            },
            cf: db_service::PROVIDER_COL_FAMILY,
            ttl: 0, // todo: set ttl for few weeks here
        };

        // step 4 - store the output in the db so it doesn't need be recomputed again and return it
        DatabaseService::write(write_item).await?;
        Ok(output)
    }
}

impl Default for X2DHService {
    fn default() -> Self {
        X2DHService {}
    }
}

impl X2DHService {
    /// helper method - given a client request to execute the protocol as Alice with Bob.
    /// compute a unique db key based on Alice's input. Used to return existing computed output for
    /// the unique provided input.
    fn compute_alice_output_db_key(input: &ProtocolInputAlice) -> Bytes {
        let mut hasher = Sha512::new();
        hasher.update(input.ikb.as_bytes().to_vec());
        hasher.update(input.pkb.as_bytes().to_vec());

        let mut buf = [0; 8];
        BigEndian::write_u64(&mut buf, input.b_bundle_id);
        hasher.update(buf);

        let res = hasher.finalize().to_vec();
        Bytes::from(res)
    }

    /// Helper method - compute a unique db key for Bob's protocol output based on his input
    fn compute_bob_output_db_key(input: &ProtocolInputBob) -> Bytes {
        let mut hasher = Sha512::new();
        hasher.update(input.eka.as_bytes().to_vec());
        hasher.update(input.ikb_pair.public.as_bytes().to_vec());
        hasher.update(input.pkb_private.to_bytes().to_vec());

        let mut buf = [0; 8];
        BigEndian::write_u64(&mut buf, input.b_bundle_id);
        hasher.update(buf);

        let res = hasher.finalize().to_vec();
        Bytes::from(res)
    }
}
