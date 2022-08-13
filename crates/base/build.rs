// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use std::path::Path;
use std::{fs, io};

// we allow this as we often comment out this method to cut time of local builds
#[allow(clippy::unnecessary_wraps)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    ////////////////////
    // note: uncomment when proto change to recompile
    // this is commented out to shorten build times when proto generated code didn't change
    ////////////////////

    // We add serde support for Protobuf structs that needs to be persisted locally as member of rust structs.

    /*
    std::env::set_var("OUT_DIR", "src");
    tonic_build::configure()
        .type_attribute(
            "EntityId",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        //.type_attribute("EntityId", "#[serde(default)]")
        .type_attribute(
            "PublicKey",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        //.type_attribute("PublicKey", "#[serde(default)]")
        .type_attribute(
            "ProviderNetInfo",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        //.type_attribute("ProviderNetInfo", "#[serde(default)]")
        .type_attribute(
            "DialupInfo",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        //.type_attribute("DialupInfo", "#[serde(default)]")
        .type_attribute(
            "Signature",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        //.type_attribute("Signature", "#[serde(default)]")
        .build_server(true)
        .out_dir("src/snp")
        .format(true)
        .compile(
            &[
                "proto/snp/core_types/types.proto",
                "proto/snp/core_types/identity_bundles.proto",
                "proto/snp/core_types/channels.proto",
                "proto/snp/blockchain/service.proto",
                "proto/snp/payments/types.proto",
                "proto/snp/payments/client_service.proto",
                "proto/snp/payments/wallet_types.proto",
                "proto/snp/server_api/provider_core_service_types.proto",
                "proto/snp/server_api/provider_core_service.proto",
                "proto/snp/server_api/client_service.proto",
                "proto/snp/server_api/public_service.proto",
                "proto/snp/client_to_client/channels.proto",
                "proto/snp/client_to_client/paid_items.proto",
                "proto/upsetter/simple_client/simple_client_service.proto",
                "proto/upsetter/server_admin/server_admin.proto",
            ],
            &["proto"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

    let src = Path::new("src/snp");
    rename_prost_generated_filenames(&src).unwrap();
    */
    Ok(())
}

// Ugly hack because prost output rust file names with `.` when packages are used, e.g. snp.foo, and the rust module system doesn't support . in file names.
fn _rename_prost_generated_filenames(dir: &Path) -> io::Result<()> {
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
}
