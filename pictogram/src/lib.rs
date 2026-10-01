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

/// Icons from Font Awesome. Every icon is a `const` [`Svg`]: `pictogram::font_awesome::<icon>::<variant>`.
#[cfg(feature = "font-awesome")]
pub use pictogram_icons_font_awesome as font_awesome;

/// Icons from Heroicons. Every icon is a `const` [`Svg`]: `pictogram::hero::<icon>::<variant>`.
#[cfg(feature = "hero")]
pub use pictogram_icons_hero as hero;

/// Icons from Iconoir. Every icon is a `const` [`Svg`]: `pictogram::iconoir::<icon>::<variant>`.
#[cfg(feature = "iconoir")]
pub use pictogram_icons_iconoir as iconoir;

/// Icons from Ionicons. Every icon is a `const` [`Svg`]: `pictogram::ion::<icon>::<variant>`.
#[cfg(feature = "ion")]
pub use pictogram_icons_ion as ion;

/// Icons from Lobe Icons. Every icon is a `const` [`Svg`]: `pictogram::lobe::<icon>::<variant>`.
#[cfg(feature = "lobe")]
pub use pictogram_icons_lobe as lobe;

/// Icons from Primer Octicons. Every icon is a `const` [`Svg`]: `pictogram::oct::<icon>::<variant>`.
#[cfg(feature = "oct")]
pub use pictogram_icons_oct as oct;

/// Icons from Phosphor. Every icon is a `const` [`Svg`]: `pictogram::phosphor::<icon>::<variant>`.
#[cfg(feature = "phosphor")]
pub use pictogram_icons_phosphor as phosphor;

/// Icons from Simple Icons. Every icon is a `const` [`Svg`]: `pictogram::simple::<icon>::<variant>`.
#[cfg(feature = "simple")]
pub use pictogram_icons_simple as simple;

/// Icons from Tabler Icons. Every icon is a `const` [`Svg`]: `pictogram::tabler::<icon>::<variant>`.
#[cfg(feature = "tabler")]
pub use pictogram_icons_tabler as tabler;

/// Icons from VSCode Codicons. Every icon is a `const` [`Svg`]: `pictogram::vscode::<icon>::<variant>`.
#[cfg(feature = "vscode")]
pub use pictogram_icons_vscode as vscode;

/// Icons from Material Design Icons. Every icon is a `const` [`Svg`]: `pictogram::material::<icon>::<variant>`.
#[cfg(feature = "material")]
pub use pictogram_icons_material as material;
