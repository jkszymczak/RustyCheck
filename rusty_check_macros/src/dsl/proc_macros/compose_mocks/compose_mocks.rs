use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_str, Ident};

use crate::dsl::attribute_macros::mock_registry::TRAIT_REGISTRY;
struct ComposeTraits {
    name: syn::Ident,
    traits: Vec<Ident>,
}

impl syn::parse::Parse for ComposeTraits {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let trait_a = input.parse::<syn::Ident>()?;
        input.parse::<syn::Token![,]>()?;
        let trait_b = input.parse::<syn::Ident>()?;
        input.parse::<syn::Token![,]>()?;
        let name = input.parse::<syn::Ident>()?;
        Ok(ComposeTraits {
            traits: vec![trait_a, trait_b],
            name,
        })
    }
}

pub fn compose_mocks_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ComposeTraits);
    let traits_names = input.traits.iter().clone();
    let traits_arguments = input.traits.iter().map(|t| {
        let mock_name = format_ident!("Mock{}", t);
        quote! {
            #t: #mock_name
        }
    });
    let composed_traits_struct = input.name;
    let struct_elements = traits_arguments.clone();
    let composed_trait_def = quote! {
        struct #composed_traits_struct {
            #(#struct_elements),*
        }
        impl #composed_traits_struct {
            fn new(#(#traits_arguments),*) -> Self {
                Self {
                    #(#traits_names),*
                }
            }
        }
    };

    dbg!(&TRAIT_REGISTRY);
    let traits_impl = input.traits.iter().map(|t| {
        dbg!(&t.to_string());
        if let Some(trait_decl) = TRAIT_REGISTRY.lock().unwrap().get(&t.to_string()) {
            let name: syn::Path = parse_str(&trait_decl.trait_name.clone()).unwrap();
            let methods = trait_decl.methods.iter().clone().map(|m| {
                let method_name: Ident = parse_str(&m.name.clone()).unwrap();
                let decl: syn::Signature = parse_str(&m.decl.clone()).unwrap();
                let args = m
                    .args
                    .iter()
                    .filter(|a| **a != "self".to_owned())
                    .map(|a| parse_str::<Ident>(a).unwrap());
                quote! {
                    #decl {
                        self.#name.#method_name(#(#args),*)
                    }
                }
            });
            dbg!(&methods);
            quote! {
                impl #name for #composed_traits_struct {
                    #(#methods)*
                }
            }
        } else {
            todo!("something is no yes")
        }
    });
    quote! {
        #composed_trait_def
        #(#traits_impl)*
    }
    .into()
}
