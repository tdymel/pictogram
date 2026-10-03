use std::path::Path;

use super::{Raw, Source, svgs};

pub const SOURCE: Source = Source {
    name: "tabler",
    title: "Tabler Icons",
    license: "MIT",
    crate_dir: "pictogram-icons-tabler",
    repo: "https://github.com/tabler/tabler-icons",
    sparse: &["/icons/"],
    recolor: false,
    default_branch: false,
    tag_prefix: "",
    drop_root_style: false,
    collect,
};

/// `icons/{filled,outline}/<name>.svg`
fn collect(root: &Path) -> Result<Vec<Raw>, String> {
    let mut raws = Vec::new();
    for (dir, variant) in [("filled", "filled"), ("outline", "outlined")] {
        for (stem, path) in svgs(&root.join("icons").join(dir))? {
            raws.push(Raw::new(stem, variant, path));
        }
    }
    Ok(raws)
}
