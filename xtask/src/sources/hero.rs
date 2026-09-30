use std::path::Path;

use super::{Raw, Source, svgs};

pub const SOURCE: Source = Source {
    name: "hero",
    crate_dir: "pictogram-icons-hero",
    repo: "https://github.com/tailwindlabs/heroicons",
    sparse: &["src/24"],
    // The icons are drawn with a hard coded #0F172A
    recolor: true,
    collect,
};

/// `src/24/{solid,outline}/<name>.svg`, the other sizes are separate drawings.
fn collect(root: &Path) -> Result<Vec<Raw>, String> {
    let mut raws = Vec::new();
    for (dir, variant) in [("solid", "solid"), ("outline", "outlined")] {
        for (stem, path) in svgs(&root.join("src/24").join(dir))? {
            raws.push(Raw::new(stem, variant, path));
        }
    }
    Ok(raws)
}
