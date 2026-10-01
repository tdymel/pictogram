use pictogram_core::Svg;
use pictogram_icons_phosphor as lib;

/// Icons are plain constants, so they can be assigned at compile time.
const ICON: Svg = lib::house::regular;
static WEIGHTS: &[Svg] = &[
    lib::house::thin,
    lib::house::light,
    ICON,
    lib::house::bold,
    lib::house::fill,
    lib::house::duotone,
];

#[test]
fn an_icon_is_data() {
    assert_eq!(ICON.view_box, "0 0 256 256");
    assert!(!ICON.body.is_empty());
    assert!(
        ICON.to_string()
            .starts_with("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"")
    );
}

/// Every icon comes in all six weights, which differ in the drawing.
#[test]
fn weights() {
    for (i, a) in WEIGHTS.iter().enumerate() {
        assert!(!a.body.is_empty());
        for b in &WEIGHTS[i + 1..] {
            assert_ne!(a.body, b.body);
        }
    }
    // the outlined weights are strokes, the filled ones follow `fill`
    assert!(lib::house::bold.body.contains("stroke-width=\"24\""));
    assert!(lib::house::thin.body.contains("stroke-width=\"8\""));
    assert!(!lib::house::fill.body.contains("stroke"));
}

/// The duotone weight draws a second, translucent layer.
#[test]
fn duotone_has_a_translucent_layer() {
    assert!(lib::house::duotone.body.contains("opacity=\"0.2\""));
    assert!(!ICON.body.contains("opacity"));
}
