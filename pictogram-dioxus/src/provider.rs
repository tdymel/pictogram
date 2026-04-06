use dioxus::prelude::*;

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

impl Default for IconProviderProps {
    fn default() -> Self {
        Self {
            height: Some(24),
            width: Some(24),
            fill: "currentColor".to_string(),
            style: Default::default(),
            attributes: Default::default(),
            children: Default::default(),
        }
    }
}

/// Provide default attributes for the icon component
/// ```rust,no_run
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
    use_context_provider(|| props.clone());
    rsx! { {props.children} }
}
