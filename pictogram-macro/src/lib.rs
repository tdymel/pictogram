#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::parse_macro_input;

use crate::{svg_icon_input::SvgIconInput, svg_icon_reader::SvgIconReader};

mod error;
mod svg_icon_input;
mod svg_icon_reader;

/// Proc macro used internally by pictogram
#[proc_macro]
pub fn svg_icon(input: TokenStream) -> TokenStream {
    match parse_macro_input!(input as SvgIconInput).read() {
        Ok(svg) => svg.to_token_stream().into(),
        Err(e) => syn::Error::new(Span::call_site(), e.to_string())
            .to_compile_error()
            .into(),
    }
}
