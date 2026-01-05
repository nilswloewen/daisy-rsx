#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonScheme {
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

impl Display for ButtonScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonScheme::Default => write!(f, ""),
            ButtonScheme::Neutral => write!(f, "btn-neutral"),
            ButtonScheme::Primary => write!(f, "btn-primary"),
            ButtonScheme::Secondary => write!(f, "btn-secondary"),
            ButtonScheme::Accent => write!(f, "btn-accent"),
            ButtonScheme::Info => write!(f, "btn-info"),
            ButtonScheme::Success => write!(f, "btn-success"),
            ButtonScheme::Warning => write!(f, "btn-warning"),
            ButtonScheme::Error => write!(f, "btn-error"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
    Medium,
}

impl Display for ButtonSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonSize::Default => write!(f, ""),
            ButtonSize::ExtraSmall => write!(f, "btn-xs"),
            ButtonSize::Small => write!(f, "btn-sm"),
            ButtonSize::Medium => write!(f, "btn-md"),
            ButtonSize::Large => write!(f, "btn-lg"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonShape {
    #[default]
    Default,
    Circle,
    Square,
}

impl Display for ButtonShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonShape::Default => write!(f, ""),
            ButtonShape::Circle => write!(f, "btn-circle"),
            ButtonShape::Square => write!(f, "btn-square"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonStyle {
    #[default]
    Default,
    Outline,
    Dash,
    Soft,
    Ghost,
    Link,
}

impl Display for ButtonStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonStyle::Default => write!(f, ""),
            ButtonStyle::Outline => write!(f, "btn-outline"),
            ButtonStyle::Dash => write!(f, "btn-dash"),
            ButtonStyle::Soft => write!(f, "btn-soft"),
            ButtonStyle::Ghost => write!(f, "btn-ghost"),
            ButtonStyle::Link => write!(f, "btn-link"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    pub children: Element,
    #[props(default)]
    pub button_scheme: ButtonScheme,
    #[props(default)]
    pub button_size: ButtonSize,
    #[props(default)]
    pub button_shape: ButtonShape,
    #[props(default)]
    pub button_style: ButtonStyle,
    /// All standard HTML button attributes (type, disabled, onclick, etc.)
    #[props(extends = button, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A button component with DaisyUI styling.
///
/// # Example
/// ```rust
/// Button {
///     button_scheme: ButtonScheme::Primary,
///     onclick: move |_| { /* handle click */ },
///     "Click me"
/// }
/// ```
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let scheme = props.button_scheme.to_string();
    let size = props.button_size.to_string();
    let shape = props.button_shape.to_string();
    let style = props.button_style.to_string();

    rsx!(
        button { class: "btn {scheme} {size} {shape} {style}", ..props.attributes, {props.children} }
    )
}
