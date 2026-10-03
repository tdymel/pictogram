use std::path::Path;

use super::{Raw, Source, svgs};

pub const SOURCE: Source = Source {
    name: "feather",
    title: "Feather",
    license: "MIT",
    crate_dir: "pictogram-icons-feather",
    repo: "https://github.com/feathericons/feather",
    sparse: &["/icons/"],
    recolor: false,
    default_branch: false,
    tag_prefix: "",
    drop_root_style: false,
    collect,
};

fn collect(root: &Path) -> Result<Vec<Raw>, String> {
    Ok(svgs(&root.join("icons"))?
        .into_iter()
        .map(|(stem, path)| Raw::new(stem, "outlined", path))
        .collect())
}
