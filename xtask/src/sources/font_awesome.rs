use std::path::Path;

use super::{Raw, Source, svgs};

pub const SOURCE: Source = Source {
    name: "font-awesome",
    crate_dir: "pictogram-icons-font-awesome",
    repo: "https://github.com/FortAwesome/Font-Awesome",
    sparse: &["/svgs/"],
    recolor: false,
    default_branch: false,
    collect,
};

/// `svgs/<variant>/<name>.svg`
fn collect(root: &Path) -> Result<Vec<Raw>, String> {
    let mut raws = Vec::new();
    for variant in ["brands", "regular", "solid"] {
        for (stem, path) in svgs(&root.join("svgs").join(variant))? {
            raws.push(Raw::new(stem, variant, path));
        }
    }
    Ok(raws)
}
