use dioxus::prelude::*;

use crate::pictogram::merge;

/// The attributes an [`IconProvider`] hands to every [`Pictogram`](crate::Pictogram) below it.
#[derive(PartialEq, Props, Clone, Default)]
pub(crate) struct ProviderAttributes {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Props for the IconProvider component
#[derive(PartialEq, Props, Clone)]
pub struct IconProviderProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Option<Element>,
}

/// Provide attributes for all icons below it.
/// They replace the attributes of the icon itself, but not the ones set on the component.
/// ```rust,ignore
/// IconProvider {
///     height: "3rem",
///     width: "3rem",
///     Pictogram {
///         icon: pictogram::lucide::house::outlined,
///     }
/// }
/// ```
#[allow(non_snake_case)]
pub fn IconProvider(props: IconProviderProps) -> Element {
    use_context_provider(|| {
        let parent: ProviderAttributes = try_consume_context().unwrap_or_default();
        ProviderAttributes {
            attributes: merge(parent.attributes.into_iter().chain(props.attributes)),
        }
    });
    rsx! { {props.children} }
}
