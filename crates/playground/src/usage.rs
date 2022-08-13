// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::playground::Playground;
use std::env;

impl Playground {
    pub(crate) fn usage() {
        println!("SNP 0.1.0. Playground 0.1.0. Press ENTER to exit. Commands:");
        println!("    👉 usage");
        println!("    👉 exit");
        println!("    👉 quit");
        println!("    👉 abc-magic");
        println!("    👉 provider new <conf_file>");
        println!("    👉 client new <conf_file>");
        println!("    👉 <client> set-provider <provider>");
        println!("    👉 <client> message <client_name> <text>");
        println!("    👉 <client> message-reply <client> <reply_to> <text>");
        println!("    👉 <client> status-create <channel>");
        println!("    👉 <client> status-subscribe <channel>");
        println!("    👉 <client> status <channel> <text>");
        println!("    👉 <client> status-reply <channel> <reply_to> <text>");
        println!("    👉 <client> status-unsubscribe <channel>");
        println!("    👉 <client> group-create <group>");
        println!("    👉 <client> group-join <group>");
        println!("    👉 <client> group-message <group> <text>");
        println!("    👉 <client> group-message-reply <group> <reply_to> <text>");
        println!("    👉 <client> group-leave <group>");
        println!("    👉 <client> create-item <price> <name> <text>");
        println!("    👉 <client> buy-item <seller> <item-id> <price>");
        println!("    👉 <client> list-items <seller>");
        println!("    👉 bc-service list-clients");
        println!("    👉 bc-service list-providers");
        println!("    👉 bc-service add-client <client>");
        println!("    👉 bc-service add-provider <provider>");

        println!("Current dir: {}", env::current_dir().unwrap().display());
    }

    pub(crate) fn logo() {
        println!(
            "██    ██ ██████  ███████ ███████ ████████ ████████ ███████ ██████ 
██    ██ ██   ██ ██      ██         ██       ██    ██      ██   ██ 
██    ██ ██████  ███████ █████      ██       ██    █████   ██████  
██    ██ ██           ██ ██         ██       ██    ██      ██   ██ 
 ██████  ██      ███████ ███████    ██       ██    ███████ ██   ██ 
                                                                   "
        );
    }
}
