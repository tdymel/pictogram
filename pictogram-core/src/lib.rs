#![doc = include_str!("../README.md")]
#![no_std]

use core::fmt;

mod index;
pub use index::{Icon, Library};

/// The namespace of every svg element.
pub const XMLNS: &str = "http://www.w3.org/2000/svg";

/// A single icon. Every field is `'static` so an icon is a plain `const`.
///
/// Generated icon crates build this with a struct literal.
/// Custom icons can be created at compile time using [`Svg::new`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Svg {
    /// The `viewBox` of the icon, e.g. `"0 0 24 24"`.
    pub view_box: &'static str,
    /// The attributes of the root element, e.g. `fill="none" stroke="currentColor"`.
    ///
    /// Use [`Svg::attributes`] to iterate over them.
    pub attrs: &'static str,
    /// Everything inside the root element.
    pub body: &'static str,
}

impl Svg {
    /// Splits the content of an `.svg` file into its parts.
    ///
    /// This is a `const fn`: `const ICON: Svg = Svg::new(include_str!("icon.svg"));`
    /// is evaluated by the compiler and costs nothing at runtime.
    /// Leading comments and an xml prolog are skipped.
    ///
    /// # Panics
    /// Panics (a compile error in a `const`) if the input has no `<svg>` element or no `viewBox`.
    pub const fn new(src: &'static str) -> Svg {
        let b = src.as_bytes();
        let open = find_root(b);
        let (tag_end, self_closing) = find_tag_end(b, open);
        let attrs_end = if self_closing { tag_end - 1 } else { tag_end };

        let mut view_box = None;
        let mut at = open + 4;
        while let Some((name, value, next)) = next_attr(b, at, attrs_end) {
            if eq(b, name, b"viewBox") {
                view_box = Some(value);
            }
            at = next;
        }
        let Some((vb_start, vb_end)) = view_box else {
            panic!("Svg::new: the root element has no viewBox")
        };

        let body = if self_closing {
            (tag_end + 1, tag_end + 1)
        } else {
            (tag_end + 1, rfind_close(b, tag_end + 1))
        };
        Svg {
            view_box: slice(src, vb_start, vb_end),
            attrs: slice(src, open + 4, attrs_end),
            body: slice(src, body.0, body.1),
        }
    }

    /// Iterates over the attributes of the root element as `(name, value)`.
    ///
    /// Attributes that describe the element itself and are handled by the renderer
    /// (`xmlns`, `viewBox`, `width`, `height`, `class`, `id`, ...) are skipped.
    pub fn attributes(&self) -> Attributes {
        Attributes {
            rest: self.attrs,
            at: 0,
        }
    }
}

/// Writes the icon as a complete `<svg>` element.
impl fmt::Display for Svg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<svg xmlns=\"{XMLNS}\" viewBox=\"{}\"", self.view_box)?;
        for (name, value) in self.attributes() {
            write!(f, " {name}=\"{value}\"")?;
        }
        write!(f, ">{}</svg>", self.body)
    }
}

/// Iterator returned by [`Svg::attributes`].
#[derive(Debug, Clone)]
pub struct Attributes {
    rest: &'static str,
    at: usize,
}

impl Iterator for Attributes {
    type Item = (&'static str, &'static str);

    fn next(&mut self) -> Option<Self::Item> {
        let b = self.rest.as_bytes();
        while let Some((name, value, next)) = next_attr(b, self.at, b.len()) {
            self.at = next;
            let name = slice(self.rest, name.0, name.1);
            if !is_structural_attribute(name) {
                return Some((name, slice(self.rest, value.0, value.1)));
            }
        }
        None
    }
}

/// Attributes that describe the element itself, not its look (`xmlns`, `viewBox`, `width`, ...).
/// Renderers own these, so [`Svg::attributes`] skips them.
pub fn is_structural_attribute(name: &str) -> bool {
    name.starts_with("xmlns")
        || matches!(
            name,
            "viewBox"
                | "width"
                | "height"
                | "x"
                | "y"
                | "class"
                | "id"
                | "version"
                | "role"
                | "data-name"
                | "xml:space"
                | "enable-background"
        )
}

// ---------------------------------------------------------------------------
// A tiny const scanner. Every split point is an ASCII delimiter, so slices are valid utf-8.
// ---------------------------------------------------------------------------

type Span = (usize, usize);

