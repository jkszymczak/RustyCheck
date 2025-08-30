use std::collections::HashMap;

use super::keywords as kw;
use proc_macro2::{TokenStream as TS, TokenTree};
use quote::ToTokens;
use syn::{braced, parse::Parse, token::Brace, Ident, Token};

/// Represents a configuration block in the RustyCheck DSL.
///
/// A `Config` block is used to define configuration options for a test case.
/// It contains:
/// - `elements`: The token stream representing the configuration values.
///
/// represents grammar from this diagram:
///
#[derive(Clone, Debug)]
pub struct Config {
    pub options: HashMap<ConfigOptionName, ConfigOption>,
}
macro_rules! create_cfg_getters {
    ($name:ident,$option:ident,$field:ident,$t:ty) => {
        pub fn $name(&self) -> $t {
            if let Some(value) = self
                .options
                .get(&ConfigOptionName::$option)
                .map(|v| match v {
                    ConfigOption::$option { $field, .. } => $field.clone(),
                    _ => panic!(),
                })
            {
                value
            } else {
                Self::default()
                    .options
                    .get(&ConfigOptionName::$option)
                    .map(|v| match v {
                        ConfigOption::$option { $field, .. } => $field.clone(),
                        _ => panic!(),
                    })
                    .unwrap()
            }
        }
    };
}
impl Config {
    pub fn new() -> Self {
        Config {
            options: HashMap::new(),
        }
    }
    create_cfg_getters!(get_cfg_flags, CfgFlags, flags, TS);
    create_cfg_getters!(get_comment_type, CommentType, comment_type, CommentType);
    create_cfg_getters!(get_unstable_test, TestUnstable, value, bool);
    create_cfg_getters!(get_module_name, ModuleName, name, Ident);
    create_cfg_getters!(get_create_module, CreateModule, value, bool);

    pub fn merge_with_other(self, other: &Config) -> Config {
        let mut combined = self.options.clone();
        for (k, v) in other.options.iter() {
            combined.entry(k.clone()).or_insert(v.clone());
        }
        Self {
            options: combined,
            ..self
        }
    }
    pub fn merge_with_default(self) -> Self {
        let default = Self::default();
        self.merge_with_other(&default)
    }
    pub fn merge_with_other_and_default(self, other: &Config) -> Config {
        let with_other = self.merge_with_other(other);
        with_other.merge_with_default()
    }
}
impl Default for Config {
    fn default() -> Self {
        Config {
            options: HashMap::from([
                (
                    ConfigOptionName::CommentType,
                    ConfigOption::CommentType {
                        comment_type: CommentType::default(),
                    },
                ),
                (
                    ConfigOptionName::ModuleName,
                    ConfigOption::ModuleName {
                        name: Ident::new("tests", proc_macro2::Span::call_site()),
                    },
                ),
                (
                    ConfigOptionName::CfgFlags,
                    ConfigOption::CfgFlags { flags: TS::new() },
                ),
                (
                    ConfigOptionName::TestUnstable,
                    ConfigOption::TestUnstable { value: false },
                ),
                (
                    ConfigOptionName::CreateModule,
                    ConfigOption::CreateModule { value: true },
                ),
            ]),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Copy)]
pub enum CommentType {
    Simple,
    ShowValues,
}

impl Default for CommentType {
    fn default() -> Self {
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
        #[derive(Clone,Debug)]
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
        CfgFlags { flags: TS },
        CommentType { comment_type: CommentType },
        TestUnstable { value: bool },
        ModuleName { name: Ident },
        CreateModule { value: bool },
    },
    ConfigOptionName
);

impl Parse for ConfigOption {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(kw::cfg) {
            _ = input.parse::<kw::cfg>()?;

            _ = input.parse::<Token![=]>()?;
            return parse_cfg_option(input);
        }
        if input.peek(kw::comment) {
            return parse_comment_option(input);
        }
        if input.peek(kw::unstable) {
            _ = input.parse::<kw::unstable>()?;
            _ = input.parse::<Token![=]>()?;
            let val = input.parse::<syn::LitBool>()?.value;
            return Ok(ConfigOption::TestUnstable { value: val });
        }
        if input.peek(kw::module) {
            _ = input.parse::<kw::module>()?;
            _ = input.parse::<kw::name>()?;
            _ = input.parse::<Token![=]>()?;
            let name = input.parse::<Ident>()?;
            return Ok(ConfigOption::ModuleName { name: name });
        }
        if input.peek(kw::create) {
            _ = input.parse::<kw::create>()?;
            _ = input.parse::<kw::module>()?;
            _ = input.parse::<Token![=]>()?;
            let val = input.parse::<syn::LitBool>()?.value;
            return Ok(ConfigOption::CreateModule { value: val });
        }

        Err(input.error("Unknown configuration option"))
    }
}

fn parse_comment_option(input: syn::parse::ParseStream) -> syn::Result<ConfigOption> {
    _ = input.parse::<kw::comment>()?;
    _ = input.parse::<Token![=]>()?;
    if input.peek(kw::simple) {
        _ = input.parse::<kw::simple>()?;
        return Ok(ConfigOption::CommentType {
            comment_type: CommentType::Simple,
        });
    }
    if input.peek(kw::show) {
        _ = input.parse::<kw::show>()?;
        _ = input.parse::<kw::values>()?;
        return Ok(ConfigOption::CommentType {
            comment_type: CommentType::ShowValues,
        });
    }
    Err(input.error("Unknown value for comment type"))
}

fn parse_cfg_option(input: syn::parse::ParseStream) -> syn::Result<ConfigOption> {
    if input.peek(syn::LitBool) {
        Ok(ConfigOption::CfgFlags {
            flags: input.parse::<syn::LitBool>()?.to_token_stream(),
        })
    } else {
        Ok(ConfigOption::CfgFlags {
            flags: input.parse::<syn::Meta>()?.to_token_stream(),
        })
    }
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
        _ = input.parse::<kw::cfg>()?;
        if input.peek(Token![=]) {
            _ = input.parse::<Token![=]>()?;
            let cfg_option = parse_cfg_option(input)?;
            Ok(Config {
                options: HashMap::from([(ConfigOptionName::CfgFlags, cfg_option)]),
            })
        } else {
            let cfg;
            braced!(cfg in input);
            let options = cfg.parse_terminated(ConfigOption::parse, Token![,])?;
            let map = options
                .into_iter()
                .map(|opt| match opt {
                    ConfigOption::CfgFlags { .. } => (ConfigOptionName::CfgFlags, opt),
                    ConfigOption::CommentType { .. } => (ConfigOptionName::CommentType, opt),
                    ConfigOption::TestUnstable { .. } => (ConfigOptionName::TestUnstable, opt),
                    ConfigOption::ModuleName { .. } => (ConfigOptionName::ModuleName, opt),
                    ConfigOption::CreateModule { .. } => (ConfigOptionName::CreateModule, opt),
                })
                .collect();
            Ok(Config { options: map })
        }
    }
}
