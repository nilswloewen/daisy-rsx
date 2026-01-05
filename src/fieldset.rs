#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FieldsetProps {
    /// Legend text displayed at the top of the fieldset
    pub legend: String,
    pub children: Element,
    /// All standard HTML fieldset attributes (disabled, form, name, etc.)
    #[props(extends = fieldset, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Fieldset(props: FieldsetProps) -> Element {
    rsx!(
        fieldset { class: "fieldset", ..props.attributes,
            legend { class: "fieldset-legend", "{props.legend}" }
            {props.children}
        }
    )
}
