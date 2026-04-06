use crate::view_box::ViewBox;

/// The emitted SVG by the macro!
/// Right now only xmlns and viewBox are extracted.
/// The body is just extracted as a whole.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Svg<'a> {
    pub xmlns: &'a str,
    pub view_box: ViewBox,
    pub body: &'a str,
}

impl<'a> std::fmt::Display for Svg<'a> {
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

#[derive(Debug)]
pub struct SvgParseError(pub &'static str);

impl<'a> Svg<'a> {
    /// Use this method for your custom icons
    /// ```rust,ignore
    /// let svg_content = include_str!("/path/to/your/icon.svg");
    /// let svg = pictogram::Svg::new(&svg_content);
    /// ```
    pub fn new(svg_content: &'a str) -> Result<Self, SvgParseError> {
        let xmlns = extract_attr(svg_content, "xmlns")?;
        let (min_x, min_y, width, height) = parse_view_box(extract_attr(svg_content, "viewBox")?)?;
        let body = extract_body(svg_content)?;

        Ok(Self {
            xmlns,
            view_box: ViewBox {
                min_x,
                min_y,
                width,
                height,
            },
            body,
        })
    }
}

fn extract_attr<'a>(input: &'a str, attr: &str) -> Result<&'a str, SvgParseError> {
    let needle = format!(r#"{attr}=""#);
    let start = input
        .find(&needle)
        .ok_or(SvgParseError("attribute not found"))?
        + needle.len();

    let rest = &input[start..];
    let end = rest
        .find('"')
        .ok_or(SvgParseError("attribute not terminated"))?;
    Ok(&rest[..end])
}

fn extract_body(input: &str) -> Result<&str, SvgParseError> {
    let start = input.find('>').ok_or(SvgParseError("missing >"))? + 1;
    let end = input
        .rfind("</svg>")
        .ok_or(SvgParseError("missing </svg>"))?;
    Ok(&input[start..end])
}

fn parse_view_box(input: &str) -> Result<(f32, f32, f32, f32), SvgParseError> {
    let mut parts = input.split_whitespace();

    let a = parts
        .next()
        .ok_or(SvgParseError("missing viewBox[0]"))?
        .parse()
        .map_err(|_| SvgParseError("invalid float"))?;
    let b = parts
        .next()
        .ok_or(SvgParseError("missing viewBox[1]"))?
        .parse()
        .map_err(|_| SvgParseError("invalid float"))?;
    let c = parts
        .next()
        .ok_or(SvgParseError("missing viewBox[2]"))?
        .parse()
        .map_err(|_| SvgParseError("invalid float"))?;
    let d = parts
        .next()
        .ok_or(SvgParseError("missing viewBox[3]"))?
        .parse()
        .map_err(|_| SvgParseError("invalid float"))?;

    Ok((a, b, c, d))
}
