use super::{configure::Config, declaration_block::DeclarationBlock, keywords as kw};
use syn::{braced, parse::Parse, token::Brace, Ident, Token};

type Consts = DeclarationBlock<kw::consts>;
type Vars = DeclarationBlock<kw::vars>;
pub struct Global {
    kw: kw::global,
    pub config: Option<Config>,
    pub consts: Option<Consts>,
    pub vars: Option<Vars>,
}

impl Parse for Global {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kw = input.parse::<kw::global>()?;
        let content;
        braced!(content in input);

        let mut config = None;
        let mut consts = None;
        let mut vars = None;

        while !content.is_empty() {
            if content.peek(kw::cfg) {
                let item = content.parse::<Config>()?;
                if config.is_some() {
                    // return Err(syn::Error::new_spanned(item, "Duplicate `configure` block"));
                    todo!()
                }
                config = Some(item);
            } else if content.peek(kw::consts) {
                let item = content.parse::<Consts>()?;
                if consts.is_some() {
                    return Err(syn::Error::new_spanned(item, "Duplicate `constants` block"));
                }
                consts = Some(item);
            } else if content.peek(kw::vars) {
                let item = content.parse::<Vars>()?;
                if vars.is_some() {
                    return Err(syn::Error::new_spanned(item, "Duplicate `vars` block"));
                }
                vars = Some(item);
            } else {
                return Err(content.error("Expected `configure`, `constants`, or `vars` block"));
            }
        }

        Ok(Global {
            kw,
            config,
            consts,
            vars,
        })
    }
}

// impl ToTokens for Global {
//     fn to_tokens(&self, tokens: &mut TS) {
//         let config = get_tokens_from_option(&self.config);
//         let consts = get_tokens_from_option(&self.consts);
//         let vars = get_tokens_from_option(&self.vars);
//         dbg!(config);
//         let code = quote! {
//             #consts
//             #vars
//         };
//         tokens.extend(code.into_iter());
//     }
// }