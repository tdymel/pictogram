use std::{fs, path::PathBuf};

use proc_macro2::Span;
use quote::{ToTokens, quote};
use roxmltree::Document;

use crate::{error::Error, svg_input::SvgInput};

#[derive(Debug, Clone)]
pub struct Svg {
    pub(crate) xmlns: String,
    pub(crate) view_box: (f32, f32, f32, f32),
    pub(crate) body: String,
}

fn parse_svg(input: &SvgInput) -> Result<Svg, Error> {
    let manifest_dir = match input.source.as_str() {
        #[cfg(feature = "material")]
        "material" => pictogram_icons_material::CARGO_MANIFEST_DIR,
        #[cfg(feature = "bootstrap")]
        "bootstrap" => pictogram_icons_bootstrap::CARGO_MANIFEST_DIR,
        #[cfg(feature = "feather")]
        "feather" => pictogram_icons_feather::CARGO_MANIFEST_DIR,
        #[cfg(feature = "font-awesome")]
        "font_awesome" => pictogram_icons_font_awesome::CARGO_MANIFEST_DIR,
        _ => return Err(Error::SourceNotInstalled(input.source.clone())),
    };

    let mut path = PathBuf::from(manifest_dir);
    path.push("icons");
    path.push(format!("{}-{}.svg", input.name, input.variant));

    let content = fs::read_to_string(&path)
        .map_err(|_| Error::IconNotFound(path.to_string_lossy().to_string()))?;

    let doc = Document::parse(&content).unwrap();
    let root = doc.root_element();

    let view_box = root.attribute("viewBox").unwrap_or("").to_string();
    let xmlns = root.tag_name().namespace().unwrap_or("").to_string();

    let range = root.range();
    let full_svg = &content[range];

    let start = full_svg.find('>').unwrap() + 1;
    let end = full_svg.rfind("</").unwrap();
    let body = full_svg[start..end].trim();

    let (min_x, min_y, width, height) = {
        let mut parts = view_box.split_whitespace();

        let min_x = parts.next().unwrap().parse::<f32>().unwrap();
        let min_y = parts.next().unwrap().parse::<f32>().unwrap();
        let width = parts.next().unwrap().parse::<f32>().unwrap();
        let height = parts.next().unwrap().parse::<f32>().unwrap();

        (min_x, min_y, width, height)
    };

    Ok(Svg {
        xmlns,
        view_box: (min_x, min_y, width, height),
        body: body.to_string(),
    })
}

impl ToTokens for SvgInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let svg = parse_svg(self);
        if svg.is_err() {
            tokens
                .extend(syn::Error::new(Span::call_site(), svg.err().unwrap()).to_compile_error());
            return;
        }
        let Svg {
            xmlns,
            view_box: (min_x, min_y, width, height),
            body,
        } = svg.unwrap();

        tokens.extend(quote! {
            pictogram::Svg {
                xmlns: #xmlns,
                view_box: pictogram::ViewBox {
                    min_x: #min_x,
                    min_y: #min_y,
                    width: #width,
                    height: #height
                },
                body: #body
            }
        });
    }
}
