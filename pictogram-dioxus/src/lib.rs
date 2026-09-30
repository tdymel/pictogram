#![doc = include_str!("../README.md")]

mod pictogram;
mod provider;

pub use paste::paste;
pub use pictogram::{Pictogram, PictogramProps, PreparedIconProps};
pub use pictogram_core::Svg;
pub use provider::IconProvider;
