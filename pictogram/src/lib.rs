#![doc = include_str!("../README.md")]

mod svg;
mod view_box;

#[doc(hidden)]
pub use pictogram_macro::svg_icon as zzz__macro_use_svg_icon;

/// Index to lookup available icons from material
#[cfg(feature = "material")]
pub use pictogram_icons_material::index as material;

/// Index to lookup available icons from bootstrap
#[cfg(feature = "bootstrap")]
pub use pictogram_icons_bootstrap::index as bootstrap;

/// Index to lookup available icons from feather
#[cfg(feature = "feather")]
pub use pictogram_icons_feather::index as feather;

/// Index to lookup available icons from font awesome
#[cfg(feature = "font-awesome")]
pub use pictogram_icons_font_awesome::index as font_awesome;

/// Index to lookup available icons from tabler
#[cfg(feature = "tabler")]
pub use pictogram_icons_tabler::index as tabler;

/// Index to lookup available icons from simple
#[cfg(feature = "simple")]
pub use pictogram_icons_simple::index as simple;

/// Index to lookup available icons from heroicons
#[cfg(feature = "hero")]
pub use pictogram_icons_hero::index as hero;

/// Index to lookup available icons from ion
#[cfg(feature = "ion")]
pub use pictogram_icons_ion::index as ion;

/// Index to lookup available icons from lucide
#[cfg(feature = "lucide")]
pub use pictogram_icons_lucide::index as lucide;

/// Index to lookup available icons from oct
#[cfg(feature = "oct")]
pub use pictogram_icons_oct::index as oct;

pub use svg::*;
pub use view_box::*;

/// Macro to lookup a SVG at compile time.
/// Wraps the proc-macro for better ergonomics.
/// ```rust, ignore
/// // Using the index
/// let svg = pictogram::svg!(pictogram::material::action_123::filled);
/// println!("{}", svg);
///
/// // Using a local path
/// let svg = pictogram::svg!("/assets/some_icon.svg");
/// println!("{}", svg);
/// ```
#[macro_export]
macro_rules! svg {
    ($path:path) => {{
        const _: () = {
            let _ = $path;
        };
        $crate::zzz__macro_use_svg_icon!($path)
    }};
    ($path:literal) => {{ $crate::Svg::new(include_str!($path)).unwrap() }};
}
