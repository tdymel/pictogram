use std::{fs, path::Path};

use super::{Raw, Source, svgs};

pub const SOURCE: Source = Source {
    name: "lucide",
    crate_dir: "pictogram-icons-lucide",
    repo: "https://github.com/lucide-icons/lucide",
    sparse: &["icons"],
    recolor: false,
    collect,
};

fn collect(root: &Path) -> Result<Vec<Raw>, String> {
    let dir = root.join("icons");
    Ok(svgs(&dir)?
        .into_iter()
        .map(|(stem, path)| Raw {
            aliases: fs::read_to_string(dir.join(format!("{stem}.json")))
                .map(|json| alias_names(&json))
                .unwrap_or_default(),
            name: stem,
            variant: "outlined".to_owned(),
            path,
        })
        .collect())
}

/// `"aliases": ["a", {"name": "b", ...}]`
fn alias_names(json: &str) -> Vec<String> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(json) else {
        return vec![];
    };
    let Some(list) = value.get("aliases").and_then(|a| a.as_array()) else {
        return vec![];
    };
    list.iter()
        .filter_map(|a| a.as_str().or_else(|| a.get("name")?.as_str()))
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_aliases() {
        let json = r#"{"tags": [], "aliases": [{"name": "home", "deprecated": true}, "old-home"]}"#;
        assert_eq!(alias_names(json), ["home", "old-home"]);
        assert!(alias_names(r#"{"tags": []}"#).is_empty());
        assert!(alias_names("not json").is_empty());
    }
}
