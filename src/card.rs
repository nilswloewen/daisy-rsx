#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    pub children: Element,
    /// All standard HTML div attributes (id, style, onclick, etc.)
    #[props(extends = div, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx!(
        div { class: "card", ..props.attributes, {props.children} }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderProps {
    pub title: String,
    pub children: Element,
    /// All standard HTML div attributes (id, style, onclick, etc.)
    #[props(extends = div, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    rsx!(
        div { class: "card-header flex items-center", ..props.attributes,
            h3 { class: "card-title overflow-hidden", "{props.title}" }
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct CardBodyProps {
    pub children: Element,
    /// All standard HTML div attributes (id, style, onclick, etc.)
    #[props(extends = div, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CardBody(props: CardBodyProps) -> Element {
    rsx!(
        div { class: "card-body", ..props.attributes, {props.children} }
    )
}
