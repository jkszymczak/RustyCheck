use super::{configure::Config, declaration_block::DeclarationBlock, keywords as kw};
use syn::{braced, parse::Parse};

type Consts = DeclarationBlock<kw::consts>;
type Vars = DeclarationBlock<kw::vars>;

/// Represents a block of global declarations in the RustyCheck DSL.
///
/// A `Global` block is used to define global configurations, constants, and variables.
/// It contains:
/// - `kw`: The `global` keyword.
/// - `config`: An optional configuration block.
/// - `consts`: An optional block of constants.
/// - `vars`: An optional block of variables.
///
/// represents grammar from this diagram:
///
#[doc = include_str!("../../../../../grammar/global/global.svg")]
pub struct Global {
    kw: kw::global,
    pub config: Option<Config>,
    pub consts: Option<Consts>,
    pub vars: Option<Vars>,
}

impl Parse for Global {
    /// Parses a `Global` block from the input stream.
    ///
    /// # Parameters
    /// - `input`: The parse stream to read from.
    ///
    /// # Returns
    /// A parsed `Global` instance containing the `global` keyword, configuration, constants, and variables.
    ///
    /// # Errors
    /// Returns a `syn::Error` if the input cannot be parsed as a valid `Global` block or if duplicate blocks are found.
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
                    return Err(syn::Error::new_spanned(item, "Duplicate `configure` block"));
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
