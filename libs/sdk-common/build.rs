use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_path = PathBuf::from("src/grpc/proto/breez.proto");

    // Verify proto file exists
    if !proto_path.exists() {
        panic!("Proto file missing: {}", proto_path.display());
    }

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(
            &[proto_path.to_str().unwrap()],
            &["src/grpc/proto"] // Import directory
        )?;

    Ok(())
}