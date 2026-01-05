#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum RangeColor {
    #[default]
    Default,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}

impl Display for RangeColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RangeColor::Default => write!(f, ""),
            RangeColor::Primary => write!(f, "range-primary"),
            RangeColor::Secondary => write!(f, "range-secondary"),
            RangeColor::Accent => write!(f, "range-accent"),
            RangeColor::Info => write!(f, "range-info"),
            RangeColor::Success => write!(f, "range-success"),
            RangeColor::Warning => write!(f, "range-warning"),
            RangeColor::Error => write!(f, "range-error"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum RangeSize {
    #[default]
    Default,
    ExtraSmall,
    Small,
    Medium,
    Large,
}

impl Display for RangeSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RangeSize::Default => write!(f, ""),
            RangeSize::ExtraSmall => write!(f, "range-xs"),
            RangeSize::Small => write!(f, "range-sm"),
            RangeSize::Medium => write!(f, "range-md"),
            RangeSize::Large => write!(f, "range-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct RangeProps {
    /// Label text displayed above the range
    pub label: String,
    #[props(default)]
    pub range_color: RangeColor,
    #[props(default)]
    pub range_size: RangeSize,
    /// All standard HTML input attributes (name, min, max, value, step, oninput, etc.)
    #[props(extends = input, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Range(props: RangeProps) -> Element {
    let color = props.range_color.to_string();
    let size = props.range_size.to_string();

    rsx!(
        label { class: "flex flex-col gap-1",
            span { "{props.label}" }
            input {
                r#type: "range",
                class: "range {color} {size}",
                ..props.attributes,
            }
        }
    )
}
