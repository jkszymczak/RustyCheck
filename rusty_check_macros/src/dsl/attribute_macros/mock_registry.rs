use quote::ToTokens;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use syn::{FnArg, ItemTrait, TraitItemFn};

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

pub static TRAIT_REGISTRY: LazyLock<Mutex<HashMap<String, TraitDecl>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn add_to_registry(trait_def: ItemTrait) {
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
}
