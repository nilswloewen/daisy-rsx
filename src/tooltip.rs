#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum ToolTipColor {
    #[default]
    Default,
    Warn,
    Info,
    Error,
    Success,
}

impl Display for ToolTipColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolTipColor::Default => write!(f, ""),
            ToolTipColor::Info => write!(f, "tooltip-info"),
            ToolTipColor::Warn => write!(f, "tooltip-warning"),
            ToolTipColor::Error => write!(f, "tooltip-error"),
            ToolTipColor::Success => write!(f, "tooltip-success"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ToolTipProps {
    pub text: String,
    pub children: Element,
    #[props(default)]
    pub tooltip_color: ToolTipColor,
    /// All standard HTML div attributes (id, style, onclick, etc.)
    #[props(extends = div, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ToolTip(props: ToolTipProps) -> Element {
    let color = props.tooltip_color.to_string();

    rsx!(
        div {
            class: "tooltip {color}",
            "data-tip": props.text,
            ..props.attributes,
            {props.children}
        }
    )
}
