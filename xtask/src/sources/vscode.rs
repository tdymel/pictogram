use std::path::Path;

use super::{Raw, Source, svgs};

pub const SOURCE: Source = Source {
    name: "vscode",
    crate_dir: "pictogram-icons-vscode",
    repo: "https://github.com/microsoft/vscode-codicons",
    sparse: &["/src/icons/"],
    recolor: false,
    default_branch: false,
    collect,
};

fn collect(root: &Path) -> Result<Vec<Raw>, String> {
    Ok(svgs(&root.join("src/icons"))?
        .into_iter()
        .map(|(stem, path)| Raw::new(stem, "regular", path))
        .collect())
}
