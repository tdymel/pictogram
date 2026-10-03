//! The index of an icon library: every icon with its name, to list and search them.

use crate::Svg;

/// One icon of a [`Library`] together with its name.
///
/// Icons are `const`s you address by path (`lucide::arrow_up::outlined`), which cannot be
/// enumerated. The index is the way to get hold of all of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Icon {
    /// The name as the upstream project spells it, e.g. `arrow-up`.
    pub name: &'static str,
    /// The module of the icon in its crate, e.g. `arrow_up` (`r#box` for a keyword).
    pub module: &'static str,
    /// The variant, which is the name of the `const` in the module, e.g. `outlined`.
    pub variant: &'static str,
    pub svg: Svg,
}

impl Icon {
    /// Whether the icon matches a search query.
    ///
    /// A query is split into words at whitespace, `-` and `_`. Every word has to be part of the
    /// name or the variant, ignoring ASCII case. `arrow left` finds `arrow-left` and `left-arrow`,
    /// `solid` finds the solid variants. An empty query matches every icon.
    pub fn matches(&self, query: &str) -> bool {
        words(query).all(|word| contains(self.name, word) || contains(self.variant, word))
    }
}

/// An icon library: where it comes from and all of its icons.
///
/// Every icon crate has one as `LIBRARY`, available with its `index` feature.
#[derive(Debug, Clone, Copy)]
pub struct Library {
    /// The short name, e.g. `font-awesome`. It is the name of the feature of `pictogram`.
    pub name: &'static str,
    /// The name of the project, e.g. `Font Awesome`.
    pub title: &'static str,
    /// The license of the icons as an SPDX identifier, e.g. `CC-BY-4.0`.
    pub license: &'static str,
    /// The repository of the upstream project.
    pub repository: &'static str,
    /// The version (or commit) of the upstream project the icons are from.
    pub upstream_version: &'static str,
    /// The variants the icons come in, sorted. Not every icon has every variant.
    pub variants: &'static [&'static str],
    /// Every icon, sorted by name and variant. Deprecated aliases of renamed icons are not in it.
    pub icons: &'static [Icon],
}

impl Library {
    /// The icons matching a query, see [`Icon::matches`].
    pub fn search<'a>(&'a self, query: &'a str) -> impl Iterator<Item = &'a Icon> + 'a {
        self.icons.iter().filter(move |icon| icon.matches(query))
    }

    /// The icons of one variant.
    pub fn variant<'a>(&'a self, variant: &'a str) -> impl Iterator<Item = &'a Icon> + 'a {
        self.icons
            .iter()
            .filter(move |icon| icon.variant == variant)
    }

    /// The icon with this name and variant, e.g. `("arrow-up", "outlined")`.
    pub fn get(&self, name: &str, variant: &str) -> Option<&Icon> {
        self.icons
            .iter()
            .find(|icon| icon.name == name && icon.variant == variant)
    }
}

fn words(query: &str) -> impl Iterator<Item = &str> {
    query
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .filter(|word| !word.is_empty())
}

/// Whether `word` is part of `text`, ignoring ASCII case and treating `_` like `-`.
fn contains(text: &str, word: &str) -> bool {
    let (text, word) = (text.as_bytes(), word.as_bytes());
    let same = |a: u8, b: u8| a.eq_ignore_ascii_case(&b) || (is_dash(a) && is_dash(b));
    text.windows(word.len())
        .any(|w| w.iter().zip(word).all(|(&a, &b)| same(a, b)))
}

fn is_dash(c: u8) -> bool {
    c == b'-' || c == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn icon(name: &'static str, variant: &'static str) -> Icon {
        Icon {
            name,
            module: name,
            variant,
            svg: Svg {
                view_box: "0 0 1 1",
                attrs: "",
                body: "",
            },
        }
    }

    #[test]
    fn matches_words_in_any_order() {
        let i = icon("arrow-left", "outlined");
        assert!(i.matches(""));
        assert!(i.matches("arrow"));
        assert!(i.matches("ARROW left"));
        assert!(i.matches("left-arrow"));
        assert!(i.matches("arrow_left"));
        assert!(i.matches("outl"));
        assert!(i.matches("left outlined"));
        assert!(!i.matches("right"));
        assert!(!i.matches("arrow solid"));
    }

    #[test]
    fn library_lookups() {
        static ICONS: &[Icon] = &[icon("a", "solid"), icon("a", "regular"), icon("b", "solid")];
        let lib = Library {
            name: "x",
            title: "X",
            license: "MIT",
            repository: "",
            upstream_version: "1",
            variants: &["regular", "solid"],
            icons: ICONS,
        };
        assert_eq!(lib.search("a").count(), 2);
        assert_eq!(lib.variant("solid").count(), 2);
        assert!(lib.get("b", "solid").is_some());
        assert!(lib.get("b", "regular").is_none());
    }
}
