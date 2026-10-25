use std::{fs, path::Path};

use super::{Raw, Source, svgs};

pub const SOURCE: Source = Source {
    name: "fluent-emoji",
    title: "Fluent Emoji",
    license: "MIT",
    crate_dir: "pictogram-icons-fluent-emoji",
    repo: "https://github.com/microsoft/fluentui-emoji",
    // The `Color` and `3D` styles and the skin tones are left out: the colored emoji alone are
    // 40 MB of svg.
    sparse: &[
        "/assets/*/Flat/",
        "/assets/*/High Contrast/",
        "/assets/*/Default/Flat/",
        "/assets/*/Default/High Contrast/",
    ],
    recolor: false,
    // There are no releases
    default_branch: true,
    tag_prefix: "",
    drop_root_style: false,
    collect,
};

/// `assets/Thumbs up/Flat/thumbs_up_flat.svg` is `thumbs_up::flat`, and
/// `High Contrast/thumbs_up_high_contrast.svg` is `thumbs_up::high_contrast`.
/// Emoji with skin tones have them in folders (`Default/Flat/thumbs_up_flat_default.svg`),
/// of which only `Default` (yellow) is used.
fn collect(root: &Path) -> Result<Vec<Raw>, String> {
    let assets = root.join("assets");
    let mut folders: Vec<_> = fs::read_dir(&assets)
        .map_err(|e| format!("{assets:?}: {e}"))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_dir())
        .collect();
    folders.sort();
    let mut raws = Vec::new();
    for folder in folders {
        let base = if folder.join("Default").is_dir() {
            folder.join("Default")
        } else {
            folder.clone()
        };
        for (dir, variant) in [("Flat", "flat"), ("High Contrast", "high_contrast")] {
            let files = svgs(&base.join(dir))?;
            let [(stem, path)] = &files[..] else {
                return Err(format!("{folder:?}: expected one svg in {dir}"));
            };
            let name =
                name_of(stem, variant).ok_or_else(|| format!("{path:?}: unexpected name"))?;
            let raw = Raw::new(name, variant, path.clone());
            // The flat emoji are drawn in color, the high contrast ones follow the text color.
            raws.push(if variant == "flat" {
                raw.colored()
            } else {
                raw.recolored()
            });
        }
    }
    Ok(raws)
}

/// `thumbs_up_flat_default` -> `thumbs_up`, `o_button_(blood_type)_flat` -> `o_button_blood_type`
fn name_of(stem: &str, variant: &str) -> Option<String> {
    let stem = stem.strip_suffix("_default").unwrap_or(stem);
    let name = stem.strip_suffix(variant)?.strip_suffix('_')?;
    // A few names have characters that cannot be in an identifier (`piñata`, `up!_button`).
    let name: String = name
        .replace('ñ', "n")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .collect();
    let parts: Vec<_> = name.split('_').filter(|p| !p.is_empty()).collect();
    Some(parts.join("_"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        let name = |stem, variant| name_of(stem, variant).unwrap();
        assert_eq!(name("grinning_face_flat", "flat"), "grinning_face");
        assert_eq!(
            name("thumbs_up_high_contrast_default", "high_contrast"),
            "thumbs_up"
        );
        assert_eq!(name("e-mail_flat", "flat"), "e-mail");
        assert_eq!(
            name("o_button_(blood_type)_flat", "flat"),
            "o_button_blood_type"
        );
        assert_eq!(name("up!_button_flat", "flat"), "up_button");
        assert_eq!(name("piñata_flat", "flat"), "pinata");
        assert_eq!(name_of("thumbs_up_flat", "high_contrast"), None);
    }
}
