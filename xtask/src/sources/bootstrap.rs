use std::path::Path;

use super::{Raw, Source, svgs};

pub const SOURCE: Source = Source {
    name: "bootstrap",
    crate_dir: "pictogram-icons-bootstrap",
    repo: "https://github.com/twbs/icons",
    sparse: &["icons"],
    recolor: false,
    collect,
};

/// `alarm.svg` is `alarm::outlined`, `alarm-fill.svg` is `alarm::filled`.
fn collect(root: &Path) -> Result<Vec<Raw>, String> {
    Ok(svgs(&root.join("icons"))?
        .into_iter()
        .map(|(stem, path)| match stem.strip_suffix("-fill") {
            Some(name) => Raw::new(name, "filled", path),
            None => Raw::new(stem, "outlined", path),
        })
        .collect())
}
