use dioxus::prelude::*;
use pictogram::Svg;

/// Icon component Props
#[derive(PartialEq, Props, Clone)]
pub struct IconProps {
    /// The icon shape to use.
    pub icon: Svg,
    /// The height of the `<svg>` element. Defaults to 20. Pass None to omit.
    #[props(default = Some(20))]
    pub height: Option<u32>,
    /// The width of the `<svg>` element. Defaults to 20. Pass None to omit.
    #[props(default = Some(20))]
    pub width: Option<u32>,
    /// The color to use for filling the icon. Defaults to "currentColor".
    #[props(default = "currentColor".to_string())]
    pub fill: String,
    /// An class for the `<svg>` element.
    #[props(default = "".to_string())]
    pub class: String,
    /// The style of the `<svg>` element.
    pub style: Option<String>,
}

/// Icon component which generates SVG elements
#[allow(non_snake_case)]
pub fn Icon(props: IconProps) -> Element {
    rsx!(svg {
        class: "{props.class}",
        style: props.style,
        height: props.height.map(|height| height.to_string()),
        width: props.width.map(|width| width.to_string()),
        view_box: "{props.icon.view_box.to_string()}",
        xmlns: "{props.icon.xmlns}",
        dangerous_inner_html: "{props.icon.body}"
    })
}
