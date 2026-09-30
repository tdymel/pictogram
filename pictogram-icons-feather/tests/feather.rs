use pictogram_core::Svg;
use pictogram_icons_feather as lib;

/// Icons are plain constants, so they can be assigned at compile time.
const ICON: Svg = lib::heart::outlined;
static ICONS: &[Svg] = &[ICON];

#[test]
fn an_icon_is_data() {
    assert!(!ICON.view_box.is_empty());
    assert!(!ICON.body.is_empty());
    assert_eq!(ICONS.len(), 1);
    assert!(
        ICON.to_string()
            .starts_with("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"")
    );
}
