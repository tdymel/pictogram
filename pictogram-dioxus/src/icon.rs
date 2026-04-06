use dioxus::prelude::*;

use crate::provider::IconProviderProps;

/// Props for the Icon component
#[derive(PartialEq, Props, Clone)]
pub struct IconProps {
    pub icon: pictogram::Svg,
    pub style: Option<String>,
    #[props(extends = SvgAttributes)]
    attributes: Vec<Attribute>,
    children: Option<Element>,
}

/// Convenience component
/// ```rust,no_run
/// rsx! {
///     Icon {
///         icon: pictogram::svg!(pictogram::material::action_123::filled),
///         width: 48,
///         height: 48,
///     }
/// }
/// ```
#[allow(non_snake_case)]
pub fn Icon(props: IconProps) -> Element {
    let context: IconProviderProps = try_use_context::<IconProviderProps>().unwrap_or_default();
    rsx!(svg {
        style: props.style.or(context.style),
        height: context.height.map(|v| v.to_string()),
        width: context.width.map(|v| v.to_string()),
        fill: context.fill,
        view_box: props.icon.view_box.to_string(),
        xmlns: props.icon.xmlns,
        "aria-hidden": "true",
        dangerous_inner_html: props.icon.body,
        ..context.attributes,
        ..props.attributes,
        {props.children}
    })
}
