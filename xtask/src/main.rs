//! Maintenance tasks of the workspace: `cargo xtask <task>`.

mod bump;
mod generate;
mod icon;

use std::{env, path::PathBuf, process::ExitCode};

const USAGE: &str = "\
Usage: cargo xtask lucide --source <checkout> --version <x.y.z> [--summary <file>] [--no-bump]

Regenerates pictogram-icons-lucide from a checkout of https://github.com/lucide-icons/lucide.
  --source   path of the upstream checkout (at the release tag)
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
    let source = match args.next().as_deref() {
        Some("lucide") => &generate::LUCIDE,
        Some(other) => return Err(format!("unknown task '{other}'")),
        None => return Err("missing task".into()),
    };

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
    let checkout = checkout.ok_or("--source is required")?;
    let version = version.ok_or("--version is required")?;

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
