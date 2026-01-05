#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct BreadcrumbItem {
    pub text: String,
    pub href: Option<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbProps {
    pub items: Vec<BreadcrumbItem>,
    /// All standard HTML div attributes (id, style, onclick, etc.)
    #[props(extends = div, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    rsx!(
        div { class: "breadcrumbs text-sm", ..props.attributes,
            ul {
                for item in props.items {
                    li {
                        if let Some(href) = &item.href {
                            a { href: "{href}", "{item.text}" }
                        } else {
                            "{item.text}"
                        }
                    }
                }
            }
        }
    )
}
