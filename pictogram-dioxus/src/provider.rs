use dioxus::prelude::*;

/// Props to decouple the provider and icon itself
#[derive(PartialEq, Props, Clone)]
pub struct DefaultProps {
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub fill: String,
    pub style: Option<String>,
    #[props(extends = SvgAttributes)]
    pub attributes: Vec<Attribute>,
}

impl Default for DefaultProps {
    fn default() -> Self {
        Self {
            height: Some(24),
            width: Some(24),
            fill: "currentColor".to_string(),
            style: Default::default(),
            attributes: Default::default(),
        }
    }
}

/// Props for the IconProvider component
#[derive(PartialEq, Props, Clone)]
pub struct IconProviderProps {
    #[props(default = Some(24))]
    pub height: Option<u32>,
    #[props(default = Some(24))]
    pub width: Option<u32>,
    #[props(default = "currentColor".to_string())]
    pub fill: String,
    pub style: Option<String>,
    #[props(extends = SvgAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Option<Element>,
}

/// Provide default attributes for the icon component
/// ```rust,ignore
/// IconProvider {
///     height: 48,
///     width: 48,
///     fill: "blue",
///     Icon {
///         icon: pictogram::svg!(pictogram::material::image_crop_free::outlined),
///     }
/// }
/// ```
#[allow(non_snake_case)]
pub fn IconProvider(props: IconProviderProps) -> Element {
    use_context_provider(|| DefaultProps {
        height: props.height,
        width: props.width,
        fill: props.fill,
        style: props.style,
        attributes: props.attributes,
    });
    rsx! { {props.children} }
}
