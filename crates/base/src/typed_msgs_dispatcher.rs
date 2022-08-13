// Copyright (c) 2021, Subnet Authors.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::snp::snp_server_api::{MessageType, TypedMessage};
use anyhow::{anyhow, Result};
use fnv::FnvHasher;
use log::*;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::marker::PhantomData;
use xactor::*;

// TypedMessageHandler is an actor message that includes an input TypedMessage
// and returns a TypedMessage
pub struct TypedMessageHandler(pub TypedMessage);

// Set the message result
impl Message for TypedMessageHandler {
    type Result = Result<TypedMessage>;
}

// Subscribe is an actor message which is used to subscribe a receiver
// for a TypedMessageHandler message
pub struct Subscribe {
    pub message_type: i32,
    pub subscriber: Caller<TypedMessageHandler>,
}

impl Message for Subscribe {
    type Result = Result<()>;
}

// Unsubscribe actor message - empty result
pub struct Unsubscribe {
    pub id: i32,
}

impl Message for Unsubscribe {
    type Result = Result<()>;
}

// TypeMessagesAdapter actor messages - sends a TypedMessage to the adapter
pub struct Publish(pub TypedMessage);

impl Message for Publish {
    type Result = Result<TypedMessage>;
}

/// TypedMessagesDispatcher is a TypedMessages broker that enables only one subscriber
/// to subscribe on a message type identified by an int enum.
/// When a client publishes a message, he gets a response from the subscriber that can be down-casted by the client to a specific prost::Message type.
pub struct TypedMessagesDispatcher {
    subscribes: HashMap<i32, Caller<TypedMessageHandler>, BuildHasherDefault<FnvHasher>>,
    mark: PhantomData<TypedMessage>,
}

impl Default for TypedMessagesDispatcher {
    fn default() -> Self {
        Self {
            subscribes: Default::default(),
            mark: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl Actor for TypedMessagesDispatcher {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("TypedMessagesDispatcher started");
        Ok(())
    }
}

impl Service for TypedMessagesDispatcher {}

#[async_trait::async_trait]
impl Handler<Subscribe> for TypedMessagesDispatcher {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: Subscribe) -> Result<()> {
        // todo: return error if there's already a subscriber for this message
        self.subscribes.insert(msg.message_type, msg.subscriber);
        debug!(
            "added subscriber to message type {}",
            MessageType::from_i32(msg.message_type).unwrap()
        );
        Ok(())
    }
}

/// Unsubscribe a message handler
#[async_trait::async_trait]
impl Handler<Unsubscribe> for TypedMessagesDispatcher {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: Unsubscribe) -> Result<()> {
        self.subscribes.remove(&msg.id);

        // todo: display warning if caller was not subscribed

        debug!("Subscriber unsubscribed from message type {:?}", msg.id);
        Ok(())
    }
}

/// Handle Publish actor message - search for the subscriber for this message,
/// Pass the message to the subscriber and return the result to the caller.
#[async_trait::async_trait]
impl Handler<Publish> for TypedMessagesDispatcher {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: Publish) -> Result<TypedMessage> {
        let subscriber = self
            .subscribes
            .get(&(msg.0.msg_type as i32))
            .ok_or_else(|| anyhow!("no subscriber to published message"))?;

        subscriber.call(TypedMessageHandler(msg.0)).await?
    }
}
