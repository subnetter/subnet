// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;
extern crate base;
extern crate db;

/// This crate includes components and services which are shared between client and servers
/// It uses both crypto and base crates for higher-level functionality and therefore just one base
/// create is insufficient
pub mod aead;
pub mod dr_service;
pub mod edh;
pub mod network_salt;
pub mod typed_msg_extensions;
pub mod wallet_service;
pub mod x2dh_service;
