use std::{collections::BTreeMap, path::Path};

use super::{Raw, Source, svgs};

pub const SOURCE: Source = Source {
    name: "oct",
    crate_dir: "pictogram-icons-oct",
    repo: "https://github.com/primer/octicons",
    sparse: &["/icons/"],
    recolor: false,
    default_branch: false,
    tag_prefix: "",
    drop_root_style: false,
    collect,
};

/// Only the 24px icons. `alert-24.svg` is `alert::outlined`, `alert-fill-24.svg` is `alert::filled`.
///
/// Some icons come as `-fill` and as `-inset`, both are filled. `-fill` wins.
fn collect(root: &Path) -> Result<Vec<Raw>, String> {
    let mut icons: BTreeMap<(String, &str), (bool, Raw)> = BTreeMap::new();
    for (stem, path) in svgs(&root.join("icons"))? {
        let Some(base) = stem.strip_suffix("-24") else {
            continue;
        };
        let (name, variant, is_fill) = if let Some(name) = base.strip_suffix("-fill") {
            (name, "filled", true)
        } else if let Some(name) = base.strip_suffix("-inset") {
            (name, "filled", false)
        } else {
            (base, "outlined", true)
        };
        let key = (name.to_owned(), variant);
        if icons
            .get(&key)
            .is_some_and(|(existing_is_fill, _)| *existing_is_fill)
            && !is_fill
        {
            continue;
        }
        icons.insert(key, (is_fill, Raw::new(name, variant, path)));
    }
    Ok(icons.into_values().map(|(_, raw)| raw).collect())
}
