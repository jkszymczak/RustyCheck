mod dsl;
use dsl::proc_macros::rusty_check::{
    configure::{CommentType, Config, ConfigOption, ConfigOptionName},
    rusty_check::RustyCheck,
};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::ToTokens;
use std::{collections::HashMap, env, error::Error, fs};
use syn::{parse_macro_input, parse_str};
use toml::{Table, Value};
/// RustyCheck procedural macro that processes the `rusty_check!` DSL.
/// Follows grammar from this diagram:

fn read_config() -> Result<Config, Box<dyn Error>> {
    if let Some(path) = option_env!("RUSTY_CONFIG") {
        let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push(path);
        let path = p.clone();
        let contents = match fs::read_to_string(&p) {
            Ok(c) => c,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Err(Box::new(e));
            }
            Err(e) => return Err(Box::new(e)),
        };
        let config: HashMap<String, Value> = toml::from_str(&contents)?;
        let mut options: Vec<(ConfigOptionName, ConfigOption)> = Vec::new();
        for (k, v) in config {
            options.push(match k.as_str() {
                "moduleName" => (
                    ConfigOptionName::ModuleName,
                    ConfigOption::ModuleName {
                        name: Ident::new(v.as_str().unwrap(), proc_macro2::Span::call_site()),
                    },
                ),
                "createModule" => (
                    ConfigOptionName::CreateModule,
                    ConfigOption::CreateModule {
                        value: v.as_bool().unwrap(),
                    },
                ),
                "unstable" => (
                    ConfigOptionName::TestUnstable,
                    ConfigOption::TestUnstable {
                        value: v.as_bool().unwrap(),
                    },
                ),
                "cfg" => (
                    ConfigOptionName::CfgFlags,
                    ConfigOption::CfgFlags {
                        flags: parse_str(v.as_str().unwrap())?,
                    },
                ),
                "comment" => (
                    ConfigOptionName::CommentType,
                    ConfigOption::CommentType {
                        comment_type: match v.as_str().unwrap() {
                            "simple" => CommentType::Simple,
                            "showValues" => CommentType::ShowValues,
                            _ => todo!(),
                        },
                    },
                ),
                _ => todo!(),
            });
        }
        Ok(Config {
            options: options.into_iter().collect(),
        })
    } else {
        Ok(Config::default())
    }
}

#[proc_macro]
pub fn rusty_check(input: TokenStream) -> TokenStream {
    // let config = read_config().unwrap();
    // let rusty = parse_macro_input!(input as RustyCheck).apply_config_file(&config);
    // rusty.to_token_stream().into()
    match read_config() {
        Ok(config) => {
            let rusty = parse_macro_input!(input as RustyCheck).apply_config_file(&config);
            rusty.to_token_stream().into()
        }
        Err(e) => {
            // Emit a proper compiler error instead of panicking
            let msg = format!("Failed to read config: {}", e);
            syn::Error::new(proc_macro2::Span::call_site(), msg)
                .to_compile_error()
                .into()
        }
    }
}
