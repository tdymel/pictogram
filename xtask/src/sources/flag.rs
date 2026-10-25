use std::path::Path;

use super::{Raw, Source, svgs};

pub const SOURCE: Source = Source {
    name: "flag",
    title: "Flag Icons",
    license: "MIT",
    crate_dir: "pictogram-icons-flag",
    repo: "https://github.com/lipis/flag-icons",
    sparse: &["/flags/"],
    recolor: false,
    default_branch: false,
    tag_prefix: "",
    drop_root_style: false,
    collect,
};

/// `flags/4x3/de.svg` is `de::landscape`, `flags/1x1/de.svg` is `de::square`. The names are the
/// ISO 3166 codes, plus a few regions (`gb-sct`) and organizations (`eu`, `un`).
fn collect(root: &Path) -> Result<Vec<Raw>, String> {
    let mut raws = Vec::new();
    for (dir, variant) in [("4x3", "landscape"), ("1x1", "square")] {
        for (stem, path) in svgs(&root.join("flags").join(dir))? {
            raws.push(Raw::new(stem, variant, path).colored());
        }
    }
    Ok(raws)
}
