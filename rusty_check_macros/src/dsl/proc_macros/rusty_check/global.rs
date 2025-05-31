use std::marker::PhantomData;

use super::expression::Expression;
use super::{super::super::traits::Code, keywords as kw};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS;
use quote::quote;
use syn::{braced, custom_keyword, parse::Parse, Token};

pub struct Global {
    keyword: kw::global,
    config: Config,
}

pub struct Config {
    keyword: kw::configure,
}
