#![doc = include_str!("../README.md")]

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Error, Field, Fields};
use syn::parse::{Parse, ParseStream};

struct RustType {
    ident: Ident,
    fields: Vec<Field>,
    imported: Ident,
}

impl Parse for RustType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input: DeriveInput = input.parse()?;

        let data = match input.data {
            Data::Struct(ref s) => s.clone(),
            Data::Enum(e) => {
                return Err(Error::new(
                    e.enum_token.span,
                    "cannot derive `FromJsValue` for enums",
                ));
            }
            Data::Union(u) => {
                return Err(Error::new(
                    u.union_token.span,
                    "cannot derive `FromJsValue` for unions",
                ));
            }
        };

        let fields = match data.fields {
            Fields::Named(f) => f,
            Fields::Unnamed(unnamed) => {
                return Err(Error::new(
                    unnamed.span(),
                    "cannot derive `FromJsValue` for tuple structs",
                ));
            }
            Fields::Unit => {
                return Err(Error::new_spanned(
                    input.to_token_stream(),
                    "cannot derive `FromJsValue` for unit structs",
                ));
            }
        };

        Ok(Self {
            imported: format_ident!("Imported{}", input.ident),
            ident: input.ident,
            fields: fields.named.into_iter().collect(),
        })
    }
}

fn setter_ident(ident: &Ident) -> Ident {
    format_ident!("set_{}", ident)
}

impl RustType {
    fn build_getters(&self) -> TokenStream {
        let imported = &self.imported;
        let mut tokens = TokenStream::new();
        for field in self.fields.iter() {
            let Field { ty, ident, .. } = &field;

            let ident = ident.as_ref().unwrap();

            tokens.extend(quote! {
                #[wasm_bindgen(method, getter)]
                fn #ident(this: &#imported) -> #ty;
            });
        }

        tokens
    }

    fn build_setters(&self) -> TokenStream {
        let imported = &self.imported;
        let mut tokens = TokenStream::new();
        for field in self.fields.iter() {
            let Field { ty, ident, .. } = &field;

            let ident = setter_ident(ident.as_ref().unwrap());

            tokens.extend(quote! {
                #[wasm_bindgen(method, setter)]
                fn #ident(this: &#imported, value: #ty);
            });
        }

        tokens
    }

    fn build_extern_block(&self, methods: impl Fn() -> TokenStream) -> TokenStream {
        let tokens = methods();
        let imported = &self.imported;
        quote! {
            use ::wasm_bindgen::prelude::*;
            #[wasm_bindgen]
            extern "C" {
                type #imported;
                #tokens
            }
        }
    }

    fn gen_from_impl(&self) -> TokenStream {
        let RustType { imported, ident: struct_ident, .. } = &self;

        let extern_block = self.build_extern_block(|| self.build_getters());
        let fields = self.fields.iter().map(|field| {
            let Field { ident, .. } = field;

            let ident = ident.as_ref().unwrap();
            quote! {
                #ident: imported.#ident(),
            }
        });


        quote! {
            #[automatically_derived]
            impl ::core::convert::From<&::wasm_bindgen::JsValue> for #struct_ident {
                fn from(value: &::wasm_bindgen::JsValue) -> Self {
                    #extern_block
                    let imported: &#imported = ::wasm_bindgen::JsValue::unchecked_ref(value);
                    #struct_ident {
                        #(#fields)*
                    }
                }
            }
        }
    }

    fn gen_into_impl(&self) -> TokenStream {
        let RustType { imported, ident: struct_ident, .. } = &self;
        let extern_block = self.build_extern_block(|| {
            let imported = &self.imported;
            let setters = self.build_setters();
            quote! {
                #[wasm_bindgen(constructor)]
                fn new() -> #imported;

                #setters
            }
        });


        let fields = self.fields.iter().map(|field| {
            let Field { ident, .. } = field;

            let setter = setter_ident(ident.as_ref().unwrap());
            quote! {
                value.#setter(self.#ident);
            }
        });

        quote! {
            #[automatically_derived]
            impl ::core::convert::Into<::wasm_bindgen::JsValue> for #struct_ident {
                fn into(self) -> ::wasm_bindgen::JsValue {
                    #extern_block
                    let value = #imported::new();
                    #(#fields)*
                    ::wasm_bindgen::JsValue::from(value)
                }
            }
        }
    }
}

impl ToTokens for RustType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.gen_from_impl());
        tokens.extend(self.gen_into_impl());
    }
}


#[proc_macro_derive(FromJsValue)]
pub fn from_js_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as RustType);
    input.into_token_stream().into()
}
