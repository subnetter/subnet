// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;

mod abc_magic_command;
mod blockchain_service_commands;
mod channel_creator;
mod channel_subscriber;
mod channel_unsubscriber;
mod client_commands;
mod group_creator;
mod group_joiner;
mod group_leaver;
mod item_buyer;
mod items_lister;
mod message_sender;
mod paid_item_creator;
mod playground;
mod provider_commands;
mod set_client_blockchain_server;
mod set_provider;
mod set_provider_blockchain_server;
mod status_update_publisher;
mod usage;

/// A node is a wrapper over server which is designed
/// to be launched as a system stand-alone executable
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    playground::Playground::default().start().await
}
