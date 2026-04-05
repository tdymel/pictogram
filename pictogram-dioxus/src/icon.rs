use dioxus::prelude::*;

/// Props for the Icon component
#[derive(PartialEq, Props, Clone)]
pub struct IconProps {
    pub icon: pictogram::Svg,
    #[props(default = Some(24))]
    pub height: Option<u32>,
    #[props(default = Some(24))]
    pub width: Option<u32>,
    #[props(default = "currentColor".to_string())]
    pub fill: String,
    pub style: Option<String>,
    #[props(extends = SvgAttributes)]
    attributes: Vec<Attribute>,
}

/// Convenience component
/// ```rust,no_run
/// rsx! {
///     Icon {
///         icon: pictogram::svg!(pictogramm::material::action_123::filled),
///         width: 48,
///         height: 48,
///     }
/// }
/// ```
#[allow(non_snake_case)]
pub fn Icon(props: IconProps) -> Element {
    rsx!(svg {
        style: props.style,
        height: props.height.map(|v| v.to_string()),
        width: props.width.map(|v| v.to_string()),
        fill: props.fill,
        view_box: props.icon.view_box.to_string(),
        xmlns: props.icon.xmlns,
        dangerous_inner_html: props.icon.body,
        ..props.attributes,
    })
}
