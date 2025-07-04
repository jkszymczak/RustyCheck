use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS;
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
use syn::FnArg;
use syn::TraitItemFn;
use syn::{parse_macro_input, ItemTrait, TraitItem};

use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::Mutex;
#[derive(Debug, Clone)]
pub struct MethodDecl {
    pub name: String,
    pub decl: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TraitDecl {
    pub trait_name: String,
    pub methods: Vec<MethodDecl>,
}

// GLOBAL TRAIT REGISTRY
pub static TRAIT_REGISTRY: LazyLock<Mutex<HashMap<String, TraitDecl>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn automockfn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let trait_def = parse_macro_input!(item as ItemTrait);
    let trait_name = trait_def.ident.clone();
    let mock_trait_name = format_ident!("Mock{}", trait_name);
    let trait_name_string = trait_def.ident.clone().to_string();
    let functions: Vec<TraitItemFn> = trait_def
        .items
        .iter()
        .filter_map(|item| match item {
            syn::TraitItem::Fn(val) => Some(val.clone()),
            _other => None,
        })
        .collect();
    let method_declaration: Vec<MethodDecl> = functions
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
            MethodDecl {
                decl: declaration_literal,
                name: name,
                args: args,
            }
        })
        .collect();
    TRAIT_REGISTRY.lock().unwrap().insert(
        trait_name_string.clone(),
        TraitDecl {
            trait_name: trait_name_string,
            methods: method_declaration,
        },
    );
    quote! {
        #[::mockall::automock]
        #trait_def
    }
    .into()
}
