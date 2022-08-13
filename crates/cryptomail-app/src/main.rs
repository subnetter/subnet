// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

/// Start the server process. Note that this uses the multi-thread tokio runtime with default number of threads.
/// see: https://docs.rs/tokio/1.9.0/tokio/attr.main.html
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    cryptomail_app::start().await
}
