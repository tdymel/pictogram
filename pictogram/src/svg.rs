use crate::view_box::ViewBox;

#[derive(Debug, Clone, Copy)]
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
