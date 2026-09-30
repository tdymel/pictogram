use pictogram_core::Svg;
use pictogram_icons_lucide as lucide;

/// Icons are plain constants, so they can be assigned at compile time.
const HOME: Svg = lucide::house::outlined;
static NAVIGATION: &[Svg] = &[lucide::house::outlined, lucide::arrow_up::outlined];

#[test]
fn an_icon_keeps_what_it_needs_to_be_drawn_correctly() {
    assert_eq!(HOME.view_box, "0 0 24 24");
    // Lucide draws with strokes. Without these the icon would be a filled blob.
    let attrs: Vec<_> = HOME.attributes().collect();
    assert!(attrs.contains(&("fill", "none")), "{attrs:?}");
    assert!(attrs.contains(&("stroke", "currentColor")), "{attrs:?}");
    assert!(attrs.contains(&("stroke-width", "2")), "{attrs:?}");
    assert!(HOME.body.starts_with("<path"));
    assert!(!HOME.body.contains('\n'), "the body is minified");
    assert_eq!(NAVIGATION.len(), 2);
}

#[test]
fn keyword_names_are_raw_identifiers() {
    assert!(lucide::r#box::outlined.body.starts_with("<path"));
    assert!(lucide::r#type::outlined.body.starts_with("<path"));
    assert!(lucide::r#move::outlined.body.starts_with("<path"));
}

#[test]
fn display_is_a_complete_svg() {
    let svg = lucide::arrow_up::outlined.to_string();
    assert!(svg.starts_with(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\""
    ));
    assert!(svg.ends_with("</svg>"));
}

/// Renamed icons stay available under their old name, as a deprecation warning instead of an error.
#[test]
#[allow(deprecated)]
fn renamed_icons_keep_working() {
    assert_eq!(lucide::home::outlined, lucide::house::outlined);
}
