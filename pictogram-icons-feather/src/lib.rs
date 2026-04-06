#![doc = include_str!("../README.md")]

/// Exports the index of the available icons
pub mod index;

/// Exports the manifest directory for the proc-macro
pub const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
