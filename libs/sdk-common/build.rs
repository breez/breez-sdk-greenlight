use std::{io, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    compile_protos("src/grpc/proto/breez.proto")?;
    Ok(())
}

pub fn compile_protos(proto: impl AsRef<Path>) -> io::Result<()> {
    let proto_path: &Path = proto.as_ref();
    let proto_dir = proto_path
        .parent()
        .expect("proto file should reside in a directory");
    let target_family =
        std::env::var("CARGO_CFG_TARGET_FAMILY").expect("CARGO_CFG_TARGET_FAMILY not set");
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS not set");
    let is_wasm = target_family == "wasm" && target_os == "unknown";

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .build_transport(!is_wasm)
        .compile(&[proto_path], &[proto_dir])?;
    Ok(())
}
