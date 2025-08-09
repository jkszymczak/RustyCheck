use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};

use proc_macro2::{Ident, TokenStream};
use syn::{
    parse2,
    visit::{self, Visit},
    Expr,
};

pub struct Comment {
    pub string: String,
    pub values: Vec<TS>,
}

impl ToTokens for Comment {
    fn to_tokens(&self, tokens: &mut TS) {
        let string = &self.string;
        let where_str = if self.values.is_empty() {
            String::new()
        } else {
            format!(
                " where, {}",
                self.values
                    .iter()
                    .map(|v| format!("{}={{:?}}", v))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        let values = &self.values;
        let comment = string.to_owned() + where_str.as_str();
        tokens.extend(quote! {
            #comment #(, #values)*
        });
    }
}

pub trait ToComment {
    fn to_comment(&self) -> Comment;
}

pub fn get_tokens_from_option<K: ToTokens>(input: &Option<K>) -> TS {
    match input {
        Some(i) => i.to_token_stream(),
        None => quote! {},
    }
}

struct IdentSeeker {
    idents: Vec<Ident>,
}

impl<'ast> Visit<'ast> for IdentSeeker {
    fn visit_ident(&mut self, i: &'ast Ident) {
        self.idents.push(i.clone());
        visit::visit_ident(self, i);
    }
}
pub fn get_idents(expr: &TokenStream) -> Vec<Ident> {
    let expr = parse2::<Expr>(expr.clone()).expect("Expected Expression");
    let mut visitor = IdentSeeker { idents: vec![] };
    visitor.visit_expr(&expr);
    visitor.idents
}

struct IdentFinder<'a> {
    target: &'a Ident,
    found: bool,
}

impl<'a, 'ast> Visit<'ast> for IdentFinder<'a> {
    fn visit_ident(&mut self, i: &'ast Ident) {
        if i == self.target {
            self.found = true;
        }
        visit::visit_ident(self, i);
    }
}

fn contains_ident(expr: &Expr, ident: &Ident) -> bool {
    let mut visitor = IdentFinder {
        target: ident,
        found: false,
    };
    visitor.visit_expr(expr);
    visitor.found
}

pub fn filter_out_streams_with_ident<'a>(
    streams: impl IntoIterator<Item = &'a TokenStream>,
    ident: &Ident,
) -> Vec<&'a TokenStream> {
    streams
        .into_iter()
        .filter(|ts| {
            match parse2::<Expr>((*ts).clone()) {
                Ok(expr) => !contains_ident(&expr, ident),
                Err(_) => true, // if not parsable, keep it
            }
        })
        .collect()
}
