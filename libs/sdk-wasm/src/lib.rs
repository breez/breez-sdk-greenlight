use breez_sdk_core::{InputType, LNInvoice, error::SdkResult};
use once_cell::sync::Lazy;

static RT: Lazy<tokio::runtime::Runtime> = Lazy::new(|| tokio::runtime::Runtime::new().unwrap());

pub fn parse_invoice(invoice: String) -> SdkResult<LNInvoice> {
    Ok(breez_sdk_core::parse_invoice(&invoice)?)
}

pub fn parse_input(s: String) -> SdkResult<InputType> {
    rt().block_on(async move { Ok(breez_sdk_core::parse(&s).await?) })
}

pub fn mnemonic_to_seed(phrase: String) -> SdkResult<Vec<u8>> {
    Ok(breez_sdk_core::mnemonic_to_seed(phrase)?)
}

fn rt() -> &'static tokio::runtime::Runtime {
    &RT
}
