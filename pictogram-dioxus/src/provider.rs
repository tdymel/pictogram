use dioxus::prelude::*;

/// Props to decouple the provider and icon itself
#[derive(PartialEq, Props, Clone)]
pub struct DefaultProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

impl Default for DefaultProps {
    fn default() -> Self {
        Self {
            attributes: vec![
                Attribute::new("height", "24px", None, false),
                Attribute::new("width", "24px", None, false),
                Attribute::new("fill", "currentColor", None, false),
            ],
        }
    }
}

/// Props for the IconProvider component
#[derive(PartialEq, Props, Clone)]
pub struct IconProviderProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Option<Element>,
}

/// Provide default attributes for the icon component
/// ```rust,ignore
/// IconProvider {
///     height: "3rem",
///     width: "3rem",
///     fill: "blue",
///     Icon {
///         icon: pictogram::svg!(pictogram::material::image_crop_free::outlined),
///     }
/// }
/// ```
#[allow(non_snake_case)]
pub fn IconProvider(props: IconProviderProps) -> Element {
    use_context_provider(|| {
        let mut default_props = DefaultProps::default();
        default_props
            .attributes
            .retain(|d| !props.attributes.iter().any(|c| c.name == d.name));
        default_props.attributes.extend(props.attributes);

        default_props
    });
    rsx! { {props.children} }
}