const fn slice(s: &'static str, start: usize, end: usize) -> &'static str {
    let (_, rest) = s.as_bytes().split_at(start);
    let (part, _) = rest.split_at(end - start);
    match core::str::from_utf8(part) {
        Ok(part) => part,
        Err(_) => panic!("Svg: split inside a multi byte character"),
    }
}

const fn eq(b: &[u8], span: Span, s: &[u8]) -> bool {
    if span.1 - span.0 != s.len() {
        return false;
    }
    let mut i = 0;
    while i < s.len() {
        if b[span.0 + i] != s[i] {
            return false;
        }
        i += 1;
    }
    true
}

const fn is_ws(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\r')
}

const fn starts_with_at(b: &[u8], at: usize, s: &[u8]) -> bool {
    if at + s.len() > b.len() {
        return false;
    }
    eq(b, (at, at + s.len()), s)
}

/// Index of the `<svg` that opens the root element. Skips comments, the prolog and doctype.
const fn find_root(b: &[u8]) -> usize {
    let mut i = 0;
    while i < b.len() {
        if starts_with_at(b, i, b"<!--") {
            i += 4;
            while i < b.len() && !starts_with_at(b, i, b"-->") {
                i += 1;
            }
        } else if starts_with_at(b, i, b"<svg")
            && i + 4 < b.len()
            && (is_ws(b[i + 4]) || b[i + 4] == b'>' || b[i + 4] == b'/')
        {
            return i;
        }
        i += 1;
    }
    panic!("Svg::new: no <svg> element found")
}

/// Index of the `>` closing the root tag (ignoring `>` inside quotes) and if it is `/>`.
const fn find_tag_end(b: &[u8], open: usize) -> (usize, bool) {
    let mut i = open + 4;
    let mut quote = 0u8;
    while i < b.len() {
        let c = b[i];
        if quote != 0 {
            if c == quote {
                quote = 0;
            }
        } else if c == b'"' || c == b'\'' {
            quote = c;
        } else if c == b'>' {
            return (i, b[i - 1] == b'/');
        }
        i += 1;
    }
    panic!("Svg::new: the root element is not closed")
}

const fn rfind_close(b: &[u8], from: usize) -> usize {
    let mut i = b.len();
    while i > from {
        i -= 1;
        if starts_with_at(b, i, b"</svg") {
            return i;
        }
    }
    panic!("Svg::new: missing </svg>")
}

/// Reads the next `name="value"` (or `name='value'`) in `b[at..end]`.
/// Returns the name span, the value span and where to continue.
const fn next_attr(b: &[u8], mut at: usize, end: usize) -> Option<(Span, Span, usize)> {
    while at < end && is_ws(b[at]) {
        at += 1;
    }
    let name_start = at;
    while at < end && !is_ws(b[at]) && b[at] != b'=' {
        at += 1;
    }
    if at == name_start {
        return None;
    }
    let name_end = at;
    while at < end && (is_ws(b[at]) || b[at] == b'=') {
        at += 1;
    }
    if at >= end || (b[at] != b'"' && b[at] != b'\'') {
        return None;
    }
    let quote = b[at];
    let value_start = at + 1;
    at = value_start;
    while at < end && b[at] != quote {
        at += 1;
    }
    if at >= end {
        return None;
    }
    Some(((name_start, name_end), (value_start, at), at + 1))
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use std::{string::ToString, vec::Vec};

    const LUCIDE: Svg = Svg::new(
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
>
  <path d="m5 12 7-7 7 7" />
</svg>"#,
    );

    #[test]
    fn splits_a_file_at_compile_time() {
        assert_eq!(LUCIDE.view_box, "0 0 24 24");
        assert_eq!(LUCIDE.body.trim(), r#"<path d="m5 12 7-7 7 7" />"#);
    }

    #[test]
    fn attributes_skip_what_the_renderer_owns() {
        let attrs: Vec<_> = LUCIDE.attributes().collect();
        assert_eq!(
            attrs,
            [
                ("fill", "none"),
                ("stroke", "currentColor"),
                ("stroke-width", "2")
            ]
        );
    }

    #[test]
    fn skips_comments_and_prolog() {
        // Tabler ships a comment containing `>` in front of the svg.
        const S: Svg = Svg::new(
            "<?xml version=\"1.0\"?>\n<!--\ntags: [a > b] <svg viewBox=\"0 0 1 1\">\n-->\n<svg viewBox='0 0 16 16' fill=\"currentColor\"><g/></svg>\n",
        );
        assert_eq!(S.view_box, "0 0 16 16");
        assert_eq!(S.body, "<g/>");
        assert_eq!(
            S.attributes().collect::<Vec<_>>(),
            [("fill", "currentColor")]
        );
    }

    #[test]
    fn self_closing_root_has_an_empty_body() {
        const S: Svg = Svg::new(r#"<svg width="16" viewBox="0 0 16 16" fill="currentColor"/>"#);
        assert_eq!(S.body, "");
        assert_eq!(S.view_box, "0 0 16 16");
        assert_eq!(
            S.attributes().collect::<Vec<_>>(),
            [("fill", "currentColor")]
        );
    }

    #[test]
    fn greater_than_inside_an_attribute_value() {
        const S: Svg = Svg::new(r#"<svg data-x="a>b" viewBox="0 0 2 2"><g/></svg>"#);
        assert_eq!(S.body, "<g/>");
    }

    #[test]
    fn nested_svg_is_part_of_the_body() {
        const S: Svg = Svg::new(r#"<svg viewBox="0 0 2 2"><svg x="1"><g/></svg></svg>"#);
        assert_eq!(S.body, r#"<svg x="1"><g/></svg>"#);
    }

    #[test]
    fn display_writes_a_complete_svg() {
        assert_eq!(
            LUCIDE.to_string(),
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\">\n  <path d=\"m5 12 7-7 7 7\" />\n</svg>"
        );
    }

    #[test]
    fn multi_byte_body() {
        const S: Svg = Svg::new("<svg viewBox=\"0 0 1 1\"><title>Ünï</title></svg>");
        assert_eq!(S.body, "<title>Ünï</title>");
    }
}
