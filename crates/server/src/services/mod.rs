//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

//! Module net_api handles all incoming api requests from the network.

mod admin_service;
mod blockchain_service;
mod clients_service;
mod messaging;
mod provider_id;
mod provider_id_service;
mod public_service;
mod terms_service;

pub mod server_service;

pub(crate) mod server_to_server;
