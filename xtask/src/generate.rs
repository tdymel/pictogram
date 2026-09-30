//! Generates the source of an icon crate from a checkout of the upstream repository.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    fs,
    path::Path,
};

use crate::{
    bump::{self, Bump},
    icon,
};

/// Where an icon library lives upstream and where its crate lives here.
pub struct Source {
    pub name: &'static str,
    pub crate_dir: &'static str,
    pub repo: &'static str,
    /// Directory of the `.svg` files, relative to the upstream checkout.
    pub icons_dir: &'static str,
    /// Every icon of the source is exposed as this variant.
    pub variant: &'static str,
}

pub const LUCIDE: Source = Source {
    name: "lucide",
    crate_dir: "pictogram-icons-lucide",
    repo: "https://github.com/lucide-icons/lucide",
    icons_dir: "icons",
    variant: "outlined",
};

const VERSION_KEY: &str = "upstream-version";

pub fn run(
    source: &Source,
    workspace: &Path,
    checkout: &Path,
    version: &str,
    summary: Option<&Path>,
    allow_bump: bool,
) -> Result<(), String> {
    check_version(version)?;
    let crate_dir = workspace.join(source.crate_dir);
    let lib_path = crate_dir.join("src/lib.rs");
    let manifest_path = crate_dir.join("Cargo.toml");

    let previous_icons = fs::read_to_string(&lib_path)
        .map(|s| modules_of(&s))
        .unwrap_or_default();
    let manifest =
        fs::read_to_string(&manifest_path).map_err(|e| format!("{manifest_path:?}: {e}"))?;
    let previous_version = read_version(&manifest)?;

    // Read and convert every icon. Sorted by file name so the output is deterministic.
    let icons_dir = checkout.join(source.icons_dir);
    let mut files: Vec<_> = fs::read_dir(&icons_dir)
        .map_err(|e| format!("{icons_dir:?}: {e}"))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "svg"))
        .collect();
    files.sort();
    if files.is_empty() {
        return Err(format!("no .svg files in {icons_dir:?}"));
    }

    let mut warnings = Vec::new();
    let mut icons = BTreeMap::new();
    for path in files {
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("non utf-8 file name")?;
        let src = fs::read_to_string(&path).map_err(|e| format!("{path:?}: {e}"))?;
        let icon = icon::parse(stem, &src, &mut warnings)?;
        if let Some(other) = icons.insert(icon.module.clone(), icon) {
            return Err(format!(
                "two icons are turned into the module `{}`",
                other.module
            ));
        }
    }

    let aliases = read_aliases(&icons_dir, &icons, &mut warnings);

    let code = emit(source, version, &icons, &aliases);
    let lib_changed = fs::read_to_string(&lib_path).map_or(true, |old| old != code);
    let current: BTreeSet<_> = icons.keys().chain(aliases.keys()).cloned().collect();
    let removed = previous_icons.difference(&current).count();

    // Icons that were only added are a patch, removed icons are breaking.
    let bump = match (allow_bump && lib_changed, removed) {
        (false, _) => None,
        (true, 0) => Some(Bump::Patch),
        (true, _) => Some(Bump::Minor),
    };
    let crate_version = bump::read_package_version(&manifest)?;
    let mut new_manifest = write_version(&manifest, version)?;
    let mut versions = Vec::new();
    if let Some(kind) = bump {
        let new = bump::next(&crate_version, kind)?;
        new_manifest = bump::write_package_version(&new_manifest, &new)?;
        versions.push(format!("{} {crate_version} -> {new}", source.crate_dir));
        if kind == Bump::Minor {
            versions.extend(bump_dependents(workspace, source.crate_dir, &new)?);
        }
    }

    write_if_changed(&lib_path, &code)?;
    write_if_changed(&manifest_path, &new_manifest)?;

    let text = summarize(&Report {
        source,
        old_version: &previous_version,
        version,
        old: &previous_icons,
        new: &current,
        aliases: aliases.len(),
        versions: &versions,
        warnings: &warnings,
    });
    println!("{text}");
    if let Some(summary) = summary {
        fs::write(summary, &text).map_err(|e| format!("{summary:?}: {e}"))?;
    }
    if !lib_changed && new_manifest == manifest {
        println!("Nothing changed.");
    }
    Ok(())
}

