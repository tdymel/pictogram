use pictogram_core::Svg;
use pictogram_icons_iconoir as lib;

/// Icons are plain constants, so they can be assigned at compile time.
const ICON: Svg = lib::home::regular;
static ICONS: &[Svg] = &[ICON, lib::planet::regular, lib::planet::solid];

#[test]
fn an_icon_is_data() {
    assert_eq!(ICON.view_box, "0 0 24 24");
    assert!(!ICON.body.is_empty());
    assert_eq!(ICONS.len(), 3);
    assert!(
        ICON.to_string()
            .starts_with("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"")
    );
}

/// The regular icons are drawn with strokes, the solid ones are filled.
#[test]
fn variants() {
    assert!(ICON.attributes().any(|a| a == ("fill", "none")));
    assert!(
        lib::planet::regular
            .body
            .contains("stroke=\"currentColor\"")
    );
    assert!(lib::planet::solid.body.contains("fill=\"currentColor\""));
    assert_ne!(lib::planet::regular.body, lib::planet::solid.body);
}

/// Names that are keywords in rust are raw identifiers.
#[test]
fn keywords() {
    let _ = lib::r#box::regular;
    let _ = lib::r#type::regular;
}
