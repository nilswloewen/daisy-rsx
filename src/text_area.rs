#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TextAreaSize {
    #[default]
    Default,
    ExtraSmall,
    Small,
    Medium,
    Large,
}

impl Display for TextAreaSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextAreaSize::Default => write!(f, ""),
            TextAreaSize::ExtraSmall => write!(f, "textarea-xs"),
            TextAreaSize::Small => write!(f, "textarea-sm"),
            TextAreaSize::Medium => write!(f, "textarea-md"),
            TextAreaSize::Large => write!(f, "textarea-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TextAreaProps {
    /// Label text displayed above the textarea
    pub label: String,
    #[props(default)]
    pub textarea_size: TextAreaSize,
    /// All standard HTML textarea attributes (name, rows, placeholder, oninput, etc.)
    #[props(extends = textarea, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn TextArea(props: TextAreaProps) -> Element {
    let size = props.textarea_size.to_string();

    rsx!(
        label { class: "flex flex-col gap-1",
            span { "{props.label}" }
            textarea { class: "textarea textarea-bordered {size}", ..props.attributes }
        }
    )
}
