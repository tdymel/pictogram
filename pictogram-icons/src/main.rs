use roxmltree::Document;
use serde_json::{json, Value};
use std::{env, fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = env::args().nth(1).ok_or("please provide folder path")?;
    let dir = Path::new(&dir);

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.extension().and_then(|s| s.to_str()) != Some("svg") {
            continue;
        }

        let svg = fs::read_to_string(&path)?;
        let json = svg_to_json(&svg)?;

        fs::write(
            path.with_extension("json"),
            serde_json::to_string_pretty(&json)?,
        )?;
    }

    Ok(())
}

fn svg_to_json(svg: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let doc = Document::parse(svg)?;
    let root = doc.root_element();

    let viewbox = root.attribute("viewBox").unwrap_or("").to_string();
    let xmlns = root.tag_name().namespace().unwrap_or("").to_string();

    let range = root.range();
    let full_svg = &svg[range];

    let start = full_svg.find('>').ok_or("invalid svg")? + 1;
    let end = full_svg.rfind("</").ok_or("invalid svg")?;
    let body = full_svg[start..end].trim();

    Ok(json!({
        "viewbox": viewbox,
        "xmlns": xmlns,
        "body": body
    }))
}
