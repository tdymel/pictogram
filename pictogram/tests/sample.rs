#[test]
fn sample() {
    let svg_str = pictogram::svg!(pictogram::bootstrap::_0_circle::filled).to_string();
    let result = pictogram::Svg::new(&svg_str).unwrap();
    println!("{:#?}", result.to_string());
    assert!(false);
}
