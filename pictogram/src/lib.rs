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

pub use svg::*;
pub use view_box::*;

/// Macro to lookup a SVG at compile time.
/// Wraps the proc-macro for better ergonomics.
/// ```rust, no_run
/// let svg = pictogram::svg!(pictogram::material::action_123::filled);
/// println!("{}", svg);
/// ```
#[macro_export]
macro_rules! svg {
    ($path:path) => {{
        use $path;
        pictogram::zzz__macro_use_svg_icon!($path)
    }};
}
