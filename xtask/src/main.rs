//! Maintenance tasks of the workspace: `cargo xtask <task>`.

mod bump;
mod generate;
mod icon;
mod sources;
mod upstream;

use std::{env, path::PathBuf, process::ExitCode};

const USAGE: &str = "\
Usage:
  cargo xtask update <library> [--version <x.y.z>] [--summary <file>] [--no-bump]
  cargo xtask generate <library> --source <checkout> --version <x.y.z> [--summary <file>] [--no-bump]

update     fetches a release of the upstream repository (the latest one by default)
           and regenerates the icon crate of the library.
           Projects that do not tag releases (material) are used at the tip of their default
           branch, and the commit is the version.
generate   does the same from a checkout you already have, without network access.

  --version  the upstream release, recorded in the crate's Cargo.toml
  --summary  writes a markdown summary of the changes (used for the pull request)
  --no-bump  do not bump the versions of the crates

The crate gets a patch release if icons were added or changed and a minor release
(breaking in 0.x) if icons were removed. Then `pictogram` follows with a minor release.";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}\n\n{USAGE}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let task = args.next().ok_or("missing task")?;
    if task != "update" && task != "generate" {
        return Err(format!("unknown task '{task}'"));
    }
    let source = sources::find(&args.next().ok_or("missing library")?)?;

    let (mut checkout, mut version, mut summary, mut allow_bump) = (None, None, None, true);
    while let Some(flag) = args.next() {
        if flag == "--no-bump" {
            allow_bump = false;
            continue;
        }
        let value = args.next().ok_or_else(|| format!("{flag} needs a value"))?;
        match flag.as_str() {
            "--source" => checkout = Some(PathBuf::from(value)),
            "--version" => version = Some(value),
            "--summary" => summary = Some(PathBuf::from(value)),
            other => return Err(format!("unknown flag '{other}'")),
        }
    }

    let (checkout, version) = if task == "generate" {
        (
            checkout.ok_or("--source is required")?,
            version.ok_or("--version is required")?,
        )
    } else {
        let dest = env::temp_dir().join(format!("pictogram-xtask-{}", source.name));
        if source.default_branch {
            if version.is_some() {
                return Err(format!(
                    "{} has no releases, --version is not possible",
                    source.name
                ));
            }
            eprintln!("fetching {}", source.repo);
            upstream::fetch(source, None, &dest)?;
            let commit = upstream::head(&dest)?;
            (dest, commit)
        } else {
            let tag = upstream::resolve(source, version.as_deref())?;
            let version = upstream::version_of(&tag).ok_or("the tag is not a release")?;
            eprintln!("fetching {} {tag}", source.repo);
            upstream::fetch(source, Some(&tag), &dest)?;
            (dest, version)
        }
    };

    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .canonicalize()
        .map_err(|e| e.to_string())?;
    generate::run(
        source,
        &workspace,
        &checkout,
        &version,
        summary.as_deref(),
        allow_bump,
    )
}
