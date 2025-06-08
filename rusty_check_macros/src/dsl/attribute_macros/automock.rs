use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
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
    quote! {
        #[::mockall::automock]
        #trait_def
        impl rusty_check::mocks::ComposableMock for #mock_trait_name {
            fn get_methods(&self) -> Vec<rusty_check::mocks::MethodDeclaration> {
                todo!();
            }
            fn compose<M,C>(&self, other: M) -> C {
                todo!();
            }

        }
    }
    .into()
}
