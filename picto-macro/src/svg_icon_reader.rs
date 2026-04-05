use std::{fs, path::PathBuf};

use crate::{error::Error, svg_icon_input::SvgIconInput};

pub trait SvgIconReader {
    fn read(&self) -> Result<String, Error>;
}

impl SvgIconReader for SvgIconInput {
    fn read(&self) -> Result<String, Error> {
        let manifest_dir = match self.source.as_str() {
            "material" => picto_icons_material::CARGO_MANIFEST_DIR,
            _ => {
                return Err(Error::IconNotFound(self.source.clone()));
            }
        };

        let mut path = PathBuf::from(manifest_dir);
        path.push("icons");
        path.push(format!("{}-{}.svg", self.name, self.variant));

        fs::read_to_string(&path)
            .map_err(|_| Error::IconNotFound(path.to_string_lossy().to_string()))
    }
}
