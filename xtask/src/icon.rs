//! Turns an upstream `.svg` file into the data of one generated icon.

use std::ops::Range;

use pictogram_core::Svg;

/// The parts of an icon, ready to be written into the generated crate.
pub struct Parsed {
    pub view_box: String,
    pub attrs: String,
    pub body: String,
}

/// Rust keywords, which can only be used as an identifier in their raw form (`r#box`).
const KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "else", "enum", "extern", "false", "fn", "for", "if",
    "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "static",
    "struct", "trait", "true", "type", "unsafe", "use", "where", "while", "async", "await", "dyn",
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized",
    "virtual", "yield", "try", "gen",
];

/// Keywords that cannot be raw identifiers at all.
const UNUSABLE: &[&str] = &["crate", "self", "Self", "super", "_"];

/// `arrow-up` -> `arrow_up`, `1-2` -> `_1_2`, `box` -> `r#box`.
/// Used for the module of an icon and for its variants.
pub fn ident(name: &str) -> Result<String, String> {
    let name = name.replace('-', "_");
    if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(format!("'{name}' cannot be turned into an identifier"));
    }
    if UNUSABLE.contains(&name.as_str()) {
        return Err(format!(
            "'{name}' is a reserved word that cannot be an identifier"
        ));
    }
    Ok(if name.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{name}")
    } else if KEYWORDS.contains(&name.as_str()) {
        format!("r#{name}")
    } else {
        name
    })
}

/// `recolor` replaces hard coded colors by `currentColor`, so monochrome icons follow the text color.
pub fn parse(
    label: &str,
    src: &str,
    recolor: bool,
    warnings: &mut Vec<String>,
) -> Result<Parsed, String> {
    let doc = roxmltree::Document::parse(src).map_err(|e| format!("{label}: invalid xml: {e}"))?;
    if doc.root_element().tag_name().name() != "svg" {
        return Err(format!("{label}: the root element is not <svg>"));
    }

    let edits = color_edits(&doc);
    let mut src = src.to_owned();
    if recolor {
        // Back to front, so the ranges stay valid
        for (range, replacement) in edits.into_iter().rev() {
            src.replace_range(range, &replacement);
        }
    } else if !edits.is_empty() {
        warnings.push(format!(
            "`{label}` hard codes a color; it will not follow the text color"
        ));
    }

    // The very same parser that is used for custom icons.
    let src: &'static str = Box::leak(src.into_boxed_str());
    let svg = std::panic::catch_unwind(|| Svg::new(src))
        .map_err(|_| format!("{label}: could not be parsed (is there a viewBox?)"))?;

    let attrs = svg
        .attributes()
        .map(|(name, value)| format!("{name}=\"{}\"", value.replace('"', "&quot;")))
        .collect::<Vec<_>>()
        .join(" ");
    Ok(Parsed {
        view_box: svg.view_box.to_owned(),
        attrs,
        body: minify(svg.body),
    })
}

const COLOR_PROPERTIES: &[&str] = &[
    "fill",
    "stroke",
    "stop-color",
    "flood-color",
    "lighting-color",
    "color",
];

fn is_hard_coded(value: &str) -> bool {
    const NEUTRAL: &[&str] = &[
        "none",
        "currentColor",
        "inherit",
        "transparent",
        "context-fill",
        "context-stroke",
    ];
    let value = value.trim();
    !NEUTRAL.contains(&value) && !value.starts_with("url(")
}

/// Where a color is hard coded, as attribute or in `style`, and what to write instead.
fn color_edits(doc: &roxmltree::Document) -> Vec<(Range<usize>, String)> {
    let mut edits = Vec::new();
    for node in doc.descendants().filter(|n| n.is_element()) {
        for attribute in node.attributes() {
            let value = attribute.value();
            if COLOR_PROPERTIES.contains(&attribute.name()) && is_hard_coded(value) {
                edits.push((attribute.range_value(), "currentColor".to_owned()));
            } else if attribute.name() == "style" {
                let rewritten = value
                    .split(';')
                    .map(|declaration| match declaration.split_once(':') {
                        Some((property, value))
                            if COLOR_PROPERTIES.contains(&property.trim())
                                && is_hard_coded(value) =>
                        {
                            format!("{property}:currentColor")
                        }
                        _ => declaration.to_owned(),
                    })
                    .collect::<Vec<_>>()
                    .join(";");
                if rewritten != value {
                    edits.push((attribute.range_value(), rewritten));
                }
            }
        }
    }
    edits.sort_by_key(|(range, _)| range.start);
    edits
}

