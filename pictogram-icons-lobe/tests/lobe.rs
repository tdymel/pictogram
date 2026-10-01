use pictogram_core::Svg;
use pictogram_icons_lobe as lib;

/// Icons are plain constants, so they can be assigned at compile time.
const ICON: Svg = lib::openai::mono;
static ICONS: &[Svg] = &[ICON, lib::openai::text, lib::claude::color];

#[test]
fn an_icon_is_data() {
    assert!(!ICON.view_box.is_empty());
    assert!(!ICON.body.is_empty());
    assert_eq!(ICONS.len(), 3);
    assert!(
        ICON.to_string()
            .starts_with("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"")
    );
}

/// The logo follows the text color, the wordmark is wider than tall, and the color
/// variants bring their own colors.
#[test]
fn variants() {
    assert!(ICON.attributes().any(|a| a == ("fill", "currentColor")));
    assert_eq!(lib::openai::text.view_box, "0 0 86 24");
    assert!(lib::claude::color.body.contains("fill=\"#"));
}

/// The `style` of the root element (`flex:none;line-height:1`) is dropped.
#[test]
fn the_root_has_no_style() {
    assert!(
        ICONS
            .iter()
            .all(|icon| icon.attributes().all(|(name, _)| name != "style"))
    );
}
