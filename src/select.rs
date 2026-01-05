#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SelectSize {
    #[default]
    Default,
    ExtraSmall,
    Small,
    Medium,
    Large,
}

impl Display for SelectSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectSize::Default => write!(f, ""),
            SelectSize::ExtraSmall => write!(f, "select-xs"),
            SelectSize::Small => write!(f, "select-sm"),
            SelectSize::Medium => write!(f, "select-md"),
            SelectSize::Large => write!(f, "select-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    pub children: Element,
    /// Label text displayed above the select
    pub label: String,
    #[props(default)]
    pub select_size: SelectSize,
    /// All standard HTML select attributes (name, required, disabled, multiple, onchange, etc.)
    #[props(extends = select, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    let size = props.select_size.to_string();

    rsx!(
        label { class: "flex flex-col gap-1",
            span { "{props.label}" }
            select {
                class: "select select-bordered {size}",
                ..props.attributes,
                {props.children}
            }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct OptionProps {
    pub children: Element,
    /// All standard HTML option attributes (value, selected, disabled, etc.)
    #[props(extends = option, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SelectOption(props: OptionProps) -> Element {
    rsx!(
        option {
            ..props.attributes,
            {props.children}
        }
    )
}
