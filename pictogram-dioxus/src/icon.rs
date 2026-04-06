use dioxus::prelude::*;

use crate::provider::DefaultProps;

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
    let context: DefaultProps = try_use_context().unwrap_or_default();
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

/// Props for the prepared icon using the macro
#[derive(PartialEq, Props, Clone)]
pub struct PreparedIconProps {
    pub style: Option<String>,
    #[props(extends = SvgAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Option<Element>,
}

/// Define a dedicated icon component
/// ```rust,no_run
/// define_icon!(pictogram::material::image_crop_free::outlined);
///
/// #[component]
/// fn SomeComponent() -> Element {
///     rsx! {
///         ImageCropFreeOutlined {
///           height: 48,
///           width: 48
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_icon {
    (pictogram::$source:ident::$name:ident::$variant:ident) => {
        use pictogram::$source::$name::$variant;
        $crate::paste! {
            #[allow(non_snake_case)]
            pub fn [<$name:camel$variant:camel>](props: pictogram_dioxus::PreparedIconProps) -> Element {
                let pictogram_dioxus::PreparedIconProps {
                    style,
                    children,
                    attributes,
                } = props;

                dioxus::prelude::rsx! {
                    pictogram_dioxus::Icon {
                        icon: pictogram::svg!(pictogram::$source::$name::$variant),
                        style: style,
                        attributes: attributes,
                        {children}
                    }
                }
            }
        }
    };
}
