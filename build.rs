/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src").join("generated");

    generate(&out_dir)?;

    Ok(())
}

/// Generates protobuf bindings into src/gen and fails if the generated files do
/// not match those that are already checked into git.
fn generate(out_dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let configure = tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_server(true)
        .build_client(true)
        .out_dir(out_dir);

    configure
        .clone()
        .file_descriptor_set_path(out_dir.join("grpc_sysinfo_v1.bin"))
        .compile(&["proto/sysinfo.proto"], &["proto"])?;

    println!("cargo:rerun-if-changed=proto/sysinfo.proto");

    Ok(())
}
