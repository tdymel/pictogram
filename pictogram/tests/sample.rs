use pictogram::Svg;

#[test]
fn lucide_icons_are_constants() {
    const HOME: Svg = pictogram::lucide::house::outlined;
    static ICONS: &[Svg] = &[HOME, pictogram::lucide::arrow_up::outlined];

    assert_eq!(HOME.view_box, "0 0 24 24");
    assert_eq!(ICONS.len(), 2);
    assert!(HOME.attributes().any(|a| a == ("stroke", "currentColor")));
}

#[test]
fn custom_icon() {
    const CUSTOM: Svg =
        Svg::new(r#"<svg viewBox="0 0 8 8" fill="none"><rect width="8" height="8"/></svg>"#);
    assert_eq!(CUSTOM.body, r#"<rect width="8" height="8"/>"#);
}

/// Every library is reachable through the facade.
#[test]
fn every_library_is_available() {
    let icons: [Svg; 14] = [
        pictogram::bootstrap::house::outlined,
        pictogram::feather::heart::outlined,
        pictogram::font_awesome::house::solid,
        pictogram::hero::bell::outlined,
        pictogram::iconoir::home::regular,
        pictogram::ion::repeat::outlined,
        pictogram::lobe::openai::mono,
        pictogram::lucide::house::outlined,
        pictogram::material::action_home::filled,
        pictogram::oct::repo::outlined,
        pictogram::phosphor::house::regular,
        pictogram::simple::github::regular,
        pictogram::tabler::home::outlined,
        pictogram::vscode::account::regular,
    ];
    for icon in icons {
        assert!(!icon.view_box.is_empty());
        assert!(!icon.body.is_empty());
    }
}

#[cfg(feature = "index")]
mod index {
    #[test]
    fn libraries_list_their_variants() {
        let names: Vec<_> = pictogram::LIBRARIES.iter().map(|l| l.name).collect();
        assert_eq!(names.len(), 14);
        assert!(names.contains(&"font-awesome"));

        let phosphor = pictogram::library("phosphor").unwrap();
        assert_eq!(
            phosphor.variants,
            ["bold", "duotone", "fill", "light", "regular", "thin"]
        );
        assert_eq!(phosphor.license, "MIT");
        assert!(pictogram::library("nope").is_none());
    }

    /// The index holds the very constants, addressed by their path.
    #[test]
    fn the_index_is_the_catalogue() {
        let lucide = pictogram::library("lucide").unwrap();
        let icon = lucide.get("arrow-up", "outlined").unwrap();
        assert_eq!(icon.module, "arrow_up");
        assert_eq!(icon.svg, pictogram::lucide::arrow_up::outlined);

        // keywords and leading digits keep their raw identifier
        let boxed = lucide.get("box", "outlined").unwrap();
        assert_eq!(boxed.module, "r#box");
        assert_eq!(boxed.svg, pictogram::lucide::r#box::outlined);
    }

    #[test]
    fn deprecated_aliases_are_not_listed() {
        let lucide = pictogram::library("lucide").unwrap();
        let mut seen = std::collections::HashSet::new();
        assert!(
            lucide
                .icons
                .iter()
                .all(|i| seen.insert((i.name, i.variant)))
        );
    }

    #[test]
    fn search_across_libraries() {
        let hits: Vec<_> = pictogram::search("arrow up").collect();
        assert!(
            hits.iter()
                .any(|(l, i)| l.name == "lucide" && i.name == "arrow-up")
        );
        assert!(hits.iter().any(|(l, _)| l.name == "tabler"));
        assert_eq!(pictogram::search("no such icon name").count(), 0);
    }

    #[test]
    fn every_icon_has_a_known_variant() {
        for library in pictogram::LIBRARIES {
            assert!(!library.icons.is_empty(), "{}", library.name);
            assert!(
                library
                    .icons
                    .iter()
                    .all(|i| library.variants.contains(&i.variant))
            );
            assert!(
                library
                    .icons
                    .windows(2)
                    .all(|w| (w[0].name, w[0].variant) < (w[1].name, w[1].variant))
            );
        }
    }
}
