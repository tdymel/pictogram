use std::path::Path;

use super::{Raw, Source, svgs};

pub const SOURCE: Source = Source {
    name: "simple",
    crate_dir: "pictogram-icons-simple",
    repo: "https://github.com/simple-icons/simple-icons",
    sparse: &["/icons/"],
    recolor: false,
    default_branch: false,
    collect,
};

fn collect(root: &Path) -> Result<Vec<Raw>, String> {
    Ok(svgs(&root.join("icons"))?
        .into_iter()
        .map(|(stem, path)| Raw::new(stem, "regular", path))
        .collect())
}
