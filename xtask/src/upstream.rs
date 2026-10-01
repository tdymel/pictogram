//! Finds and fetches releases of the upstream repositories.

use std::{fs, path::Path, process::Command};

use crate::sources::Source;

/// `refs/tags/v1.2.3` and `refs/tags/1.2.3` are both release `1.2.3`.
///
/// A repository with several packages tags `@scope/package@1.2.3`. The prefix is then given and
/// the tags of the other packages are no release.
pub fn version_of(tag: &str, prefix: &str) -> Option<String> {
    let version = if prefix.is_empty() {
        tag.strip_prefix('v').unwrap_or(tag)
    } else {
        tag.strip_prefix(prefix)?
    };
    let numbers: Option<Vec<u64>> = version.split('.').map(|p| p.parse().ok()).collect();
    (numbers?.len() == 3).then(|| version.to_owned())
}

fn number_triple(version: &str) -> (u64, u64, u64) {
    let mut parts = version.split('.').map(|p| p.parse().unwrap_or(0));
    (
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
    )
}

/// The tags of the output of `git ls-remote --tags`.
fn tags(ls_remote: &str) -> impl Iterator<Item = &str> {
    ls_remote
        .lines()
        .filter_map(|l| l.split('\t').nth(1)?.strip_prefix("refs/tags/"))
        .filter(|t| !t.ends_with("^{}"))
}

/// The tag of the highest release. Tags that are no release (`nightly`, `v1.0.0-rc1`) are ignored.
pub fn latest_tag(ls_remote: &str, prefix: &str) -> Option<String> {
    tags(ls_remote)
        .filter_map(|t| Some((number_triple(&version_of(t, prefix)?), t)))
        .max()
        .map(|(_, tag)| tag.to_owned())
}

/// The tag of a release: `1.2.3` may be tagged `1.2.3` or `v1.2.3`.
pub fn find_tag(ls_remote: &str, version: &str, prefix: &str) -> Option<String> {
    tags(ls_remote)
        .find(|t| version_of(t, prefix).as_deref() == Some(version))
        .map(str::to_owned)
}

fn git(args: &[&str], dir: Option<&Path>) -> Result<String, String> {
    let mut command = Command::new("git");
    if let Some(dir) = dir {
        command.arg("-C").arg(dir);
    }
    let output = command
        .args(args)
        .output()
        .map_err(|e| format!("git: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "git {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// The commit that is checked out, which is the version of a project without releases.
pub fn head(dest: &Path) -> Result<String, String> {
    Ok(git(&["rev-parse", "--short=12", "HEAD"], Some(dest))?
        .trim()
        .to_owned())
}

/// Resolves the tag of a release, the latest one if no version is given.
pub fn resolve(source: &Source, version: Option<&str>) -> Result<String, String> {
    let listing = git(&["ls-remote", "--tags", "--refs", source.repo], None)?;
    match version {
        Some(version) => find_tag(&listing, version, source.tag_prefix)
            .ok_or_else(|| format!("{} has no release {version}", source.repo)),
        None => latest_tag(&listing, source.tag_prefix)
            .ok_or_else(|| format!("{} has no releases", source.repo)),
    }
}

/// Checks out a tag (the default branch if there is none), with only the paths that are needed
/// and without history.
pub fn fetch(source: &Source, tag: Option<&str>, dest: &Path) -> Result<(), String> {
    let _ = fs::remove_dir_all(dest);
    let dest_str = dest.to_str().ok_or("non utf-8 path")?;
    let mut args = vec![
        "clone",
        "--quiet",
        "--depth",
        "1",
        "--filter=blob:none",
        "--sparse",
    ];
    if let Some(tag) = tag {
        args.extend(["--branch", tag]);
    }
    args.extend([source.repo, dest_str]);
    git(&args, None)?;
    let mut args = vec!["sparse-checkout", "set", "--no-cone"];
    args.extend(source.sparse);
    git(&args, Some(dest))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const LISTING: &str = "aaa\trefs/tags/v2.1.3\nbbb\trefs/tags/4.0.0\nccc\trefs/tags/v4.10.2\nddd\trefs/tags/v4.9.0\neee\trefs/tags/nightly\nfff\trefs/tags/v5.0.0-rc1\nggg\trefs/tags/4.10.1\n";

    const PACKAGES: &str = "a\trefs/tags/v5.22.0\nb\trefs/tags/@lobehub/icons-static-png@1.99.0\nc\trefs/tags/@lobehub/icons-static-svg@1.9.0\nd\trefs/tags/@lobehub/icons-static-svg@1.95.1\ne\trefs/tags/@lobehub/icons-static-svg@1.95.0\nf\trefs/tags/@lobehub/icons-static-svg@2.0.0-beta.1\n";
    const SVG: &str = "@lobehub/icons-static-svg@";

    #[test]
    fn versions_of_tags() {
        assert_eq!(version_of("v1.2.3", "").as_deref(), Some("1.2.3"));
        assert_eq!(version_of("1.2.3", "").as_deref(), Some("1.2.3"));
        assert_eq!(version_of("v1.2.3-rc1", ""), None);
        assert_eq!(version_of("nightly", ""), None);
        assert_eq!(version_of("1.2", ""), None);
    }

    #[test]
    fn the_latest_release_is_by_number_not_by_text() {
        assert_eq!(latest_tag(LISTING, "").as_deref(), Some("v4.10.2"));
    }

    #[test]
    fn a_release_may_be_tagged_with_or_without_v() {
        assert_eq!(find_tag(LISTING, "4.0.0", "").as_deref(), Some("4.0.0"));
        assert_eq!(find_tag(LISTING, "4.9.0", "").as_deref(), Some("v4.9.0"));
        assert_eq!(find_tag(LISTING, "9.9.9", ""), None);
    }

    #[test]
    fn the_tags_of_a_package_are_told_apart_by_the_prefix() {
        assert_eq!(
            version_of("@lobehub/icons-static-svg@1.95.1", SVG).as_deref(),
            Some("1.95.1")
        );
        assert_eq!(version_of("@lobehub/icons-static-png@1.99.0", SVG), None);
        assert_eq!(version_of("v5.22.0", SVG), None);
        // without a prefix the tag of another package is no release either
        assert_eq!(version_of("@lobehub/icons-static-svg@1.95.1", ""), None);
        // and the optional `v` belongs to the tags without a prefix
        assert_eq!(version_of("@lobehub/icons-static-svg@v1.95.1", SVG), None);
    }

    #[test]
    fn the_latest_release_of_a_package() {
        assert_eq!(
            latest_tag(PACKAGES, SVG).as_deref(),
            Some("@lobehub/icons-static-svg@1.95.1")
        );
        assert_eq!(
            find_tag(PACKAGES, "1.9.0", SVG).as_deref(),
            Some("@lobehub/icons-static-svg@1.9.0")
        );
        assert_eq!(find_tag(PACKAGES, "1.99.0", SVG), None);
    }
}
