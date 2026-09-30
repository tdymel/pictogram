#![doc = include_str!("../README.md")]

pub use pictogram_core::{Svg, XMLNS};

/// Icons from lucide. Every icon is a `const` [`Svg`]: `pictogram::lucide::house::outlined`.
#[cfg(feature = "lucide")]
pub use pictogram_icons_lucide as lucide;

/// Icons from Bootstrap Icons. Every icon is a `const` [`Svg`]: `pictogram::bootstrap::<icon>::<variant>`.
#[cfg(feature = "bootstrap")]
pub use pictogram_icons_bootstrap as bootstrap;
