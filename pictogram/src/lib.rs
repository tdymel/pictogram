#![doc = include_str!("../README.md")]

pub use pictogram_core::{Icon, Library, Svg, XMLNS};

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

/// The enabled icon libraries with all of their icons, to list and search them.
///
/// Needs the `index` feature. A library is in it if its own feature is enabled.
#[cfg(feature = "index")]
pub static LIBRARIES: &[&Library] = &[
    #[cfg(feature = "bootstrap")]
    &bootstrap::LIBRARY,
    #[cfg(feature = "feather")]
    &feather::LIBRARY,
    #[cfg(feature = "font-awesome")]
    &font_awesome::LIBRARY,
    #[cfg(feature = "hero")]
    &hero::LIBRARY,
    #[cfg(feature = "iconoir")]
    &iconoir::LIBRARY,
    #[cfg(feature = "ion")]
    &ion::LIBRARY,
    #[cfg(feature = "lobe")]
    &lobe::LIBRARY,
    #[cfg(feature = "lucide")]
    &lucide::LIBRARY,
    #[cfg(feature = "material")]
    &material::LIBRARY,
    #[cfg(feature = "oct")]
    &oct::LIBRARY,
    #[cfg(feature = "phosphor")]
    &phosphor::LIBRARY,
    #[cfg(feature = "simple")]
    &simple::LIBRARY,
    #[cfg(feature = "tabler")]
    &tabler::LIBRARY,
    #[cfg(feature = "vscode")]
    &vscode::LIBRARY,
];

/// The enabled library with this name (`lucide`, `font-awesome`, ...).
#[cfg(feature = "index")]
pub fn library(name: &str) -> Option<&'static Library> {
    LIBRARIES
        .iter()
        .copied()
        .find(|library| library.name == name)
}

/// The icons of all enabled libraries matching a query, see [`Icon::matches`].
#[cfg(feature = "index")]
pub fn search<'a>(query: &'a str) -> impl Iterator<Item = (&'static Library, &'static Icon)> + 'a {
    LIBRARIES.iter().copied().flat_map(move |library| {
        (library.icons.iter())
            .filter(move |icon| icon.matches(query))
            .map(move |icon| (library, icon))
    })
}
