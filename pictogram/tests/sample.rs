use pictogram::Svg;

#[test]
fn lucide_icons_are_constants() {
    const HOME: Svg = pictogram::lucide::house::outlined;
    static ICONS: &[Svg] = &[HOME, pictogram::lucide::arrow_up::outlined];

    assert_eq!(HOME.view_box, "0 0 24 24");
    assert_eq!(ICONS.len(), 2);
    assert!(HOME.attributes().any(|a| a == ("stroke", "currentColor")));
}

#[test]
fn custom_icon() {
    const CUSTOM: Svg =
        Svg::new(r#"<svg viewBox="0 0 8 8" fill="none"><rect width="8" height="8"/></svg>"#);
    assert_eq!(CUSTOM.body, r#"<rect width="8" height="8"/>"#);
}

/// Every library is reachable through the facade.
#[test]
fn every_library_is_available() {
    let icons: [Svg; 10] = [
        pictogram::bootstrap::house::outlined,
        pictogram::feather::heart::outlined,
        pictogram::font_awesome::house::solid,
        pictogram::hero::bell::outlined,
        pictogram::ion::repeat::outlined,
        pictogram::lucide::house::outlined,
        pictogram::oct::repo::outlined,
        pictogram::simple::github::regular,
        pictogram::tabler::home::outlined,
        pictogram::vscode::account::regular,
    ];
    for icon in icons {
        assert!(!icon.view_box.is_empty());
        assert!(!icon.body.is_empty());
    }
}
