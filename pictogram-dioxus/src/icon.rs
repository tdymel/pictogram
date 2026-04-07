use dioxus::prelude::*;

use crate::provider::DefaultProps;

/// Props for the Icon component
#[derive(PartialEq, Props, Clone)]
pub struct IconProps {
    pub icon: pictogram::Svg<'static>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Option<Element>,
}

/// Convenience component
/// ```rust,ignore
/// rsx! {
///     Icon {
///         icon: pictogram::svg!(pictogram::material::action_123::filled),
///         width: "3rem",
///         height: "3rem",
///     }
/// }
/// ```
#[allow(non_snake_case)]
pub fn Icon(props: IconProps) -> Element {
    let context: DefaultProps = try_use_context().unwrap_or_default();
    rsx!(svg {
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
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Option<Element>,
}

/// Define a dedicated icon component
/// ```rust,ignore
/// define_icon!(pictogram::material::image_crop_free::outlined);
/// define_icon!(CustomIcon, "local-path-to-custom-icon.svg");
///
/// #[component]
/// fn SomeComponent() -> Element {
///     rsx! {
///         ImageCropFreeOutlined {
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
            pub fn [<$name:camel$variant:camel>](props: pictogram_dioxus::PreparedIconProps) -> Element {
                let pictogram_dioxus::PreparedIconProps {
                    children,
                    attributes,
                } = props;

                dioxus::prelude::rsx! {
                    pictogram_dioxus::Icon {
                        icon: pictogram::svg!(pictogram::$source::$name::$variant),
                        attributes: attributes,
                        {children}
                    }
                }
            }
        }
    };
    ($name:ident, $path:literal) => {
        #[allow(non_snake_case)]
        pub fn $name(props: pictogram_dioxus::PreparedIconProps) -> Element {
            let pictogram_dioxus::PreparedIconProps {
                children,
                attributes,
            } = props;

            dioxus::prelude::rsx! {
                pictogram_dioxus::Icon {
                    icon: pictogram::svg!($path),
                    attributes: attributes,
                    {children}
                }
            }
        }
    };
}
