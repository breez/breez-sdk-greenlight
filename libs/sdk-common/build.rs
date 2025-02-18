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

    let target_arch =
        std::env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH not set");
    let build_transport = target_arch != "wasm32";

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .build_transport(build_transport)
        .compile(&[proto_path], &[proto_dir])?;
    Ok(())
}