/// A breaking release of an icon crate: the crates depending on it require the new version,
/// and `pictogram`, which re-exports it, gets a breaking release as well.
fn bump_dependents(workspace: &Path, krate: &str, version: &str) -> Result<Vec<String>, String> {
    let mut lines = Vec::new();
    bump::edit_manifests(workspace, |_, manifest| {
        bump::update_pin(manifest, krate, version)
    })?;

    let facade_path = workspace.join(bump::FACADE).join("Cargo.toml");
    let facade = fs::read_to_string(&facade_path).map_err(|e| format!("{facade_path:?}: {e}"))?;
    let old = bump::read_package_version(&facade)?;
    let new = bump::next(&old, Bump::Minor)?;
    // Only the first icon crate of a run bumps `pictogram`; the version is compared, not stacked.
    lines.push(format!("{} {old} -> {new}", bump::FACADE));
    fs::write(&facade_path, bump::write_package_version(&facade, &new)?)
        .map_err(|e| format!("{facade_path:?}: {e}"))?;
    bump::edit_manifests(workspace, |_, manifest| {
        bump::update_pin(manifest, bump::FACADE, &new)
    })?;
    Ok(lines)
}

fn check_version(version: &str) -> Result<(), String> {
    let ok = !version.is_empty()
        && version.split('.').count() == 3
        && version
            .split('.')
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()));
    ok.then_some(())
        .ok_or_else(|| format!("'{version}' is not a version like 1.2.3"))
}

fn emit(
    source: &Source,
    version: &str,
    icons: &BTreeMap<String, icon::Icon>,
    aliases: &BTreeMap<String, String>,
) -> String {
    let mut out = String::with_capacity(icons.len() * 700);
    let _ = writeln!(
        out,
        "// @generated by `cargo xtask {}` from {} {version}. Do not edit.",
        source.name, source.repo
    );
    out.push_str("#![doc = include_str!(\"../README.md\")]\n#![no_std]\n#![allow(non_upper_case_globals)]\n#![cfg_attr(rustfmt, rustfmt::skip)]\n");

    // Icons and aliases in one alphabetical list.
    let mut names: Vec<&String> = icons.keys().chain(aliases.keys()).collect();
    names.sort();
    for name in names {
        if let Some(icon) = icons.get(name) {
            let _ = write!(
                out,
                "\npub mod {module} {{\n    pub const {variant}: ::pictogram_core::Svg = ::pictogram_core::Svg {{\n        view_box: {view_box:?},\n        attrs: {attrs:?},\n        body: {body:?},\n    }};\n}}\n",
                module = icon.module,
                variant = source.variant,
                view_box = icon.view_box,
                attrs = icon.attrs,
                body = icon.body,
            );
        } else {
            let target = &aliases[name];
            let _ = write!(
                out,
                "\n#[deprecated(note = \"renamed to `{target}`\")]\npub mod {name} {{\n    pub const {variant}: ::pictogram_core::Svg = super::{target}::{variant};\n}}\n",
                variant = source.variant,
            );
        }
    }
    out
}

/// Former names of icons, from the `<icon>.json` next to every `<icon>.svg`.
/// They stay available as deprecated modules, so a rename upstream does not break anyone.
fn read_aliases(
    icons_dir: &Path,
    icons: &BTreeMap<String, icon::Icon>,
    warnings: &mut Vec<String>,
) -> BTreeMap<String, String> {
    let mut aliases: BTreeMap<String, String> = BTreeMap::new();
    for (module, icon) in icons {
        let path = icons_dir.join(format!("{}.json", icon.file_stem));
        let Ok(json) = fs::read_to_string(&path) else {
            continue;
        };
        for name in alias_names(&json) {
            let alias = match icon::module_ident(&name) {
                Ok(alias) => alias,
                Err(e) => {
                    warnings.push(format!("alias of `{module}` ignored: {e}"));
                    continue;
                }
            };
            if icons.contains_key(&alias) {
                continue; // the name is a real icon again
            }
            if let Some(other) = aliases.insert(alias.clone(), module.clone())
                && &other != module
            {
                warnings.push(format!(
                    "`{alias}` is an alias of `{other}` and `{module}`; kept `{other}`"
                ));
                aliases.insert(alias, other);
            }
        }
    }
    aliases
}

/// `"aliases": ["a", {"name": "b", ...}]`
fn alias_names(json: &str) -> Vec<String> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(json) else {
        return vec![];
    };
    let Some(list) = value.get("aliases").and_then(|a| a.as_array()) else {
        return vec![];
    };
    list.iter()
        .filter_map(|a| a.as_str().or_else(|| a.get("name")?.as_str()))
        .map(str::to_owned)
        .collect()
}

