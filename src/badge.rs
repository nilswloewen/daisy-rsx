#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum BadgeStyle {
    #[default]
    None,
    Outline,
    Dash,
    Soft,
    Ghost,
}

impl Display for BadgeStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadgeStyle::None => write!(f, ""),
            BadgeStyle::Outline => write!(f, "badge-outline"),
            BadgeStyle::Dash => write!(f, "badge-dash"),
            BadgeStyle::Soft => write!(f, "badge-soft"),
            BadgeStyle::Ghost => write!(f, "badge-ghost"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum BadgeColor {
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

impl Display for BadgeColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadgeColor::Default => write!(f, ""),
            BadgeColor::Neutral => write!(f, "badge-neutral"),
            BadgeColor::Primary => write!(f, "badge-primary"),
            BadgeColor::Secondary => write!(f, "badge-secondary"),
            BadgeColor::Accent => write!(f, "badge-accent"),
            BadgeColor::Info => write!(f, "badge-info"),
            BadgeColor::Success => write!(f, "badge-success"),
            BadgeColor::Warning => write!(f, "badge-warning"),
            BadgeColor::Error => write!(f, "badge-error"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum BadgeSize {
    #[default]
    Md,
    Xs,
    Sm,
    Lg,
    Xl,
}

impl Display for BadgeSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadgeSize::Md => write!(f, "badge-md"),
            BadgeSize::Xs => write!(f, "badge-xs"),
            BadgeSize::Sm => write!(f, "badge-sm"),
            BadgeSize::Lg => write!(f, "badge-lg"),
            BadgeSize::Xl => write!(f, "badge-xl"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    pub children: Element,
    #[props(default)]
    pub badge_style: BadgeStyle,
    #[props(default)]
    pub badge_color: BadgeColor,
    #[props(default)]
    pub badge_size: BadgeSize,
    /// All standard HTML span attributes (id, style, onclick, etc.)
    #[props(extends = span, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let style = props.badge_style.to_string();
    let color = props.badge_color.to_string();
    let size = props.badge_size.to_string();

    rsx!(
        span { class: "badge {style} {color} {size}", ..props.attributes, {props.children} }
    )
}
