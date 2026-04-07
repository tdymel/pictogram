#[test]
fn sample() {
    let svg_str = pictogram::svg!(pictogram::tabler::a_b::outlined).to_string();
    let result = pictogram::Svg::new(&svg_str).unwrap();
    println!("{:#?}", result.to_string());
    assert!(false);
}
