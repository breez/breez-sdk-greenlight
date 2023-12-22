use breez_sdk_core::{
    mnemonic_to_seed as sdk_mnemonic_to_seed, parse as sdk_parse_input,
    parse_invoice as sdk_parse_invoice,
};
use js_sys::{Array, Promise};
use std::rc::Rc;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

pub type JSResult<T, E = JSError> = Result<T, E>;

#[wasm_bindgen(js_name = parseInput)]
pub fn parse_input(s: String) -> Promise {
    future_to_promise(async move {
        sdk_parse_input(&s)
            .await
            .map(|inputType| serde_wasm_bindgen::to_value(&inputType)?)
    })
}

#[wasm_bindgen(js_name = mnemonicToSeed)]
pub fn mnemonic_to_seed(phrase: String) -> JSResult<Array> {
    Ok(sdk_mnemonic_to_seed(phrase)?
        .map(JsValue::from)
        .map_err(|e| JsError::new(&e.to_string())))
}
