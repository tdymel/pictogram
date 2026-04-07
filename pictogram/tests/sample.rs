#[test]
fn sample() {
    let svg_str = pictogram::svg!(pictogram::font_awesome::wheat_alt::solid).to_string();
    let result = pictogram::Svg::new(&svg_str).unwrap();
    println!("{:#?}", result.to_string());
    assert!(false);
}
