use pictogram_core::Svg;
use pictogram_icons_flag as lib;

/// Icons are plain constants, so they can be assigned at compile time.
const ICON: Svg = lib::de::landscape;
static ICONS: &[Svg] = &[
    ICON,
    lib::de::square,
    lib::gb_sct::landscape,
    lib::eu::square,
];

#[test]
fn an_icon_is_data() {
    assert_eq!(ICON.view_box, "0 0 640 480");
    assert!(!ICON.body.is_empty());
    assert_eq!(ICONS.len(), 4);
    assert!(
        ICON.to_string()
            .starts_with("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"")
    );
}

/// Flags keep their colors and come in 4:3 and 1:1.
#[test]
fn variants() {
    assert!(ICON.body.contains("fill=\"#"));
    assert_eq!(lib::de::square.view_box, "0 0 512 512");
}

/// The root has no `id`, and the ids of the elements are unique across the variants.
#[test]
fn ids() {
    for icon in [lib::zw::landscape, lib::zw::square] {
        assert!(icon.attributes().all(|(name, _)| name != "id"));
    }
    assert!(
        lib::zw::landscape
            .body
            .contains("id=\"flag-zw-landscape-zw-a\"")
    );
    assert!(lib::zw::square.body.contains("id=\"flag-zw-square-zw-a\""));
}

/// Codes that are keywords in rust are raw identifiers.
#[test]
fn keywords() {
    let _ = lib::r#in::landscape;
    let _ = lib::r#do::landscape;
    let _ = lib::r#as::landscape;
}
