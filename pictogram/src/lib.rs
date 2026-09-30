#![doc = include_str!("../README.md")]

pub use pictogram_core::{Svg, XMLNS};

/// Icons from lucide. Every icon is a `const` [`Svg`]: `pictogram::lucide::house::outlined`.
#[cfg(feature = "lucide")]
pub use pictogram_icons_lucide as lucide;

/// Icons from Bootstrap Icons. Every icon is a `const` [`Svg`]: `pictogram::bootstrap::<icon>::<variant>`.
#[cfg(feature = "bootstrap")]
pub use pictogram_icons_bootstrap as bootstrap;

/// Icons from Feather. Every icon is a `const` [`Svg`]: `pictogram::feather::<icon>::<variant>`.
#[cfg(feature = "feather")]
pub use pictogram_icons_feather as feather;
