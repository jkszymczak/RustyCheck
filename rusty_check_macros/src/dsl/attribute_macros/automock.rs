use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS;
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
use syn::FnArg;
use syn::TraitItemFn;
use syn::{parse_macro_input, ItemTrait, TraitItem};

pub fn automockfn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let trait_def = parse_macro_input!(item as ItemTrait);
    let trait_name = trait_def.ident.clone();
    let mock_trait_name = format_ident!("Mock{}", trait_name);
    let functions: Vec<TraitItemFn> = trait_def
        .items
        .iter()
        .filter_map(|item| match item {
            syn::TraitItem::Fn(val) => Some(val.clone()),
            _other => None,
        })
        .collect();
    let method_declaration: Vec<TS> = functions
        .iter()
        .map(|f| {
            let declaration_literal = f.sig.clone().into_token_stream().to_string();
            let name = f.sig.ident.clone().to_string();
            let args: Vec<String> = f
                .sig
                .inputs
                .iter()
                .filter_map(|arg| match arg {
                    FnArg::Receiver(_) => Some("self".to_string()),
                    FnArg::Typed(pat) => match &*pat.pat {
                        syn::Pat::Ident(ident) => Some(ident.ident.to_string()),
                        _ => None,
                    },
                })
                .collect();
            let args_str = args.join(", ");
            quote! {
                rusty_check::mocks::MethodDeclaration {
                    declaration_literal: #declaration_literal,
                    name: #name,
                    args: #args_str
                }
            }
            .into()
        })
        .collect();
    quote! {
        #[::mockall::automock]
        #trait_def
        impl rusty_check::mocks::ComposableMock for #mock_trait_name {
            fn get_methods(&self) -> Vec<rusty_check::mocks::MethodDeclaration> {
                vec![#(#method_declaration),*]
            }
        }
    }
    .into()
}
