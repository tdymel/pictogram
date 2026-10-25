use pictogram_core::Svg;
use pictogram_icons_fluent_emoji as lib;

/// Icons are plain constants, so they can be assigned at compile time.
const ICON: Svg = lib::thumbs_up::flat;
static ICONS: &[Svg] = &[ICON, lib::thumbs_up::high_contrast, lib::red_heart::flat];

#[test]
fn an_icon_is_data() {
    assert_eq!(ICON.view_box, "0 0 32 32");
    assert!(!ICON.body.is_empty());
    assert_eq!(ICONS.len(), 3);
    assert!(
        ICON.to_string()
            .starts_with("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"")
    );
}

/// The flat emoji keep their colors, the high contrast ones follow the text color.
#[test]
fn variants() {
    assert!(ICON.body.contains("fill=\"#"));
    let mono = lib::thumbs_up::high_contrast;
    assert!(mono.body.contains("fill=\"currentColor\""));
    assert!(!mono.body.contains("fill=\"#"));
}

/// Names that are no identifiers upstream are cleaned up.
#[test]
fn names() {
    let _ = lib::_1st_place_medal::flat;
    let _ = lib::e_mail::flat;
    let _ = lib::o_button_blood_type::flat;
    let _ = lib::pinata::flat;
}
