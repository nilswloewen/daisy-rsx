#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum AlertColor {
    #[default]
    Default,
    Warn,
    Info,
    Error,
    Success,
}

impl Display for AlertColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertColor::Default => write!(f, ""),
            AlertColor::Info => write!(f, "alert-info"),
            AlertColor::Warn => write!(f, "alert-warning"),
            AlertColor::Error => write!(f, "alert-error"),
            AlertColor::Success => write!(f, "alert-success"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertProps {
    pub children: Element,
    #[props(default)]
    pub alert_color: AlertColor,
    /// All standard HTML div attributes (id, style, onclick, etc.)
    #[props(extends = div, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Alert(props: AlertProps) -> Element {
    let color = props.alert_color.to_string();

    rsx!(
        div { class: "alert {color}", ..props.attributes, {props.children} }
    )
}
