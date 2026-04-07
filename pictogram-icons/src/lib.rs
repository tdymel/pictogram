#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};
use std::{collections::BTreeMap, fs, path::PathBuf};

#[proc_macro]
pub fn generate_index(_: TokenStream) -> TokenStream {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("icons");

    let entries = match fs::read_dir(&root) {
        Ok(entries) => entries,
        Err(_) => {
            return quote! {
                compile_error!("Source folder not found")
            }
            .into();
        }
    };

    let mut grouped: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for (name, variant) in entries.filter_map(|entry| {
        let path = entry.ok()?.path();

        (path.extension().and_then(|e| e.to_str()) == Some("svg"))
            .then_some(path)?
            .file_stem()
            .and_then(|s| s.to_str())
            .and_then(|stem| {
                let (name, variant) = stem.rsplit_once('-')?;
                (!name.is_empty() && !variant.is_empty()).then(|| {
                    (
                        if name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                            format!("_{name}")
                        } else {
                            name.to_string()
                        },
                        variant.to_string(),
                    )
                })
            })
    }) {
        grouped.entry(name).or_default().push(variant);
    }

    let modules = grouped.into_iter().map(|(name, mut variants)| {
        variants.sort();
        variants.dedup();

        let module = format_ident!("r#{}", name, span = Span::call_site());
        let consts = variants.into_iter().map(|variant| {
            let ident = Ident::new(&variant, Span::call_site());
            quote! { pub const #ident: () = (); }
        });

        quote! {
            pub mod #module {
                #(#consts)*
            }
        }
    });

    quote! {
        #(#modules)*
    }
    .into()
}
