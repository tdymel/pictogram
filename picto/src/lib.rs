#[doc(hidden)]
pub use picto_macro::svg_icon as zzz__macro_use_svg_icon;

pub use picto_icons_material::index as material;

#[macro_export]
macro_rules! svg {
    ($path:path) => {{
        use $path;
        picto::zzz__macro_use_svg_icon!($path)
    }};
}
