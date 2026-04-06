#![doc = include_str!("../README.md")]

mod icon;
mod provider;

pub use icon::{Icon, PreparedIconProps};
pub use paste::paste;
pub use provider::IconProvider;
