//! Version bumps of the crates and the version requirements on them.

use std::{fs, path::Path};

/// The `pictogram` crate re-exports every icon crate, so it follows their breaking changes.
pub const FACADE: &str = "pictogram";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bump {
    /// Icons were added or changed.
    Patch,
    /// Icons were removed. In `0.x` a minor release is the breaking one.
    Minor,
}

pub fn next(version: &str, bump: Bump) -> Result<String, String> {
    let parts: Vec<u64> = version
        .split('.')
        .map(|p| {
            p.parse()
                .map_err(|_| format!("'{version}' is not a version like 1.2.3"))
        })
        .collect::<Result<_, _>>()?;
    let [major, minor, patch] = parts[..] else {
        return Err(format!("'{version}' is not a version like 1.2.3"));
    };
    Ok(match bump {
        Bump::Patch => format!("{major}.{minor}.{}", patch + 1),
        Bump::Minor => format!("{major}.{}.0", minor + 1),
    })
}

/// The `version = "..."` of the `[package]` section.
pub fn read_package_version(manifest: &str) -> Result<String, String> {
    package_version_line(manifest)
        .and_then(|i| manifest.lines().nth(i)?.split('"').nth(1))
        .map(str::to_owned)
        .ok_or_else(|| "Cargo.toml has no version in [package]".to_owned())
}

pub fn write_package_version(manifest: &str, version: &str) -> Result<String, String> {
    let line = package_version_line(manifest).ok_or("Cargo.toml has no version in [package]")?;
    Ok(replace_line(manifest, line, |_| {
        format!("version = \"{version}\"")
    }))
}

fn package_version_line(manifest: &str) -> Option<usize> {
    let mut in_package = false;
    for (i, line) in manifest.lines().enumerate() {
        if line.starts_with('[') {
            in_package = line.trim() == "[package]";
        } else if in_package && line.starts_with("version") && line.contains('=') {
            return Some(i);
        }
    }
    None
}

/// Sets the required version of a dependency: `name = { path = "..", version = "0.3.0" }`.
pub fn update_pin(manifest: &str, dependency: &str, version: &str) -> String {
    let prefix = format!("{dependency} = ");
    let mut out = String::with_capacity(manifest.len());
    for line in manifest.split_inclusive('\n') {
        match (line.starts_with(&prefix), line.find("version = \"")) {
            (true, Some(at)) => {
                let start = at + "version = \"".len();
                let end = start + line[start..].find('"').unwrap_or(0);
                out.push_str(&line[..start]);
                out.push_str(version);
                out.push_str(&line[end..]);
            }
            _ => out.push_str(line),
        }
    }
    out
}

fn replace_line(text: &str, index: usize, new: impl Fn(&str) -> String) -> String {
    let mut out = String::with_capacity(text.len());
    for (i, line) in text.split_inclusive('\n').enumerate() {
        if i == index {
            out.push_str(&new(line.trim_end_matches('\n')));
            if line.ends_with('\n') {
                out.push('\n');
            }
        } else {
            out.push_str(line);
        }
    }
    out
}

/// Applies `edit` to the manifest of every crate of the workspace, except `xtask`.
pub fn edit_manifests(workspace: &Path, edit: impl Fn(&str, &str) -> String) -> Result<(), String> {
    let mut dirs: Vec<_> = fs::read_dir(workspace)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.join("Cargo.toml").is_file() && !p.ends_with("xtask"))
        .collect();
    dirs.sort();
    for dir in dirs {
        let path = dir.join("Cargo.toml");
        let manifest = fs::read_to_string(&path).map_err(|e| format!("{path:?}: {e}"))?;
        let name = dir.file_name().and_then(|n| n.to_str()).unwrap_or_default();
        let edited = edit(name, &manifest);
        if edited != manifest {
            fs::write(&path, edited).map_err(|e| format!("{path:?}: {e}"))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST: &str = "[package]\nname = \"a\"\nversion = \"0.3.0\"\n\n[dependencies]\nfoo = { path = \"../foo\", version = \"0.3.0\", optional = true }\nbar = { path = \"../bar\", version = \"0.3.0\" }\nversion = \"9\"\n";

    #[test]
    fn bumps() {
        assert_eq!(next("0.3.0", Bump::Patch).unwrap(), "0.3.1");
        assert_eq!(next("0.3.7", Bump::Minor).unwrap(), "0.4.0");
        assert_eq!(next("1.2.3", Bump::Patch).unwrap(), "1.2.4");
        assert!(next("1.2", Bump::Patch).is_err());
        assert!(next("a.b.c", Bump::Patch).is_err());
    }

    #[test]
    fn package_version() {
        assert_eq!(read_package_version(MANIFEST).unwrap(), "0.3.0");
        let updated = write_package_version(MANIFEST, "0.3.1").unwrap();
        assert_eq!(read_package_version(&updated).unwrap(), "0.3.1");
        // only the package, not the dependencies
        assert_eq!(updated.matches("version = \"0.3.0\"").count(), 2);
        assert!(updated.contains("version = \"9\""));
    }

    #[test]
    fn pins() {
        let updated = update_pin(MANIFEST, "foo", "0.4.0");
        assert!(
            updated.contains("foo = { path = \"../foo\", version = \"0.4.0\", optional = true }")
        );
        assert!(updated.contains("bar = { path = \"../bar\", version = \"0.3.0\" }"));
        assert_eq!(update_pin(MANIFEST, "baz", "1.0.0"), MANIFEST);
    }
}
