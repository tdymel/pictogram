use std::path::Path;

use super::{Raw, Source, svgs};

pub const SOURCE: Source = Source {
    name: "phosphor",
    crate_dir: "pictogram-icons-phosphor",
    repo: "https://github.com/phosphor-icons/core",
    sparse: &["/raw/"],
    recolor: false,
    default_branch: false,
    tag_prefix: "",
    drop_root_style: false,
    collect,
};

/// The weights, as the directories of `raw/`. Every weight but `regular` adds its name to the
/// file name.
const WEIGHTS: &[&str] = &["thin", "light", "regular", "bold", "fill", "duotone"];

/// `raw/regular/house.svg` is `house::regular`, `raw/bold/house-bold.svg` is `house::bold`.
fn collect(root: &Path) -> Result<Vec<Raw>, String> {
    let mut raws = Vec::new();
    for weight in WEIGHTS {
        for (stem, path) in svgs(&root.join("raw").join(weight))? {
            raws.push(Raw::new(name_of(&stem, weight), weight, path));
        }
    }
    Ok(raws)
}

fn name_of<'a>(stem: &'a str, weight: &str) -> &'a str {
    if weight == "regular" {
        return stem;
    }
    stem.strip_suffix(weight)
        .and_then(|name| name.strip_suffix('-'))
        .unwrap_or(stem)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_weight_is_taken_from_the_name() {
        assert_eq!(name_of("house", "regular"), "house");
        assert_eq!(name_of("house-bold", "bold"), "house");
        assert_eq!(name_of("arrow-up-right-thin", "thin"), "arrow-up-right");
        assert_eq!(name_of("house-duotone", "duotone"), "house");
    }
}
