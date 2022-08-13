// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};
use base::hex_utils::short_hex_string;
use bytes::Bytes;
use db::db_service;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use db::types::IntDbKey;
use double_ratchet::dr::DoubleRatchet;
use ed25519_dalek::PublicKey;
use xactor::*;

/// DRService is a system service for managing tow-party DR (double ratchet) sessions.
/// The service maintains persistent DR sessions with other peers and enables clients
/// to load these sessions from store and call the DR protocol on them.
/// See double_ratchet::DoubleRatchet for more info.
#[derive(Debug, Default)]
pub struct DRService {}
impl Service for DRService {}

// TODO: add functionality to store and retrieve the key-pairs for public pre-keys so entities can create new pre-keys and use them in new identity bundles.

impl DRService {
    /// Helper function to save a dr session
    pub async fn save_dr_session(remote_entity: PublicKey, dr: DoubleRatchet) -> Result<()> {
        let dr_service = DRService::from_registry()
            .await
            .map_err(|e| anyhow!(format!("failed to get provider service: {:?}", e)))?;

        dr_service
            .call(SaveSession {
                entity_id: remote_entity,
                dr,
            })
            .await
            .map_err(|e| anyhow!("internal error - failed to call SaveSession: {:?}", e))?
            .map_err(|e| anyhow!("internal error - error result: {:?}", e))?;

        Ok(())
    }

    /// Helper method to move boilerplate code from many places around the codebase to one canonical place
    pub async fn get_dr_session(
        entity_id: ed25519_dalek::PublicKey,
    ) -> Result<Option<DoubleRatchet>> {
        let dr_service = DRService::from_registry()
            .await
            .map_err(|e| anyhow!(format!("failed to get provider service: {:?}", e)))?;

        debug!(
            "looking for dr session with: {:?}",
            short_hex_string(entity_id.to_bytes().as_ref())
        );

        let res = dr_service
            .call(GetSession(entity_id))
            .await
            .map_err(|e| anyhow!(format!("internal error - failed to call: {:?}", e)))?
            .map_err(|e| anyhow!(format!("internal error - failed to call: {:?}", e)))?;

        if res.is_some() {
            debug!("found dr session");
        } else {
            debug!("no dr session");
        }

        Ok(res)
    }

    /// Helper method to move boilerplate code from many places around the codebase to one canonical place
    pub async fn get_dr_session_by_id(
        session_id: u64,
    ) -> Result<Option<(DoubleRatchet, PublicKey)>> {
        let dr_service = DRService::from_registry()
            .await
            .map_err(|e| anyhow!(format!("failed to get provider service: {:?}", e)))?;

        let res = dr_service
            .call(GetSessionById(session_id))
            .await
            .map_err(|e| anyhow!(format!("internal error - failed to call: {:?}", e)))?
            .map_err(|e| anyhow!(format!("internal error - failed to call: {:?}", e)))?;

        Ok(res)
    }
}

#[async_trait::async_trait]
impl Actor for DRService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("DRService started");
        Ok(())
    }
}

/// Get an existing DR session with an entity identified by an ed25519 public key
#[message(result = "Result<Option<DoubleRatchet>>")]
pub struct GetSession(pub PublicKey);

#[async_trait::async_trait]
impl Handler<GetSession> for DRService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetSession,
    ) -> Result<Option<DoubleRatchet>> {
        let read_item = ReadItem {
            key: bytes::Bytes::from(msg.0.to_bytes().to_vec()),
            cf: db_service::PROVIDER_COL_FAMILY,
        };

        match DatabaseService::read(read_item).await? {
            Some((data, _)) => {
                let dr: DoubleRatchet = bincode::deserialize(&data).unwrap();
                Ok(Some(dr))
            }
            _ => Ok(None),
        }
    }
}

/// Get a session's entity from a session id.
/// Use the entity returned to call GetSession
#[message(result = "Result<Option<(DoubleRatchet, PublicKey)>>")]
pub struct GetSessionById(pub u64);

#[async_trait::async_trait]
impl Handler<GetSessionById> for DRService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetSessionById,
    ) -> Result<Option<(DoubleRatchet, PublicKey)>> {
        // Get entity from the session id
        let key: IntDbKey = msg.0.into();
        debug!("Looking for dr session id : {}", msg.0);

        let read_item = ReadItem {
            key: key.0,
            cf: db_service::PROVIDER_COL_FAMILY,
        };

        let entity = DatabaseService::read(read_item).await?;

        if entity.is_none() {
            debug!("Did not find entity id by session id");
            return Ok(None);
        }

        let pub_id = PublicKey::from_bytes(entity.as_ref().unwrap().0.as_ref())
            .map_err(|e| anyhow!("invalid pub key data: {:?}", e))?;

        debug!(
            "Entity id for session id: {}",
            base::hex_utils::short_hex_string(pub_id.to_bytes().as_ref())
        );

        // Load the session for this entity
        let session_read_item = ReadItem {
            key: entity.unwrap().0,
            cf: db_service::PROVIDER_COL_FAMILY,
        };

        match DatabaseService::read(session_read_item).await? {
            Some((data, _)) => {
                let dr: DoubleRatchet = bincode::deserialize(&data).unwrap();
                debug!(
                    "Found stored dr session in db. Session id: {}",
                    dr.session_id
                );

                Ok(Some((dr, pub_id)))
            }
            _ => {
                debug!("No stored DR session for entity.");
                Ok(None)
            }
        }
    }
}

/// Save an existing DR session so it can be loaded later via GetSession
/// Caller should only call this after verifying that entity_id is the other party
/// of the dr session.
#[message(result = "Result<()>")]
pub struct SaveSession {
    pub entity_id: PublicKey, // session authenticated initiator (remote peer)
    pub dr: DoubleRatchet,    // dr session
}

#[async_trait::async_trait]
impl Handler<SaveSession> for DRService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SaveSession) -> Result<()> {
        // todo: these 2 db ops should be atomic - if 2nd fails, first one needs to be rolled back...

        // store mapping from entity id to the dr session
        let write_req = WriteItem {
            data: DataItem {
                key: Bytes::from(msg.entity_id.to_bytes().to_vec()),
                value: Bytes::from(bincode::serialize(&msg.dr).unwrap()),
            },
            cf: db_service::PROVIDER_COL_FAMILY,
            ttl: 0, // todo: think about ttl for dr sessions with other peers
        };

        DatabaseService::write(write_req).await?;

        debug!(
            "Stored dr session in the db for entity: {:?}",
            base::hex_utils::short_hex_string(msg.entity_id.as_ref())
        );

        let key: IntDbKey = msg.dr.session_id.into();

        // store mappings from session id to entity id
        let write_req = WriteItem {
            data: DataItem {
                key: key.0,
                value: Bytes::from(msg.entity_id.to_bytes().to_vec()),
            },
            cf: db_service::PROVIDER_COL_FAMILY,
            ttl: 0, // todo: think about ttl for dr sessions with other peers
        };

        DatabaseService::write(write_req).await?;

        debug!(
            "Stored latest session id for entity in db: {}",
            msg.dr.session_id
        );

        Ok(())
    }
}
