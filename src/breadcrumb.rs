#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct BreadcrumbItem {
    pub text: String,
    pub href: Option<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbProps {
    items: Vec<BreadcrumbItem>,
    class: Option<String>,
}

#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx!(
        div {
            class: "breadcrumbs text-sm {class}",
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