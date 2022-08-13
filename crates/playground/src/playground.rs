// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

extern crate base;
extern crate clap;
extern crate nix;
use self::base::snp::snp_blockchain::blockchain_service_client::BlockchainServiceClient;
use self::base::snp::snp_core_types::DialupInfo;
use self::base::snp::upsetter_server_admin::server_admin_service_client::ServerAdminServiceClient;
use anyhow::{anyhow, Result};
use base::snp::snp_core_types::{ChannelBundle, ProviderSignedClientIdentityBundle};
use base::snp::upsetter_simple_client::simple_client_user_service_client::SimpleClientUserServiceClient;
use clap::App;
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, Read, Write};
use std::process::Child;
use tonic::transport::Channel;

pub struct Playground {
    // String keys are friendly clients and providers names
    /// Interface to client user api keyed by client name
    pub(crate) clients: HashMap<String, SimpleClientUserServiceClient<Channel>>,
    /// Client for provider admin interface keyed by provider name
    pub(crate) providers_admin_clients: HashMap<String, ServerAdminServiceClient<Channel>>,
    /// Channel bundles by channel name
    pub(crate) channels: HashMap<String, ChannelBundle>,
    /// Client config keyed by client name
    pub(crate) clients_config: HashMap<String, Value>,
    /// Provider config keyed by provider name
    pub(crate) providers_config: HashMap<String, Value>,
    /// Processes guards - used to cleanup processes on main process exist
    pub(crate) proc_guards: Vec<ChildGuard>,
    /// todo: remove this and use name service
    pub(crate) clients_bundles: HashMap<String, ProviderSignedClientIdentityBundle>,
    /// Name service dialup info
    pub(crate) blockchain_service_info: Option<DialupInfo>,
    /// Name service grpc api client
    pub(crate) blockchain_server_client: Option<BlockchainServiceClient<Channel>>,
}

