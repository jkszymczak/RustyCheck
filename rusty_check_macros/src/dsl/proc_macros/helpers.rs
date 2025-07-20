use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};

pub fn get_tokens_from_option<K: ToTokens>(input: &Option<K>) -> TS {
    match input {
        Some(i) => i.to_token_stream(),
        None => quote! {},
    }
}
