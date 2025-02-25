mod async_trait;
mod testing;
mod wasm_bindgen;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn async_trait(args: TokenStream, input: TokenStream) -> TokenStream {
    async_trait::async_trait(args, input)
}

#[proc_macro_attribute]
pub fn async_test(args: TokenStream, input: TokenStream) -> TokenStream {
    testing::async_test(args, input)
}

#[proc_macro_attribute]
pub fn async_test_only_wasm(args: TokenStream, input: TokenStream) -> TokenStream {
    testing::async_test_only_wasm(args, input)
}

#[proc_macro_attribute]
pub fn async_test_all(args: TokenStream, input: TokenStream) -> TokenStream {
    testing::async_test_all(args, input)
}

#[proc_macro_attribute]
pub fn test(args: TokenStream, input: TokenStream) -> TokenStream {
    testing::test(args, input)
}

#[proc_macro_attribute]
pub fn test_all(args: TokenStream, input: TokenStream) -> TokenStream {
    testing::test_all(args, input)
}

/// Attribute macro to mirror the external struct/enum in WASM
///
/// ```rust
/// #[sdk_macros::extern_wasm_bindgen(sdk_common::prelude::RouteHint)]
/// pub struct RouteHint {
///     pub hops: Vec<RouteHintHop>,
/// }
/// ```
/// Generates in WASM typescript:
/// ```typescript
/// export interface RouteHint {
///     hops: RouteHintHop[];
/// }
/// ```
#[proc_macro_attribute]
pub fn extern_wasm_bindgen(args: TokenStream, input: TokenStream) -> TokenStream {
    wasm_bindgen::extern_wasm_bindgen(args, input)
}
