use std::path::Path;

use super::{Raw, Source, svgs};

pub const SOURCE: Source = Source {
    name: "lobe",
    crate_dir: "pictogram-icons-lobe",
    repo: "https://github.com/lobehub/lobe-icons",
    sparse: &["/packages/static-svg/icons/"],
    recolor: false,
    default_branch: false,
    // The repository releases many packages (`@lobehub/icons-static-png@1.2.3`, `v5.22.0`, ...).
    // The icons are the `static-svg` one.
    tag_prefix: "@lobehub/icons-static-svg@",
    // `style="flex:none;line-height:1"` on every icon
    drop_root_style: true,
    collect,
};

/// The suffixes of the file names and the variants they stand for. Longer ones first, so that
/// `openai-text-color` is not taken for the icon `openai-text`.
const VARIANTS: &[(&str, &str)] = &[
    ("-brand-color", "brand_color"),
    ("-text-color", "text_color"),
    ("-text-cn", "text_cn"),
    ("-brand", "brand"),
    ("-color", "color"),
    ("-text", "text"),
];

/// `openai.svg` is `openai::mono`, `openai-color.svg` is `openai::color`, `openai-text.svg` is the
/// wordmark `openai::text`. `brand` is the logo with the name of the product, and `cn` the
/// chinese version.
fn collect(root: &Path) -> Result<Vec<Raw>, String> {
    Ok(svgs(&root.join("packages/static-svg/icons"))?
        .into_iter()
        .map(|(stem, path)| {
            let (name, variant) = split(&stem);
            let raw = Raw::new(name, variant, path);
            // Only the `color` variants are drawn with their own colors
            if variant.ends_with("color") {
                raw.colored()
            } else {
                raw
            }
        })
        .collect())
}

fn split(stem: &str) -> (&str, &'static str) {
    VARIANTS
        .iter()
        .find_map(|(suffix, variant)| Some((stem.strip_suffix(suffix)?, *variant)))
        .unwrap_or((stem, "mono"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_the_variant_from_the_name() {
        assert_eq!(split("openai"), ("openai", "mono"));
        assert_eq!(split("openai-color"), ("openai", "color"));
        assert_eq!(split("openai-text"), ("openai", "text"));
        assert_eq!(split("ai21-brand"), ("ai21", "brand"));
        assert_eq!(split("ai21-brand-color"), ("ai21", "brand_color"));
        assert_eq!(split("civitai-text-color"), ("civitai", "text_color"));
        assert_eq!(split("baidu-text-cn"), ("baidu", "text_cn"));
    }
}
