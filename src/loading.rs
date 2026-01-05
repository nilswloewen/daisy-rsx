#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoadingType {
    #[default]
    Spinner,
    Dots,
    Ring,
    Ball,
    Bars,
    Infinity,
}

impl Display for LoadingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadingType::Spinner => write!(f, "loading-spinner"),
            LoadingType::Dots => write!(f, "loading-dots"),
            LoadingType::Ring => write!(f, "loading-ring"),
            LoadingType::Ball => write!(f, "loading-ball"),
            LoadingType::Bars => write!(f, "loading-bars"),
            LoadingType::Infinity => write!(f, "loading-infinity"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoadingSize {
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

impl Display for LoadingSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadingSize::ExtraSmall => write!(f, "loading-xs"),
            LoadingSize::Small => write!(f, "loading-sm"),
            LoadingSize::Medium => write!(f, "loading-md"),
            LoadingSize::Large => write!(f, "loading-lg"),
            LoadingSize::ExtraLarge => write!(f, "loading-xl"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoadingColor {
    #[default]
    Default,
    Primary,
    Secondary,
    Accent,
    Neutral,
    Info,
    Success,
    Warning,
    Error,
}

impl Display for LoadingColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadingColor::Default => write!(f, ""),
            LoadingColor::Primary => write!(f, "text-primary"),
            LoadingColor::Secondary => write!(f, "text-secondary"),
            LoadingColor::Accent => write!(f, "text-accent"),
            LoadingColor::Neutral => write!(f, "text-neutral"),
            LoadingColor::Info => write!(f, "text-info"),
            LoadingColor::Success => write!(f, "text-success"),
            LoadingColor::Warning => write!(f, "text-warning"),
            LoadingColor::Error => write!(f, "text-error"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct LoadingProps {
    #[props(default)]
    pub loading_type: LoadingType,
    #[props(default)]
    pub loading_size: LoadingSize,
    #[props(default)]
    pub loading_color: LoadingColor,
    /// All standard HTML span attributes (id, style, etc.)
    #[props(extends = span, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A loading spinner component with DaisyUI styling.
///
/// # Example
/// ```rust
/// Loading {
///     loading_type: LoadingType::Spinner,
///     loading_size: LoadingSize::Large,
///     loading_color: LoadingColor::Primary,
/// }
/// ```
#[component]
pub fn Loading(props: LoadingProps) -> Element {
    let loading_type = props.loading_type.to_string();
    let size = props.loading_size.to_string();
    let color = props.loading_color.to_string();

    rsx!(span {
        class: "loading {loading_type} {size} {color}",
        ..props.attributes
    })
}
