#![allow(non_snake_case)]
#![allow(unused_braces)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TabContainerProps {
    pub children: Element,
    /// All standard HTML div attributes (id, style, onclick, etc.)
    #[props(extends = div, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn TabContainer(props: TabContainerProps) -> Element {
    rsx!(
        div { role: "tablist", class: "tabs tabs-border", ..props.attributes, {props.children} }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct TabPanelProps {
    pub name: String,
    pub tab_name: String,
    pub children: Element,
    #[props(default)]
    pub checked: bool,
    /// All standard HTML input attributes (id, style, etc.)
    #[props(extends = input, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn TabPanel(props: TabPanelProps) -> Element {
    rsx!(
        input {
            checked: props.checked,
            r#type: "radio",
            class: "tab",
            "aria-label": props.tab_name,
            name: props.name,
            ..props.attributes,
        }
        div { role: "tabpanel", class: "tab-content", {props.children} }
    )
}
