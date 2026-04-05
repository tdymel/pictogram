use crate::view_box::ViewBox;

/// The emitted SVG by the macro!
/// Right now only xmlns and viewBox are extracted.
/// The body is just extracted as a whole.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Svg {
    pub xmlns: &'static str,
    pub view_box: ViewBox,
    pub body: &'static str,
}

impl std::fmt::Display for Svg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<svg xmlns=\"{}\" viewBox=\"{}\">",
            self.xmlns, self.view_box
        )?;
        f.write_str(self.body)?;
        f.write_str("</svg>")?;

        Ok(())
    }
}