impl Playground {
    // Start playground
    pub async fn start(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let _ = App::new("Upsetter Playground")
            .version("0.1.0")
            .author("cmdev2. <cmdev2@protonmail.com")
            .about("Does awesome upsetting")
            .get_matches();
        let stdio = io::stdin();
        let mut input = stdio.lock();
        let out = io::stdout();
        let mut output = out.lock();

        Playground::logo();
        Playground::usage();

        println!("starting blockchain service...");

        // Start name server and store its info
        self.start_blockchain_server("blockchain_service1.json".into())
            .await?;

        loop {
            let _ = io::stdout().flush();
            let _ = output.write("👉 ".as_ref());
            let _ = io::stdout().flush();

            let mut line = String::new();
            input.read_line(&mut line)?;
            let iter = line.split_terminator(|c| c == '\n' || c == ' ' || c == '\r');
            let tokens: Vec<&str> = iter.collect();
            if tokens.is_empty() {
                break;
            }

            if tokens[0].is_empty() {
                println!("Exit? (y/n)");
                let mut line = String::new();
                input.read_line(&mut line)?;
                let iter = line.split_terminator(|c| c == '\n' || c == ' ' || c == '\r');
                let tokens: Vec<&str> = iter.collect();
                if tokens.is_empty() || tokens[0].to_lowercase() == "n" {
                    continue;
                } else {
                    break;
                }
            }

            let res = match tokens[0] {
                "exit" => break,
                "quit" => break,
                "usage" => {
                    Playground::usage();
                    Ok(())
                }
                "abc-magic" => self.exec_abc_magic_command().await,
                "provider" => self.exec_provider_cmd(tokens[1..].to_vec()).await,
                "client" => self.exec_client_cmd(tokens[1..].to_vec()).await,
                "bc-service" => self.exec_blockchain_service_cmd(tokens[1..].to_vec()).await,
                name => match tokens[1] {
                    "create-item" => {
                        if tokens.len() < 5 {
                            Err(anyhow!("missing arguments"))
                        } else {
                            let price = tokens[2].parse::<u64>()?;
                            let item_name = tokens[3];
                            self.create_paid_item(
                                name,
                                price,
                                item_name.into(),
                                tokens[4..].join(" "),
                            )
                            .await
                        }
                    }
                    "buy-item" => {
                        if tokens.len() < 5 {
                            Err(anyhow!("missing arguments"))
                        } else {
                            let item_id = tokens[3].parse::<u64>()?;
                            let price = tokens[4].parse::<u64>()?;
                            self.buy_item(name, tokens[2], item_id, price).await
                        }
                    }
                    "list-items" => {
                        if tokens.len() < 2 {
                            Err(anyhow!("missing arguments"))
                        } else {
                            self.list_paid_items(name, tokens[2]).await
                        }
                    }
                    "set-provider" => self.set_client_provider(name, tokens[2]).await,
                    "message" => {
                        if tokens.len() < 3 {
                            Err(anyhow!(
                                "missing arguments. Expected receiver name and text."
                            ))
                        } else {
                            self.send_message(name, tokens[2], tokens[3..].join(" "), 0)
                                .await
                        }
                    }
                    "message-reply" => {
                        if tokens.len() < 4 {
                            Err(anyhow!(
                                "missing arguments. Expected receiver name, original item id and text."
                            ))
                        } else if let Ok(reply_to) = tokens[3].parse::<u64>() {
                            self.send_message(name, tokens[2], tokens[3..].join(" "), reply_to)
                                .await
                        } else {
                            Err(anyhow!("invalid reply_to. expected a message id"))
                        }
                    }
                    "status-create" => self.create_channel(name, tokens[2]).await,
                    "status-subscribe" => self.channel_subscribe(name, tokens[2]).await,
                    "status-unsubscribe" => self.channel_unsubscribe(name, tokens[2]).await,
                    "status" => {
                        if tokens.len() < 3 {
                            Err(anyhow!(
                                "missing arguments. Expected channel name and text."
                            ))
                        } else {
                            self.status_update(name, tokens[2], tokens[3..].join(" "), 0)
                                .await
                        }
                    }
                    "status-reply" => {
                        if tokens.len() < 4 {
                            Err(anyhow!(
                                "missing arguments. Expected channel name, text and original post id"
                            ))
                        } else if let Ok(reply_to) = tokens[3].parse::<u64>() {
                            self.status_update(name, tokens[2], tokens[4..].join(" "), reply_to)
                                .await
                        } else {
                            Err(anyhow!("expected reply id"))
                        }
                    }
                    "group-create" => self.create_group(name, tokens[2]).await,
                    "group-join" => self.join_group(name, tokens[2]).await,
                    "group-leave" => self.leave_group(name, tokens[2]).await,
                    "group-message" => {
                        if tokens.len() < 3 {
                            Err(anyhow!("missing arguments. Expected group name and text."))
                        } else {
                            self.status_update(name, tokens[2], tokens[3..].join(" "), 0)
                                .await
                        }
                    }
                    "group-message-reply" => {
                        if tokens.len() < 4 {
                            Err(anyhow!(
                                "missing arguments. Expected group name, text and original post id"
                            ))
                        } else {
                            let reply_to = tokens[3].parse::<u64>()?;
                            self.status_update(name, tokens[2], tokens[4..].join(" "), reply_to)
                                .await
                        }
                    }
                    _ => Err(anyhow!("unrecognized command")),
                },
            };

            if res.is_err() {
                println!("💣 {}", res.err().unwrap())
            }
        }

        Ok(())
    }

    pub(crate) async fn read_json_from_file(&self, file_path: String) -> Result<Value> {
        let mut file = File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        // println!("{}", data);
        let json_data = serde_json::from_str(&*data).map_err(|_| anyhow!("bad data"))?;
        Ok(json_data)
    }
}

pub(crate) struct ChildGuard(pub(crate) Child);

/// A guard of a child os process, sends a ctrl-c SIGINT to child process when it is dropped.
impl Drop for ChildGuard {
    fn drop(&mut self) {
        let pid = self.0.id() as i32;
        // send ctrl-c to child process to let it gracefully shut down
        match signal::kill(Pid::from_raw(pid), Signal::SIGINT) {
            Err(e) => debug!("could not kill child process id {}: {}", pid, e),
            Ok(_) => debug!("killed child process id {}", pid),
        }
    }
}

impl Default for Playground {
    fn default() -> Self {
        Playground {
            clients: HashMap::new(),
            providers_admin_clients: HashMap::new(),
            channels: HashMap::new(),
            clients_config: HashMap::new(),
            providers_config: HashMap::new(),
            proc_guards: vec![],
            clients_bundles: HashMap::new(),
            blockchain_service_info: None,
            blockchain_server_client: None,
        }
    }
}
