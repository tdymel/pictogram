use std::path::Path;

use super::{Raw, Source, svgs};

pub const SOURCE: Source = Source {
    name: "iconoir",
    crate_dir: "pictogram-icons-iconoir",
    repo: "https://github.com/iconoir-icons/iconoir",
    sparse: &["/icons/"],
    recolor: false,
    default_branch: false,
    tag_prefix: "",
    drop_root_style: false,
    collect,
};

/// `icons/{regular,solid}/<name>.svg`. Only some of the icons have a solid variant.
fn collect(root: &Path) -> Result<Vec<Raw>, String> {
    let mut raws = Vec::new();
    for variant in ["regular", "solid"] {
        for (stem, path) in svgs(&root.join("icons").join(variant))? {
            raws.push(Raw::new(stem, variant, path));
        }
    }
    Ok(raws)
}
