#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum CheckBoxScheme {
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

impl Display for CheckBoxScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckBoxScheme::Default => write!(f, ""),
            CheckBoxScheme::Primary => write!(f, "checkbox-primary"),
            CheckBoxScheme::Secondary => write!(f, "checkbox-secondary"),
            CheckBoxScheme::Accent => write!(f, "checkbox-accent"),
            CheckBoxScheme::Info => write!(f, "checkbox-info"),
            CheckBoxScheme::Success => write!(f, "checkbox-success"),
            CheckBoxScheme::Warning => write!(f, "checkbox-warning"),
            CheckBoxScheme::Error => write!(f, "checkbox-error"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum CheckBoxSize {
    #[default]
    Default,
    ExtraSmall,
    Small,
    Medium,
    Large,
}

impl Display for CheckBoxSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckBoxSize::Default => write!(f, ""),
            CheckBoxSize::ExtraSmall => write!(f, "checkbox-xs"),
            CheckBoxSize::Small => write!(f, "checkbox-sm"),
            CheckBoxSize::Medium => write!(f, "checkbox-md"),
            CheckBoxSize::Large => write!(f, "checkbox-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CheckBoxProps {
    /// Label text displayed next to the checkbox
    pub label: String,
    #[props(default)]
    pub checkbox_size: CheckBoxSize,
    #[props(default)]
    pub checkbox_scheme: CheckBoxScheme,
    /// All standard HTML input attributes (name, value, checked, onchange, etc.)
    #[props(extends = input, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CheckBox(props: CheckBoxProps) -> Element {
    let scheme = props.checkbox_scheme.to_string();
    let size = props.checkbox_size.to_string();

    rsx!(
        label { class: "flex items-center gap-2 cursor-pointer",
            input {
                r#type: "checkbox",
                class: "checkbox {scheme} {size}",
                ..props.attributes
            }
            span { "{props.label}" }
        }
    )
}
