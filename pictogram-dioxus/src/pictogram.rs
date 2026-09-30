use dioxus::prelude::*;
use pictogram_core::{Svg, XMLNS};

use crate::provider::ProviderAttributes;

/// Props for the [`Pictogram`] component
#[derive(PartialEq, Props, Clone)]
pub struct PictogramProps {
    pub icon: Svg,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Option<Element>,
}

/// Renders an icon.
///
/// The icon is drawn the way its author intended (lucide draws with strokes, material with fills).
/// It follows the text color, so `color: "red"` (or a class) colors it.
/// ```rust,ignore
/// rsx! {
///     Pictogram {
///         icon: pictogram::lucide::house::outlined,
///         width: "3rem",
///         height: "3rem",
///     }
/// }
/// ```
///
/// Attributes are applied in this order, a later one replaces an earlier one:
/// 1. defaults of this component (size, `fill: currentColor` if the icon has no fill of its own)
/// 2. the attributes of the icon itself (`fill`, `stroke`, ...)
/// 3. attributes of an [`IconProvider`](crate::IconProvider)
/// 4. attributes of the component
#[allow(non_snake_case)]
pub fn Pictogram(props: PictogramProps) -> Element {
    let provider: ProviderAttributes = try_use_context().unwrap_or_default();
    let attributes = attributes(&props.icon, &provider.attributes, props.attributes);

    rsx!(svg {
        view_box: props.icon.view_box,
        xmlns: XMLNS,
        dangerous_inner_html: props.icon.body,
        ..attributes,
        {props.children}
    })
}

/// Everything but `view_box` and `xmlns` of the svg element.
pub(crate) fn attributes(
    icon: &Svg,
    provider: &[Attribute],
    own: Vec<Attribute>,
) -> Vec<Attribute> {
    let mut defaults = vec![
        Attribute::new("width", "24px", None, false),
        Attribute::new("height", "24px", None, false),
        Attribute::new("aria-hidden", "true", None, false),
    ];
    if !icon.attributes().any(|(name, _)| name == "fill") {
        defaults.push(Attribute::new("fill", "currentColor", None, false));
    }
    let from_icon = icon
        .attributes()
        .map(|(name, value)| Attribute::new(name, value, None, false));

    merge(
        defaults
            .into_iter()
            .chain(from_icon)
            .chain(provider.iter().cloned())
            .chain(own),
    )
}

/// Merges attributes. A later attribute replaces an earlier one with the same name.
pub(crate) fn merge(attributes: impl IntoIterator<Item = Attribute>) -> Vec<Attribute> {
    let mut merged: Vec<Attribute> = Vec::new();
    for attribute in attributes {
        match merged
            .iter_mut()
            .find(|a| a.name == attribute.name && a.namespace == attribute.namespace)
        {
            Some(existing) => *existing = attribute,
            None => merged.push(attribute),
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::core::AttributeValue;

    fn names(attributes: &[Attribute]) -> Vec<&'static str> {
        attributes.iter().map(|a| a.name).collect()
    }

    #[test]
    fn a_later_attribute_replaces_an_earlier_one() {
        let merged = merge([
            Attribute::new("fill", "a", None, false),
            Attribute::new("stroke", "b", None, false),
            Attribute::new("fill", "c", None, false),
        ]);
        assert_eq!(names(&merged), ["fill", "stroke"]);
        assert_eq!(merged[0].value, AttributeValue::Text("c".into()));
    }

    #[test]
    fn the_namespace_is_part_of_the_identity() {
        let merged = merge([
            Attribute::new("fill", "a", None, false),
            Attribute::new("fill", "b", Some("style"), false),
        ]);
        assert_eq!(merged.len(), 2);
    }

    #[test]
    fn an_icon_with_its_own_fill_does_not_get_the_default() {
        let stroke = pictogram_core::Svg::new(
            r#"<svg viewBox="0 0 1 1" fill="none" stroke="currentColor"/>"#,
        );
        let attrs = attributes(&stroke, &[], vec![]);
        let fills: Vec<_> = attrs.iter().filter(|a| a.name == "fill").collect();
        assert_eq!(fills.len(), 1);
        assert_eq!(fills[0].value, AttributeValue::Text("none".into()));
    }

    #[test]
    fn provider_beats_the_icon_and_the_component_beats_the_provider() {
        let icon = pictogram_core::Svg::new(r#"<svg viewBox="0 0 1 1" stroke="currentColor"/>"#);
        let provider = [Attribute::new("stroke", "blue", None, false)];
        let stroke = |attrs: &[Attribute]| {
            attrs
                .iter()
                .find(|a| a.name == "stroke")
                .unwrap()
                .value
                .clone()
        };

        let attrs = attributes(&icon, &provider, vec![]);
        assert_eq!(stroke(&attrs), AttributeValue::Text("blue".into()));
        let attrs = attributes(
            &icon,
            &provider,
            vec![Attribute::new("stroke", "green", None, false)],
        );
        assert_eq!(stroke(&attrs), AttributeValue::Text("green".into()));
    }
}

/// Props for the prepared icon using the macro
#[derive(PartialEq, Props, Clone)]
pub struct PreparedIconProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Option<Element>,
}

/// Define a dedicated icon component
/// ```rust,ignore
/// define_icon!(pictogram::lucide::house::outlined);
/// define_icon!(CustomIcon, "local-path-to-custom-icon.svg");
///
/// #[component]
/// fn SomeComponent() -> Element {
///     rsx! {
///         HouseOutlined {
///           height: "3rem",
///           width: "3rem"
///         }
///         CustomIcon {
///           height: "3rem",
///           width: "3rem"
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_icon {
    (pictogram::$source:ident::$name:ident::$variant:ident) => {
        const _: () = {
            let _ = pictogram::$source::$name::$variant;
        };
        $crate::paste! {
            #[allow(non_snake_case)]
            pub fn [<$name:camel$variant:camel>](props: $crate::PreparedIconProps) -> Element {
                let $crate::PreparedIconProps {
                    children,
                    attributes,
                } = props;

                dioxus::prelude::rsx! {
                    $crate::Pictogram {
                        icon: pictogram::$source::$name::$variant,
                        attributes: attributes,
                        {children}
                    }
                }
            }
        }
    };
    ($name:ident, $path:literal) => {
        #[allow(non_snake_case)]
        pub fn $name(props: $crate::PreparedIconProps) -> Element {
            const ICON: $crate::Svg = $crate::Svg::new(include_str!($path));
            let $crate::PreparedIconProps {
                children,
                attributes,
            } = props;

            dioxus::prelude::rsx! {
                $crate::Pictogram {
                    icon: ICON,
                    attributes: attributes,
                    {children}
                }
            }
        }
    };
}
