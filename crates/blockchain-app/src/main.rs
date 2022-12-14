// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    blockchain_app::start().await
}
