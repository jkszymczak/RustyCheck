use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemTrait, TraitItem};

pub fn automockfn(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    let item_clone = item.clone();
    let trait_def = parse_macro_input!(item_clone as ItemTrait);
    match trait_def.items.first().unwrap() {
        TraitItem::Fn(func) => {
            let sig = func.sig.to_token_stream();
            dbg!(sig.to_string());
        }
        other => {}
    }
    item
}
