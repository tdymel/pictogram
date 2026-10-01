use std::{
    fs,
    path::{Path, PathBuf},
};

use super::{Raw, Source};

pub const SOURCE: Source = Source {
    name: "material",
    crate_dir: "pictogram-icons-material",
    repo: "https://github.com/google/material-design-icons",
    // The repository is huge: only the 24px (and the rare 20px) svgs are needed
    sparse: &["/src/*/*/*/24px.svg", "/src/*/*/*/20px.svg"],
    recolor: false,
    // Google does not tag releases any more, the last tag (4.0.0) misses a third of the icons
    default_branch: true,
    tag_prefix: "",
    drop_root_style: false,
    collect,
};

/// The directory of a style and the variant it becomes.
const STYLES: &[(&str, &str)] = &[
    ("materialicons", "filled"),
    ("materialiconsoutlined", "outlined"),
    ("materialiconsround", "rounded"),
    ("materialiconssharp", "sharp"),
    ("materialiconstwotone", "two_tone"),
];

/// `src/<category>/<name>/<style>/24px.svg` is `<category>_<name>::<variant>`.
/// A few icons only exist in 20px, which is used then.
fn collect(root: &Path) -> Result<Vec<Raw>, String> {
    let mut raws = Vec::new();
    for category in dirs(&root.join("src"))? {
        let category_name = file_name(&category);
        for icon in dirs(&category)? {
            let name = format!("{category_name}_{}", file_name(&icon));
            for (style, variant) in STYLES {
                let dir = icon.join(style);
                let svg = ["24px.svg", "20px.svg"]
                    .into_iter()
                    .map(|size| dir.join(size))
                    .find(|path| path.is_file());
                if let Some(path) = svg {
                    raws.push(Raw::new(&name, variant, path));
                }
            }
        }
    }
    Ok(raws)
}

fn dirs(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut dirs: Vec<_> = fs::read_dir(dir)
        .map_err(|e| format!("{dir:?}: {e}"))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    Ok(dirs)
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}
