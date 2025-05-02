#[cfg(feature = "uniffi-25")]
extern crate uniffi_25 as uniffi;
#[cfg(feature = "uniffi-28")]
extern crate uniffi_28 as uniffi;
#[cfg(feature = "uniffi-25")]
extern crate uniffi_bindgen_25 as uniffi_bindgen;

#[cfg(feature = "uniffi-25")]
use camino::Utf8Path;

fn main() {
    uniffi::uniffi_bindgen_main();
    #[cfg(feature = "uniffi-25")]
    build_kmp()
}

#[cfg(feature = "uniffi-25")]
fn build_kmp() {
    let udl_file = "./src/breez_sdk.udl";
    let out_dir = Utf8Path::new("ffi/kmp");
    let config = Utf8Path::new("uniffi.toml");
    uniffi_bindgen::generate_external_bindings(
        uniffi_bindgen_kotlin_multiplatform::KotlinBindingGenerator {},
        udl_file,
        Some(config),
        Some(out_dir),
        None::<&Utf8Path>,
        None,
    )
    .unwrap();
}
