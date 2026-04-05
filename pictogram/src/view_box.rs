/// ViewBox auf the SVG
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewBox {
    pub min_x: f32,
    pub min_y: f32,
    pub width: f32,
    pub height: f32,
}

impl std::fmt::Display for ViewBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.min_x, self.min_y, self.width, self.height
        )
    }
}
