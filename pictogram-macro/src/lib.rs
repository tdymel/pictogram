#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

use crate::svg_input::SvgInput;

mod error;
mod svg_input;
mod svg_parser;

/// Proc macro used internally by pictogram
#[proc_macro]
pub fn svg_icon(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as SvgInput)
        .to_token_stream()
        .into()
}
