use quote::ToTokens;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use syn::{FnArg, ItemTrait, TraitItemFn};

/// Represents a method declaration within a trait.
///
/// # Fields
/// - `name`: The name of the method.
/// - `decl`: The full signature of the method as a string.
/// - `args`: A list of argument names for the method.
#[derive(Debug, Clone)]
pub struct MethodDecl {
    pub name: String,
    pub decl: String,
    pub args: Vec<String>,
}

/// Represents a trait declaration, including its name and methods.
///
/// # Fields
/// - `trait_name`: The name of the trait.
/// - `methods`: A list of method declarations within the trait.
#[derive(Debug, Clone)]
pub struct TraitDecl {
    pub trait_name: String,
    pub methods: Vec<MethodDecl>,
}

/// A global registry for storing trait declarations.
///
/// This registry is a thread-safe, lazily initialized `HashMap`
/// that maps trait names to their corresponding `TraitDecl` objects.
pub static TRAIT_REGISTRY: LazyLock<Mutex<HashMap<String, TraitDecl>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Adds a trait definition to the global `TRAIT_REGISTRY`.
///
/// # Parameters
/// - `trait_def`: The `ItemTrait` representing the trait to be added.
///
/// This function extracts the trait's name and its methods, converts them
/// into `TraitDecl` and `MethodDecl` objects, and stores them in the registry.
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

    /// Converts the list of trait methods into `MethodDecl` objects.
    ///
    /// Each method's signature, name, and arguments are extracted and stored.
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

    /// Inserts the trait declaration into the global registry.
    TRAIT_REGISTRY.lock().unwrap().insert(
        trait_name_string.clone(),
        TraitDecl {
            trait_name: trait_name_string,
            methods: method_declaration,
        },
    );
}
