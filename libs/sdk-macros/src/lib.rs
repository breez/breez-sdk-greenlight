extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn async_trait(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut out = TokenStream::new();
    out.extend(quote! { #[cfg_attr(not(all(target_family = "wasm", target_os = "unknown")), async_trait::async_trait)] });
    out.extend(quote! { #[cfg_attr(all(target_family = "wasm", target_os = "unknown"), async_trait::async_trait(?Send))] });
    out.extend(TokenStream::from(input));
    out.into()
}

#[proc_macro_attribute]
pub fn async_test(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut out = TokenStream::new();
    out.extend(quote! { #[cfg_attr(not(all(target_family = "wasm", target_os = "unknown")), tokio::test)] });
    out.extend(TokenStream::from(input));
    out.into()
}

#[proc_macro_attribute]
pub fn async_test_only_wasm(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut out = TokenStream::new();
    out.extend(quote! { #[cfg_attr(all(target_family = "wasm", target_os = "unknown"), wasm_bindgen_test::wasm_bindgen_test)] });
    out.extend(TokenStream::from(input));
    out.into()
}

#[proc_macro_attribute]
pub fn async_test_all(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut out = TokenStream::new();
    out.extend(quote! { #[cfg_attr(not(all(target_family = "wasm", target_os = "unknown")), tokio::test)] });
    out.extend(quote! { #[cfg_attr(all(target_family = "wasm", target_os = "unknown"), wasm_bindgen_test::wasm_bindgen_test)] });
    out.extend(TokenStream::from(input));
    out.into()
}

#[proc_macro_attribute]
pub fn test(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut out = TokenStream::new();
    out.extend(
        quote! { #[cfg_attr(not(all(target_family = "wasm", target_os = "unknown")), test)] },
    );
    out.extend(TokenStream::from(input));
    out.into()
}

#[proc_macro_attribute]
pub fn test_all(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut out = TokenStream::new();
    out.extend(
        quote! { #[cfg_attr(not(all(target_family = "wasm", target_os = "unknown")), test)] },
    );
    out.extend(quote! { #[cfg_attr(all(target_family = "wasm", target_os = "unknown"), wasm_bindgen_test::wasm_bindgen_test)] });
    out.extend(TokenStream::from(input));
    out.into()
}

#[proc_macro_attribute]
pub fn tsify_wasm(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut out = TokenStream::new();
    out.extend(quote! { #[cfg_attr(all(target_family = "wasm", target_os = "unknown"), derive(tsify_next::Tsify))] });
    out.extend(TokenStream::from(input));
    out.into()
}
