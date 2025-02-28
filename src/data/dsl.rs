use crate::data::{check::Check, declare_data::DeclareData, keywords as kw, traits::Code};
use quote::quote;
use syn::{braced, parse::Parse, Ident, Token};

pub struct Case {
    keyword: kw::case,
    name: Ident,
    declare_data: Option<DeclareData>,
    check_condition: Check,
}

impl Parse for Case {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kw = input.parse::<kw::case>()?;
        let name = input.parse::<Ident>()?;
        let case_block;
        let mut declarations: Option<DeclareData> = None;
        braced!(case_block in input);
        if case_block.peek(kw::given) {
            declarations = Some(case_block.parse::<DeclareData>()?);
        }
        let conditions = case_block.parse::<Check>()?;
        if case_block.peek(Token![where]) && declarations.is_none() {
            declarations = Some(case_block.parse::<DeclareData>()?);
        }
        Ok(Case {
            keyword: kw,
            name: name,
            declare_data: declarations,
            check_condition: conditions,
        })
    }
}

impl Code for Case {
    fn get_code(&self) -> proc_macro2::TokenStream {
        let name = self.name.clone();
        let declarations = match &self.declare_data {
            Some(x) => x.get_code(),
            None => quote! {""},
        };
        let case = self.check_condition.get_code();
        quote! {
            #[test]
            fn #name() {
                #declarations
                #case
            }
        }
        .into()
    }
}

struct DSL {
    cases: Vec<Case>,
}
