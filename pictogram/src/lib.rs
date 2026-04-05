mod svg;
mod view_box;

#[doc(hidden)]
pub use pictogram_macro::svg_icon as zzz__macro_use_svg_icon;

pub use pictogram_icons_material::index as material;
pub use svg::*;
pub use view_box::*;

#[macro_export]
macro_rules! svg {
    ($path:path) => {{
        use $path;
        pictogram::zzz__macro_use_svg_icon!($path)
    }};
}
