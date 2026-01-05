#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DrawerProps {
    pub trigger_id: String,
    pub label: String,
    pub children: Element,
    pub submit_action: Option<String>,
    /// All standard HTML div attributes (id, style, etc.)
    #[props(extends = div, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Drawer(props: DrawerProps) -> Element {
    if let Some(action) = &props.submit_action {
        rsx!(
            form { action: "{action}", method: "post",
                div {..props.attributes,
                    div {
                        class: "side-drawer flex flex-col",
                        id: props.trigger_id,
                        div { class: "drawer__overlay", tabindex: "-1" }
                        div { class: "drawer__panel",
                            header { class: "drawer__header",
                                h4 { class: "drawer__title", "{props.label}" }
                                a { href: "#", class: "drawer__close", "X" }
                            }
                            {props.children}
                        }
                    }
                }
            }
        )
    } else {
        rsx!(
            div {..props.attributes,
                div { class: "side-drawer flex flex-col", id: props.trigger_id,
                    div { class: "drawer__overlay", tabindex: "-1" }
                    div { class: "drawer__panel",
                        header { class: "drawer__header",
                            h4 { class: "drawer__title", "{props.label}" }
                            a { href: "#", class: "drawer__close", "X" }
                        }
                        {props.children}
                    }
                }
            }
        )
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DrawerFooterProps {
    pub children: Element,
    /// All standard HTML div attributes (id, style, onclick, etc.)
    #[props(extends = div, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn DrawerFooter(props: DrawerFooterProps) -> Element {
    rsx!(
        div { class: "drawer__footer", ..props.attributes, {props.children} }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct DrawerBodyProps {
    pub children: Element,
    /// All standard HTML div attributes (id, style, onclick, etc.)
    #[props(extends = div, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn DrawerBody(props: DrawerBodyProps) -> Element {
    rsx!(
        div { class: "drawer__body", ..props.attributes, {props.children} }
    )
}
