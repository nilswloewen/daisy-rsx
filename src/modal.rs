#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    pub trigger_id: String,
    pub children: Element,
    pub submit_action: Option<String>,
    /// All standard HTML dialog attributes (id, style, etc.)
    #[props(extends = dialog, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    rsx!(
        if let Some(action) = &props.submit_action {
            form { action: "{action}", method: "post",
                dialog {
                    class: "modal",
                    id: "{props.trigger_id}",
                    popover: "auto",
                    ..props.attributes,
                    {props.children}
                }
            }
        } else {
            dialog {
                class: "modal",
                id: "{props.trigger_id}",
                popover: "auto",
                ..props.attributes,
                {props.children}
            }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalBodyProps {
    pub children: Element,
    /// All standard HTML div attributes (id, style, onclick, etc.)
    #[props(extends = div, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ModalBody(props: ModalBodyProps) -> Element {
    rsx!(
        div { class: "modal-box", ..props.attributes, {props.children} }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalActionProps {
    pub children: Element,
    /// All standard HTML div attributes (id, style, onclick, etc.)
    #[props(extends = div, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ModalAction(props: ModalActionProps) -> Element {
    rsx!(
        div { class: "modal-action", ..props.attributes, {props.children} }
    )
}
