use std::collections::{BTreeMap, HashMap};

use super::keywords as kw;
use proc_macro2::{TokenStream as TS, TokenTree};
use quote::{quote, ToTokens};
use syn::{braced, parse::Parse, punctuated, token::Brace, Ident, Token};

/// Represents a configuration block in the RustyCheck DSL.
///
/// A `Config` block is used to define configuration options for a test case.
/// It contains:
/// - `keyword`: The `cfg` keyword that introduces the block.
/// - `elements`: The token stream representing the configuration values.
///
/// represents grammar from this diagram:
///
#[doc = include_str!("../../../../../grammar/global/cfg.svg")]
#[derive(Clone)]
pub struct Config {
    keyword: kw::cfg,
    pub options: HashMap<ConfigOptionName, ConfigOption>,
}

macro_rules! create_cfg_getters {
    ($name:ident,$option:ident,$t:ty) => {
        pub fn $name(&self) -> Option<$t> {
            self.options
                .get(&ConfigOptionName::$option)
                .map(|v| match v {
                    ConfigOption::$option(val) => val.clone(),
                    _ => panic!(),
                })
        }
    };
}
impl Config {
    create_cfg_getters!(get_cfg_flags, CfgFlags, TS);
    create_cfg_getters!(get_comment_type, CommentType, CommentType);
    create_cfg_getters!(get_unstabe_test, TestUnstable, bool);

    pub fn merge_with_global(self, other: &Config) -> Config {
        let mut combined = self.options.clone();
        if let Some(comment_type) = other.get_comment_type() {
            combined
                .entry(ConfigOptionName::CommentType)
                .or_insert(ConfigOption::CommentType(comment_type));
        }
        if let Some(test_unstable) = other.get_unstabe_test() {
            combined
                .entry(ConfigOptionName::TestUnstable)
                .or_insert(ConfigOption::TestUnstable(test_unstable));
        }
        Config {
            options: combined,
            ..self
        }
    }
    pub fn default() -> Config {
        Config {
            keyword: kw::cfg(proc_macro2::Span::call_site()),
            options: HashMap::from([(
                ConfigOptionName::CommentType,
                ConfigOption::CommentType(CommentType::default()),
            )]),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Copy)]
pub enum CommentType {
    Simple,
    ShowValues,
}

impl Default for CommentType {
    fn default() -> Self {
        CommentType::ShowValues
    }
}

impl CommentType {
    fn new_default() -> CommentType {
        CommentType::ShowValues
    }
}

macro_rules! enum_with_names {
    (
        enum $EnumName:ident {
            $(
                $Variant:ident $( ( $($Field:ty),* $(,)? ) )?
                $( { $($StructField:ident : $StructTy:ty),* $(,)? } )?
            ),* $(,)?
        }, $EnumWithNames:ident
    ) => {
        #[derive(Clone)]
        pub enum $EnumName {
            $(
                $Variant $( ( $($Field),* ) )?
                $( { $($StructField : $StructTy),* } )?
            ),*
        }

        #[derive(Clone,Debug, PartialEq, Eq, Hash)]
        pub enum $EnumWithNames {
            $(
                $Variant
            ),*
        }
    };
}
enum_with_names!(
    enum ConfigOption {
        CfgFlags(TS),
        CommentType(CommentType),
        TestUnstable(bool),
    },
    ConfigOptionName
);

impl Parse for ConfigOption {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(kw::cfg) {
            let kw = input.parse::<kw::cfg>()?;

            _ = input.parse::<Token![=]>()?;
            return parse_cfg_option(input);
        }
        if input.peek(kw::comment) {
            let kw = input.parse::<kw::comment>()?;

            _ = input.parse::<Token![=]>()?;
            return parse_comment_option(input);
        }
        if input.peek(kw::unstable) {
            let kw = input.parse::<kw::unstable>()?;
            _ = input.parse::<Token![=]>()?;
            let val = input.parse::<syn::LitBool>()?.value;
            return Ok(ConfigOption::TestUnstable(val));
        }

        Err(input.error("Unknown configuration option"))
    }
}

fn parse_comment_option(input: syn::parse::ParseStream) -> syn::Result<ConfigOption> {
    if input.peek(kw::simple) {
        _ = input.parse::<kw::simple>()?;
        return Ok(ConfigOption::CommentType(CommentType::Simple));
    }
    if input.peek(kw::show) {
        _ = input.parse::<kw::show>()?;
        _ = input.parse::<kw::values>()?;
        return Ok(ConfigOption::CommentType(CommentType::ShowValues));
    }
    Err(input.error("Unknown value for comment type"))
}

fn parse_cfg_option(input: syn::parse::ParseStream) -> syn::Result<ConfigOption> {
    let mut value_tokens = TS::new();
    while !input.is_empty() {
        let fork = input.fork();
        if fork.peek(Ident) && fork.peek2(Brace) {
            break; // found start of next statement
        }
        let tt: TokenTree = input.parse()?; // consume one token
        value_tokens.extend(std::iter::once(tt));
    }
    Ok(ConfigOption::CfgFlags(value_tokens))
}

impl Parse for Config {
    /// Parses a `Config` block from the input stream.
    ///
    /// # Parameters
    /// - `input`: The parse stream to read from.
    ///
    /// # Returns
    /// A parsed `Config` instance containing the `cfg` keyword and the configuration values.
    ///
    /// # Errors
    /// Returns a `syn::Error` if the input cannot be parsed as a valid `Config` block.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kw = input.parse::<kw::cfg>()?;
        if input.peek(Token![=]) {
            _ = input.parse::<Token![=]>()?;
            let cfg_option = parse_cfg_option(input)?;
            Ok(Config {
                keyword: kw,
                options: HashMap::from([(ConfigOptionName::CfgFlags, cfg_option)]),
            })
        } else {
            let cfg;
            braced!(cfg in input);
            let options = cfg.parse_terminated(ConfigOption::parse, Token![,])?;
            let map = options
                .into_iter()
                .map(|opt| match opt {
                    ConfigOption::CfgFlags(_) => (ConfigOptionName::CfgFlags, opt),
                    ConfigOption::CommentType(_) => (ConfigOptionName::CommentType, opt),
                    ConfigOption::TestUnstable(_) => (ConfigOptionName::TestUnstable, opt),
                })
                .collect();
            Ok(Config {
                keyword: kw,
                options: map,
            })
        }
    }
}