/// The module names of a previously generated `lib.rs`.
fn modules_of(code: &str) -> BTreeSet<String> {
    code.lines()
        .filter_map(|l| l.strip_prefix("pub mod ")?.strip_suffix(" {"))
        .map(str::to_owned)
        .collect()
}

fn read_version(manifest: &str) -> Result<String, String> {
    manifest
        .lines()
        .find_map(|l| l.strip_prefix(VERSION_KEY)?.split('"').nth(1))
        .map(str::to_owned)
        .ok_or_else(|| format!("Cargo.toml has no `{VERSION_KEY}` in [package.metadata.pictogram]"))
}

fn write_version(manifest: &str, version: &str) -> Result<String, String> {
    read_version(manifest)?;
    let mut out = String::with_capacity(manifest.len());
    for line in manifest.split_inclusive('\n') {
        if line.starts_with(VERSION_KEY) {
            let _ = writeln!(out, "{VERSION_KEY} = \"{version}\"");
        } else {
            out.push_str(line);
        }
    }
    Ok(out)
}

fn write_if_changed(path: &Path, content: &str) -> Result<bool, String> {
    if fs::read_to_string(path).is_ok_and(|old| old == content) {
        return Ok(false);
    }
    fs::write(path, content).map_err(|e| format!("{path:?}: {e}"))?;
    Ok(true)
}

/// What a run changed, for the summary of the pull request.
struct Report<'a> {
    source: &'a Source,
    old_version: &'a str,
    version: &'a str,
    old: &'a BTreeSet<String>,
    new: &'a BTreeSet<String>,
    aliases: usize,
    versions: &'a [String],
    warnings: &'a [String],
}

fn summarize(report: &Report) -> String {
    let Report {
        source,
        old_version,
        version,
        old,
        new,
        aliases,
        versions,
        warnings,
    } = *report;
    let added: Vec<_> = new.difference(old).collect();
    let removed: Vec<_> = old.difference(new).collect();
    let mut out = String::new();
    let _ = writeln!(out, "### {} {old_version} -> {version}", source.name);
    let _ = writeln!(
        out,
        "\n{} icons, {aliases} of them deprecated aliases of renamed icons (+{}, -{}).",
        new.len(),
        added.len(),
        removed.len()
    );
    list(&mut out, "Added", &added);
    if !versions.is_empty() {
        let versions: Vec<_> = versions.iter().collect();
        list(&mut out, "Versions", &versions);
    }
    if !removed.is_empty() {
        list(
            &mut out,
            "Removed (breaking for users of these icons)",
            &removed,
        );
    }
    if !warnings.is_empty() {
        let warnings: Vec<_> = warnings.iter().collect();
        list(&mut out, "Warnings", &warnings);
    }
    out
}

fn list<T: std::fmt::Display>(out: &mut String, title: &str, items: &[T]) {
    if items.is_empty() {
        return;
    }
    const MAX: usize = 60;
    let _ = write!(out, "\n**{title}:**");
    for item in items.iter().take(MAX) {
        let _ = write!(out, " `{item}`");
    }
    if items.len() > MAX {
        let _ = write!(out, " and {} more", items.len() - MAX);
    }
    out.push('\n');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_roundtrip() {
        let manifest = "[package]\nname = \"x\"\n\n[package.metadata.pictogram]\nupstream = \"u\"\nupstream-version = \"1.2.3\"\n";
        assert_eq!(read_version(manifest).unwrap(), "1.2.3");
        let updated = write_version(manifest, "2.0.0").unwrap();
        assert_eq!(read_version(&updated).unwrap(), "2.0.0");
        assert!(updated.contains("upstream = \"u\""));
    }

    #[test]
    fn versions() {
        assert!(check_version("1.7.0").is_ok());
        assert!(check_version("v1.7.0").is_err());
        assert!(check_version("1.7").is_err());
        assert!(check_version("1.7.0-beta").is_err());
    }

    #[test]
    fn reads_aliases() {
        let json = r#"{"tags": [], "aliases": [{"name": "home", "deprecated": true}, "old-home"]}"#;
        assert_eq!(alias_names(json), ["home", "old-home"]);
        assert!(alias_names(r#"{"tags": []}"#).is_empty());
        assert!(alias_names("not json").is_empty());
    }

    #[test]
    fn finds_modules() {
        let code = "pub mod a {\n    pub const x: () = ();\n}\n\npub mod r#box {\n}\n";
        assert_eq!(
            modules_of(code),
            BTreeSet::from(["a".to_owned(), "r#box".to_owned()])
        );
    }
}
