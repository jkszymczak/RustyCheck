use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};

use proc_macro2::{Ident, TokenStream};
use syn::{
    parse2,
    visit::{self, Visit},
    Expr,
};

use super::rusty_check::configure::CommentType;

#[derive(Clone)]
pub struct Comment {
    pub string: String,
    pub values: Vec<TS>,
}

impl Comment {
    pub fn prepend_comment_string(&mut self, val: &str) {
        self.string = val.to_owned() + self.string.as_str()
    }
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
    fn to_comment(&self, comment_type: CommentType) -> Comment;
}

struct IdentSeeker {
    idents: Vec<TS>,
}

impl<'ast> Visit<'ast> for IdentSeeker {
    fn visit_pat(&mut self, pat: &'ast syn::Pat) {
        if let syn::Pat::Ident(_pat_ident) = pat {
            // Add the pattern identifier to the idents list
            self.idents.push(quote! { #pat });
        }
        visit::visit_pat(self, pat);
    }
    fn visit_expr_path(&mut self, path: &'ast syn::ExprPath) {
        // dbg!(path);
        if let Some(_ident) = path.path.get_ident() {
            // We assume that simple identifiers are variable references
            self.idents.push(path.to_token_stream());
        }
    }

    fn visit_local(&mut self, local: &'ast syn::Local) {
        let ident = &local.pat;
        // Assuming locals are always variable references
        self.idents.push(ident.to_token_stream());
        visit::visit_local(self, local);
    }
    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        // dbg!(call);
        self.idents.push(call.to_token_stream());
        // visit::visit_expr_call(self, call);
    }
    fn visit_expr_method_call(&mut self, method: &'ast syn::ExprMethodCall) {
        self.idents.push(method.to_token_stream());
    }
}
pub fn get_idents(expr: &TokenStream) -> Vec<TS> {
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
