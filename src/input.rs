#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
    Medium,
}

impl Display for InputSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputSize::Default => write!(f, ""),
            InputSize::ExtraSmall => write!(f, "input-xs"),
            InputSize::Small => write!(f, "input-sm"),
            InputSize::Medium => write!(f, "input-md"),
            InputSize::Large => write!(f, "input-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    /// Label text displayed above the input
    pub label: String,
    /// DaisyUI size modifier
    #[props(default)]
    pub input_size: InputSize,
    /// All standard HTML input attributes (name, type, value, placeholder, oninput, etc.)
    #[props(extends = input, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A labeled input component with DaisyUI styling.
/// The label wraps the input for accessibility.
///
/// # Example
/// ```rust
/// Input {
///     label: "Email",
///     input_size: InputSize::Small,
///     name: "email",
///     r#type: "email",
///     placeholder: "you@example.com",
///     oninput: move |e| { /* handle input */ },
/// }
/// ```
#[component]
pub fn Input(props: InputProps) -> Element {
    let size_class = props.input_size.to_string();

    rsx!(
        label { class: "flex flex-col gap-1",
            span { "{props.label}" }
            input {
                class: "input input-bordered {size_class}",
                ..props.attributes
            }
        }
    )
}
