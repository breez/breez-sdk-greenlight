use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields, GenericArgument, Ident,
    PathArguments, Type, TypePath,
};

pub fn extern_wasm_bindgen(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let (serde_case, output_impl) = extern_wasm_bindgen_from(attr, &derive_input);
    let output = quote! {
        #[derive(serde::Deserialize, serde::Serialize, tsify_next::Tsify)]
        #[tsify(from_wasm_abi, into_wasm_abi)]
        #[serde(rename_all = #serde_case)]
        #derive_input
        #output_impl
    };
    output.into()
}

fn extern_wasm_bindgen_from(
    attr: proc_macro::TokenStream,
    derive_input: &DeriveInput,
) -> (&str, TokenStream) {
    let internal_name = &derive_input.ident;
    let external_name = TokenStream::from(attr);
    let mut output = TokenStream::new();
    let serde_case = match &derive_input.data {
        Data::Enum(data) => {
            output.extend(extern_wasm_bindgen_from_enum(
                internal_name,
                &external_name,
                data,
            ));
            "SCREAMING_SNAKE_CASE"
        }
        Data::Struct(data) => {
            output.extend(extern_wasm_bindgen_from_struct(
                internal_name,
                &external_name,
                data,
            ));
            "camelCase"
        }
        _ => "camelCase",
    };
    output.extend(quote! {
        impl wasm_bindgen::__rt::VectorIntoJsValue for #internal_name {
            fn vector_into_jsvalue(vector: Box<[#internal_name]>) -> wasm_bindgen::JsValue {
                wasm_bindgen::__rt::js_value_vector_into_jsvalue(vector)
            }
        }
    });
    (serde_case, output)
}

fn extern_wasm_bindgen_from_enum(
    internal_name: &Ident,
    external_name: &TokenStream,
    data_enum: &DataEnum,
) -> TokenStream {
    let variants = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let field_names = get_enum_field_names(&variant.fields);
        let fields = get_enum_fields(&variant.fields);
        if fields.is_empty() {
            quote! { #variant_name => Self::#variant_name }
        } else {
            quote! { #variant_name{#(#field_names),*} => Self::#variant_name{#(#fields),*} }
        }
    });
    let variants_clone = variants.clone();
    quote! {
        impl From<#external_name> for #internal_name {
            fn from(val: #external_name) -> Self {
                match val {
                    #(#external_name::#variants),*
                }
            }
        }
        impl From<#internal_name> for #external_name {
            fn from(val: #internal_name) -> Self {
                match val {
                    #(#internal_name::#variants_clone),*
                }
            }
        }
    }
}

fn extern_wasm_bindgen_from_struct(
    internal_name: &Ident,
    external_name: &TokenStream,
    data_stuct: &DataStruct,
) -> TokenStream {
    let fields = get_struct_fields(&data_stuct.fields);
    let fields_clone = fields.clone();
    quote! {
        impl From<#external_name> for #internal_name {
            fn from(val: #external_name) -> Self {
                #internal_name {
                    #(#fields),*
                }
            }
        }
        impl From<#internal_name> for #external_name {
            fn from(val: #internal_name) -> Self {
                #external_name {
                    #(#fields_clone),*
                }
            }
        }
    }
}

fn get_path(path_args: &PathArguments) -> Option<&TypePath> {
    if let PathArguments::AngleBracketed(bracketed_args) = path_args {
        for arg in bracketed_args.args.iter() {
            if let GenericArgument::Type(ty) = arg {
                if let Type::Path(path) = ty {
                    return Some(path);
                } else if let Type::Reference(reference) = ty {
                    if let Type::Path(ref path) = *reference.elem {
                        return Some(path);
                    }
                }
            }
        }
    }
    None
}

fn get_struct_fields(fields: &Fields) -> Vec<TokenStream> {
    fields
        .iter()
        .filter_map(|field| {
            let ty = &field.ty;
            field.ident.as_ref().map(|ident| match ty {
                Type::Path(type_path) => {
                    let segment = &type_path.path.segments[0];
                    if segment.ident == "Vec" {
                        quote! { #ident: val.#ident.into_iter().map(|i| i.into()).collect() }
                    } else if segment.ident == "Option" {
                        if get_path(&segment.arguments).is_some_and(|tp| tp.path.segments[0].ident == "Vec") {
                            quote! { #ident: val.#ident.map(|i| i.into_iter().map(|a| a.into()).collect()) }
                        } else {
                            quote! { #ident: val.#ident.map(|i| i.into()) }
                        }
                    } else {
                        quote! { #ident: val.#ident.into() }
                    }
                }
                _ => quote! { #ident: val.#ident.into() },
            })
        })
        .collect()
}

fn get_enum_fields(fields: &Fields) -> Vec<TokenStream> {
    fields
        .iter()
        .filter_map(|field| {
            let ty = &field.ty;
            field.ident.as_ref().map(|ident| match ty {
                Type::Path(type_path) => {
                    let segment = &type_path.path.segments[0];
                    if segment.ident == "Vec" {
                        quote! { #ident: #ident.into_iter().map(|i| i.into()).collect() }
                    } else if segment.ident == "Option" {
                        if get_path(&segment.arguments).is_some_and(|tp| tp.path.segments[0].ident == "Vec") {
                            quote! { #ident: #ident.map(|i| i.into_iter().map(|a| a.into()).collect()) }
                        } else {
                            quote! { #ident: #ident.map(|i| i.into()) }
                        }
                    } else {
                        quote! { #ident: #ident.into() }
                    }
                }
                _ => quote! { #ident: #ident.into() },
            })
        })
        .collect()
}

fn get_enum_field_names(fields: &Fields) -> Vec<TokenStream> {
    fields
        .iter()
        .filter_map(|field| field.ident.as_ref().map(|ident| quote! { #ident }))
        .collect()
}
