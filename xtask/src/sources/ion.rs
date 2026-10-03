use std::path::Path;

use super::{Raw, Source, svgs};

pub const SOURCE: Source = Source {
    name: "ion",
    title: "Ionicons",
    license: "MIT",
    crate_dir: "pictogram-icons-ion",
    repo: "https://github.com/ionic-team/ionicons",
    sparse: &["/src/svg/"],
    // The outlines are drawn with a hard coded #000
    recolor: true,
    default_branch: false,
    tag_prefix: "",
    drop_root_style: false,
    collect,
};

/// `home.svg` is `home::filled`, `home-outline.svg` is `home::outlined`, `home-sharp.svg` is `home::sharp`.
fn collect(root: &Path) -> Result<Vec<Raw>, String> {
    Ok(svgs(&root.join("src/svg"))?
        .into_iter()
        .map(|(stem, path)| {
            if let Some(name) = stem.strip_suffix("-outline") {
                Raw::new(name, "outlined", path)
            } else if let Some(name) = stem.strip_suffix("-sharp") {
                Raw::new(name, "sharp", path)
            } else {
                Raw::new(stem, "filled", path)
            }
        })
        .collect())
}
