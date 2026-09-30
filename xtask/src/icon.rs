//! Turns an upstream `.svg` file into the data of one generated icon.

use pictogram_core::Svg;

pub struct Icon {
    /// The name of the upstream file without its extension.
    pub file_stem: String,
    /// The identifier of the module, e.g. `arrow_up` or `r#box`.
    pub module: String,
    pub view_box: String,
    pub attrs: String,
    pub body: String,
}

/// Rust keywords, which can only be used as a module name in their raw form (`r#box`).
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
pub fn module_ident(file_stem: &str) -> Result<String, String> {
    let name = file_stem.replace('-', "_");
    if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(format!("'{file_stem}' cannot be turned into a module name"));
    }
    if UNUSABLE.contains(&name.as_str()) {
        return Err(format!(
            "'{file_stem}' is a reserved word that cannot be a module name"
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

pub fn parse(file_stem: &str, src: &str, warnings: &mut Vec<String>) -> Result<Icon, String> {
    let doc =
        roxmltree::Document::parse(src).map_err(|e| format!("{file_stem}: invalid xml: {e}"))?;
    if doc.root_element().tag_name().name() != "svg" {
        return Err(format!("{file_stem}: the root element is not <svg>"));
    }
    if let Some(color) = hard_coded_color(&doc) {
        warnings.push(format!(
            "`{file_stem}` hard codes {color}; it will not follow the text color"
        ));
    }

    // The very same parser that is used for custom icons.
    let src: &'static str = Box::leak(src.to_owned().into_boxed_str());
    let svg = std::panic::catch_unwind(|| Svg::new(src))
        .map_err(|_| format!("{file_stem}: could not be parsed (is there a viewBox?)"))?;

    let attrs = svg
        .attributes()
        .map(|(name, value)| format!("{name}=\"{}\"", value.replace('"', "&quot;")))
        .collect::<Vec<_>>()
        .join(" ");
    Ok(Icon {
        file_stem: file_stem.to_owned(),
        module: module_ident(file_stem)?,
        view_box: svg.view_box.to_owned(),
        attrs,
        body: minify(svg.body),
    })
}

fn hard_coded_color(doc: &roxmltree::Document) -> Option<String> {
    const COLOR_ATTRS: &[&str] = &[
        "fill",
        "stroke",
        "stop-color",
        "flood-color",
        "lighting-color",
        "color",
    ];
    const NEUTRAL: &[&str] = &[
        "none",
        "currentColor",
        "inherit",
        "transparent",
        "context-fill",
        "context-stroke",
    ];
    doc.descendants()
        .filter(|n| n.is_element())
        .find_map(|node| {
            node.attributes()
                .find(|a| {
                    COLOR_ATTRS.contains(&a.name())
                        && !NEUTRAL.contains(&a.value().trim())
                        && !a.value().trim().starts_with("url(")
                })
                .map(|a| format!("{}=\"{}\"", a.name(), a.value()))
        })
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

    #[test]
    fn module_names() {
        assert_eq!(module_ident("arrow-up").unwrap(), "arrow_up");
        assert_eq!(module_ident("grid-2x2").unwrap(), "grid_2x2");
        assert_eq!(module_ident("1-2").unwrap(), "_1_2");
        assert_eq!(module_ident("box").unwrap(), "r#box");
        assert_eq!(module_ident("type").unwrap(), "r#type");
        assert!(module_ident("self").is_err());
        assert!(module_ident("a.b").is_err());
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
    fn parses_a_lucide_icon() {
        let src = "<!-- c -->\n<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\">\n  <path d=\"M1\" />\n</svg>\n";
        let mut warnings = vec![];
        let icon = parse("box", src, &mut warnings).unwrap();
        assert_eq!(icon.module, "r#box");
        assert_eq!(icon.view_box, "0 0 24 24");
        assert_eq!(icon.attrs, "fill=\"none\" stroke=\"currentColor\"");
        assert_eq!(icon.body, "<path d=\"M1\" />");
        assert!(warnings.is_empty());
    }

    #[test]
    fn warns_about_hard_coded_colors() {
        let mut warnings = vec![];
        parse(
            "a",
            "<svg viewBox=\"0 0 1 1\"><path fill=\"#000\"/></svg>",
            &mut warnings,
        )
        .unwrap();
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn missing_view_box_is_an_error() {
        assert!(parse("a", "<svg><g/></svg>", &mut vec![]).is_err());
    }
}
