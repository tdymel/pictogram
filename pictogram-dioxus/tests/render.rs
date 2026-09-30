use dioxus::prelude::*;
use pictogram_dioxus::{IconProvider, Pictogram, define_icon};

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

/// How often `name="` occurs as a whole attribute.
fn count(html: &str, name: &str) -> usize {
    html.matches(&format!(" {name}=\"")).count()
}

#[test]
fn a_stroke_icon_is_drawn_with_strokes() {
    let html = render(|| {
        rsx! { Pictogram { icon: pictogram::lucide::house::outlined } }
    });
    assert!(html.contains("viewBox=\"0 0 24 24\""), "{html}");
    assert!(html.contains("fill=\"none\""), "{html}");
    assert!(html.contains("stroke=\"currentColor\""), "{html}");
    assert!(html.contains("stroke-width=\"2\""), "{html}");
    assert!(html.contains("<path"), "{html}");
    // The default `fill: currentColor` must not win over the icon's own `fill: none`.
    assert!(!html.contains("fill=\"currentColor\""), "{html}");
    assert_eq!(count(&html, "fill"), 1, "{html}");
    assert_eq!(count(&html, "width"), 1, "{html}");
}

#[test]
fn an_icon_without_a_fill_follows_the_text_color() {
    let html = render(|| {
        const FILLED: pictogram::Svg =
            pictogram::Svg::new(r#"<svg viewBox="0 0 24 24"><circle r="4"/></svg>"#);
        rsx! { Pictogram { icon: FILLED } }
    });
    assert!(html.contains("fill=\"currentColor\""), "{html}");
}

#[test]
fn attributes_of_the_component_win() {
    let html = render(|| {
        rsx! {
            Pictogram {
                icon: pictogram::lucide::house::outlined,
                width: "3rem",
                height: "3rem",
                stroke: "red",
            }
        }
    });
    // `width`, `height` and `stroke` are css properties in dioxus. Css beats the presentation
    // attributes of the icon in the browser, so these win over `stroke="currentColor"`.
    assert!(html.contains("width:3rem;"), "{html}");
    assert!(html.contains("stroke:red;"), "{html}");
    // the rest of the icon is untouched
    assert!(html.contains("stroke=\"currentColor\""), "{html}");
    assert!(html.contains("stroke-width=\"2\""), "{html}");
}

#[test]
fn a_provider_replaces_the_icon_but_not_the_component() {
    let html = render(|| {
        rsx! {
            IconProvider {
                width: "2rem",
                height: "2rem",
                stroke: "blue",
                Pictogram { icon: pictogram::lucide::house::outlined }
                Pictogram { icon: pictogram::lucide::arrow_up::outlined, stroke: "green" }
            }
        }
    });
    assert_eq!(html.matches("width:2rem;").count(), 2, "{html}");
    assert_eq!(html.matches("stroke:blue;").count(), 1, "{html}");
    // the component wins over the provider, and only one `stroke` is left
    assert_eq!(html.matches("stroke:green;").count(), 1, "{html}");
    assert_eq!(html.matches("stroke:").count(), 2, "{html}");
}

#[test]
fn icons_can_be_composed() {
    let html = render(|| {
        rsx! {
            Pictogram {
                icon: pictogram::lucide::house::outlined,
                Pictogram { icon: pictogram::lucide::arrow_up::outlined, width: 8, height: 8 }
            }
        }
    });
    assert_eq!(html.matches("<svg").count(), 2, "{html}");
}

define_icon!(pictogram::lucide::house::outlined);
define_icon!(Circle, "circle.svg");

#[test]
fn prepared_icons() {
    let html = render(|| {
        rsx! {
            HouseOutlined { width: "1rem" }
            Circle { height: "1rem" }
        }
    });
    assert!(html.contains("stroke=\"currentColor\""), "{html}");
    assert!(html.contains("<circle"), "{html}");
    assert!(html.contains("width:1rem;"), "{html}");
    assert!(html.contains("height:1rem;"), "{html}");
}