/// Removes the pretty printing between elements, which is not needed in the binary.
///
/// * whitespace between `>` and `<` is dropped
/// * any other whitespace run containing a line break becomes a single space
///   (an attribute may be wrapped onto the next line)
pub fn minify(body: &str) -> String {
    let body = body.trim();
    let mut out = String::with_capacity(body.len());
    let mut chars = body.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        if !c.is_whitespace() {
            out.push(c);
            continue;
        }
        let mut end = i + c.len_utf8();
        while let Some(&(j, next)) = chars.peek() {
            if !next.is_whitespace() {
                break;
            }
            end = j + next.len_utf8();
            chars.next();
        }
        let run = &body[i..end];
        let next = body[end..].chars().next();
        if out.ends_with('>') && next == Some('<') {
            continue;
        }
        if run.contains('\n') {
            out.push(' ');
        } else {
            out.push_str(run);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_plain(src: &str) -> Result<Parsed, String> {
        parse("test", src, false, &mut vec![])
    }

    #[test]
    fn identifiers() {
        assert_eq!(ident("arrow-up").unwrap(), "arrow_up");
        assert_eq!(ident("grid-2x2").unwrap(), "grid_2x2");
        assert_eq!(ident("1-2").unwrap(), "_1_2");
        assert_eq!(ident("box").unwrap(), "r#box");
        assert_eq!(ident("type").unwrap(), "r#type");
        assert!(ident("self").is_err());
        assert!(ident("a.b").is_err());
    }

    #[test]
    fn minify_between_elements() {
        assert_eq!(
            minify("\n  <path d=\"a\" />\n  <path d=\"b\" />\n"),
            "<path d=\"a\" /><path d=\"b\" />"
        );
    }

    #[test]
    fn minify_keeps_wrapped_attributes_apart() {
        assert_eq!(
            minify("<path\n    d=\"a\"\n    fill=\"none\"\n/>"),
            "<path d=\"a\" fill=\"none\" />"
        );
    }

    #[test]
    fn minify_keeps_text_and_inline_spaces() {
        assert_eq!(minify("<text>a  b</text>"), "<text>a  b</text>");
    }

    #[test]
    fn parses_an_icon() {
        let src = "<!-- c -->\n<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\">\n  <path d=\"M1\" />\n</svg>\n";
        let icon = parse_plain(src).unwrap();
        assert_eq!(icon.view_box, "0 0 24 24");
        assert_eq!(icon.attrs, "fill=\"none\" stroke=\"currentColor\"");
        assert_eq!(icon.body, "<path d=\"M1\" />");
    }

    #[test]
    fn warns_about_hard_coded_colors() {
        let mut warnings = vec![];
        parse(
            "a",
            "<svg viewBox=\"0 0 1 1\"><path fill=\"#000\"/></svg>",
            false,
            &mut warnings,
        )
        .unwrap();
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn recolors_attributes_and_styles() {
        let src = "<svg viewBox=\"0 0 1 1\" fill=\"none\"><path fill=\"#0F172A\" stroke=\"black\"/><path style=\"fill:none;stroke:#000;stroke-width:32px\"/><path fill=\"url(#a)\"/></svg>";
        let mut warnings = vec![];
        let icon = parse("a", src, true, &mut warnings).unwrap();
        assert_eq!(
            icon.body,
            "<path fill=\"currentColor\" stroke=\"currentColor\"/><path style=\"fill:none;stroke:currentColor;stroke-width:32px\"/><path fill=\"url(#a)\"/>"
        );
        assert!(warnings.is_empty());
    }

    #[test]
    fn missing_view_box_is_an_error() {
        assert!(parse_plain("<svg><g/></svg>").is_err());
    }
}
