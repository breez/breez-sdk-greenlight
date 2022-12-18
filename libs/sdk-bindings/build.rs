use anyhow::*;

fn main() -> Result<()> {
    uniffi_build::generate_scaffolding("./src/breez_sdk.udl").unwrap();
    Ok(())
}
