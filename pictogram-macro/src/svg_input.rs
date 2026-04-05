use proc_macro2::Span;
use syn::{Path, parse::Parse};

use crate::error::Error;

pub struct SvgInput {
    pub(crate) name: String,
    pub(crate) variant: String,
    pub(crate) source: String,
}

impl Parse for SvgInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = input.parse::<Path>()?;
        let num_segments = path.segments.len();

        if num_segments > 0
            && path
                .segments
                .get(0)
                .unwrap()
                .ident
                .to_string()
                .starts_with("pictogram")
            && num_segments < 4
        {
            return Err(syn::Error::new(
                Span::call_site(),
                Error::IncompleteInputPath,
            ));
        }

        let source = path
            .segments
            .get(num_segments - 3)
            .ok_or(syn::Error::new(
                Span::call_site(),
                Error::IncompleteInputPath,
            ))?
            .ident
            .to_string();
        let mut name = path
            .segments
            .get(num_segments - 2)
            .ok_or(syn::Error::new(
                Span::call_site(),
                Error::IncompleteInputPath,
            ))?
            .ident
            .to_string();
        if name.starts_with("_") {
            name = name[1..].to_owned();
        }

        let variant = path
            .segments
            .get(num_segments - 1)
            .ok_or(syn::Error::new(
                Span::call_site(),
                Error::IncompleteInputPath,
            ))?
            .ident
            .to_string();

        Ok(Self {
            name,
            variant,
            source,
        })
    }
}
