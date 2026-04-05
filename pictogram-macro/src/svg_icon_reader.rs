use std::{fs, path::PathBuf};

use quote::{ToTokens, quote};
use serde::Deserialize;

use crate::{error::Error, svg_icon_input::SvgIconInput};

pub trait SvgIconReader {
    fn read(&self) -> Result<SvgJson, Error>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct SvgJson {
    pub(crate) xmlns: String,
    #[serde(rename = "viewbox")]
    pub(crate) view_box: String,
    pub(crate) body: String,
}

impl SvgIconReader for SvgIconInput {
    fn read(&self) -> Result<SvgJson, Error> {
        let manifest_dir = match self.source.as_str() {
            "material" => pictogram_icons_material::CARGO_MANIFEST_DIR,
            "bootstrap" => pictogram_icons_bootstrap::CARGO_MANIFEST_DIR,
            _ => {
                return Err(Error::IconNotFound(self.source.clone()));
            }
        };

        let mut path = PathBuf::from(manifest_dir);
        path.push("icons");
        path.push(format!("{}-{}.json", self.name, self.variant));

        let content = fs::read_to_string(&path)
            .map_err(|_| Error::IconNotFound(path.to_string_lossy().to_string()))?;

        Ok(serde_json::from_str(&content).unwrap())
    }
}

impl ToTokens for SvgJson {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let xmlns = &self.xmlns;
        let (min_x, min_y, width, height) = {
            let mut parts = self.view_box.split_whitespace();

            let min_x = parts.next().unwrap().parse::<f32>().unwrap();
            let min_y = parts.next().unwrap().parse::<f32>().unwrap();
            let width = parts.next().unwrap().parse::<f32>().unwrap();
            let height = parts.next().unwrap().parse::<f32>().unwrap();

            (min_x, min_y, width, height)
        };
        let body = &self.body;

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
