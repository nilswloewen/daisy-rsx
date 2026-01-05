#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum FileInputStyle {
    #[default]
    Default,
    Ghost,
}

impl Display for FileInputStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileInputStyle::Default => write!(f, ""),
            FileInputStyle::Ghost => write!(f, "file-input-ghost"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum FileInputColor {
    #[default]
    Default,
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}

impl Display for FileInputColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileInputColor::Default => write!(f, ""),
            FileInputColor::Neutral => write!(f, "file-input-neutral"),
            FileInputColor::Primary => write!(f, "file-input-primary"),
            FileInputColor::Secondary => write!(f, "file-input-secondary"),
            FileInputColor::Accent => write!(f, "file-input-accent"),
            FileInputColor::Info => write!(f, "file-input-info"),
            FileInputColor::Success => write!(f, "file-input-success"),
            FileInputColor::Warning => write!(f, "file-input-warning"),
            FileInputColor::Error => write!(f, "file-input-error"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum FileInputSize {
    #[default]
    Default,
    ExtraSmall,
    Small,
    Medium,
    Large,
}

impl Display for FileInputSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileInputSize::Default => write!(f, ""),
            FileInputSize::ExtraSmall => write!(f, "file-input-xs"),
            FileInputSize::Small => write!(f, "file-input-sm"),
            FileInputSize::Medium => write!(f, "file-input-md"),
            FileInputSize::Large => write!(f, "file-input-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct FileInputProps {
    /// Label text displayed above the file input
    pub label: String,
    #[props(default)]
    pub file_input_style: FileInputStyle,
    #[props(default)]
    pub file_input_color: FileInputColor,
    #[props(default)]
    pub file_input_size: FileInputSize,
    /// All standard HTML input attributes (name, accept, multiple, etc.)
    #[props(extends = input, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn FileInput(props: FileInputProps) -> Element {
    let style = props.file_input_style.to_string();
    let color = props.file_input_color.to_string();
    let size = props.file_input_size.to_string();

    rsx!(
        label { class: "flex flex-col gap-1",
            span { "{props.label}" }
            input {
                r#type: "file",
                class: "file-input {style} {color} {size}",
                ..props.attributes,
            }
        }
    )
}
