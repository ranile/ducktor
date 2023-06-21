#![doc = include_str!("../README.md")]

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Error, Field, Fields};

fn expand_from_js_value(input: DeriveInput) -> syn::Result<TokenStream> {
    let data = match input.data {
        Data::Struct(ref s) => s.clone(),
        Data::Enum(e) => {
            return Err(Error::new(
                e.enum_token.span,
                "cannot derive `FromJsValue` for enums",
            ))
        }
        Data::Union(u) => {
            return Err(Error::new(
                u.union_token.span,
                "cannot derive `FromJsValue` for unions",
            ))
        }
    };

    let fields = match data.fields {
        Fields::Named(f) => f,
        Fields::Unnamed(unnamed) => {
            return Err(Error::new(
                unnamed.span(),
                "cannot derive `FromJsValue` for tuple structs",
            ))
        }
        Fields::Unit => {
            return Err(Error::new_spanned(
                input.to_token_stream(),
                "cannot derive `FromJsValue` for unit structs",
            ))
        }
    };
    let struct_ident = input.ident;
    let imported = format_ident!("Imported{}", struct_ident);

    let mut tokens = TokenStream::new();
    for field in fields.named.iter() {
        let Field { ty, ident, .. } = &field;

        let ident = ident.as_ref().unwrap();

        tokens.extend(quote! {
            #[wasm_bindgen(method, getter)]
            fn #ident(this: &#imported) -> #ty;
        });
    }

    let extern_block = quote! {
        use ::wasm_bindgen::prelude::*;
        #[wasm_bindgen]
        extern "C" {
            type #imported;
            #tokens
        }
    };

    let fields = fields.named.iter().map(|field| {
        let Field { ident, .. } = field;

        let ident = ident.as_ref().unwrap();
        quote! {
            #ident: imported.#ident(),
        }
    });

    let output = quote! {
        impl ::core::convert::From<&::wasm_bindgen::JsValue> for #struct_ident {
            fn from(value: &::wasm_bindgen::JsValue) -> Self {
                #extern_block
                let imported: &#imported = ::wasm_bindgen::JsValue::unchecked_ref(value);
                #struct_ident {
                    #(#fields)*
                }
            }
        }
    };

    Ok(output)
}

#[proc_macro_derive(FromJsValue)]
pub fn from_js_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_from_js_value(input) {
        Ok(ts) => ts.into(),
        Err(err) => err.into_compile_error().into(),
    }
}
