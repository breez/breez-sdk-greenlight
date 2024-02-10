pub mod error;

struct BreezSdk;

impl breez_sdk::BreezSdk for BreezSdk {
    fn mnemonic_to_seed(phrase: String) -> Result<Vec<u8>, breez_sdk::SdkError> {
        Ok(breez_sdk_core::mnemonic_to_seed(phrase)?)
    }
}

wai_bindgen_rust::export!("breez_sdk.wai");
