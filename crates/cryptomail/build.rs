// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

// use std::path::Path;
// use std::{fs, io};

// we allow this as we often comment out this method to cut time of local builds
#[allow(clippy::unnecessary_wraps)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    ////////////////////
    // note: uncomment when proto changes to recompile
    // this is commented out to shorten build times when proto generated code didn't change
    ////////////////////

    // We add serde support for Protobuf structs that needs to be persisted locally as member of rust structs.

    /*
    std::env::set_var("OUT_DIR", "src");
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/model")
        .format(true)
        .type_attribute("types.MessageId", "#[derive(Hash, Eq)]")
        .compile(
            &[
                "proto/types/types.proto",
                "proto/types/accounts.proto",
                "proto/types/content.proto",
                "proto/api/api.proto",
                "proto/api/api_types.proto",
                "proto/api/admin_api.proto",
            ],
            &["proto"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
    */

    //let src = Path::new("src/model");
    //rename_prost_generated_filenames(&src).unwrap();

    // todo: uncomment to compile smart contract changes

    /*
    use ethers::prelude::Abigen;
    Abigen::new("CryptoMailCore", "resources/crypto_mail_abi.json")?
        .generate()?
        .write_to_file("src/model/crypto_mail_contract.rs")?;
    */
    Ok(())
}

/*
// Ugly hack because prost output rust file names with `.` when packages are used, e.g. snp.foo, and the rust module system doesn't support . in file names.
fn rename_prost_generated_filenames(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let file_stem_renamed = &path
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace(".", "_");

                fs::rename(&path, dir.join(format!("{}.rs", file_stem_renamed)))?;
            }
        }
    }

    Ok(())
}*/
