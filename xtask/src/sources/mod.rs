//! The icon libraries. Every library knows how to read its upstream repository.

use std::{
    fs,
    path::{Path, PathBuf},
};

mod bootstrap;
mod feather;
mod lucide;

/// Where an icon library lives upstream and how it is laid out.
pub struct Source {
    /// The name on the command line, e.g. `lucide`.
    pub name: &'static str,
    /// The directory of the crate in the workspace.
    pub crate_dir: &'static str,
    /// The git repository of the upstream project.
    pub repo: &'static str,
    /// The only paths of the repository that are needed. Some repositories are huge.
    pub sparse: &'static [&'static str],
    /// Replace hard coded colors by `currentColor`.
    pub recolor: bool,
    /// Reads the icons of a checkout of the repository.
    pub collect: fn(&Path) -> Result<Vec<Raw>, String>,
}

impl Raw {
    pub fn new(name: impl Into<String>, variant: &str, path: PathBuf) -> Raw {
        Raw {
            name: name.into(),
            variant: variant.to_owned(),
            path,
            aliases: Vec::new(),
        }
    }
}

/// One icon of one variant, as found upstream.
pub struct Raw {
    /// The name of the icon, e.g. `arrow-up`. Becomes the module.
    pub name: String,
    /// The variant, e.g. `outlined`. Becomes the constant in the module.
    pub variant: String,
    pub path: PathBuf,
    /// Former names of the icon. They stay available as deprecated modules.
    pub aliases: Vec<String>,
}

pub const ALL: &[&Source] = &[&bootstrap::SOURCE, &feather::SOURCE, &lucide::SOURCE];

pub fn find(name: &str) -> Result<&'static Source, String> {
    ALL.iter().copied().find(|s| s.name == name).ok_or_else(|| {
        let names: Vec<_> = ALL.iter().map(|s| s.name).collect();
        format!("unknown library '{name}', available: {}", names.join(", "))
    })
}

/// All `.svg` files of a directory as `(file stem, path)`, sorted.
pub fn svgs(dir: &Path) -> Result<Vec<(String, PathBuf)>, String> {
    let mut files: Vec<_> = fs::read_dir(dir)
        .map_err(|e| format!("{dir:?}: {e}"))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "svg"))
        .filter_map(|p| Some((p.file_stem()?.to_str()?.to_owned(), p)))
        .collect();
    files.sort();
    Ok(files)
}
