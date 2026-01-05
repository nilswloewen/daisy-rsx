#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    #[default]
    None,
    Top,
    Bottom,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::None => write!(f, ""),
            Direction::Top => write!(f, "dropdown-top"),
            Direction::Bottom => write!(f, "dropdown-bottom"),
            Direction::Left => write!(f, "dropdown-left"),
            Direction::Right => write!(f, "dropdown-right"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropDownProps {
    pub children: Element,
    pub button_text: String,
    #[props(default)]
    pub carat: bool,
    #[props(default)]
    pub direction: Direction,
    pub prefix_image_src: Option<String>,
    pub suffix_image_src: Option<String>,
    /// All standard HTML div attributes (id, style, onclick, etc.)
    #[props(extends = div, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn DropDown(props: DropDownProps) -> Element {
    let direction = props.direction.to_string();

    rsx!(
        div { class: "dropdown {direction}", ..props.attributes,
            label {
                tabindex: "0",
                class: "btn btn-default btn-sm m-1 w-full flex flex-nowrap justify-between",
                "aria-haspopup": "true",
                if let Some(img_src) = props.prefix_image_src {
                    img { src: "{img_src}", class: "mr-2", width: "16" }
                }
                span { class: "truncate", "{props.button_text}" }
                if let Some(img_src) = props.suffix_image_src {
                    img { src: "{img_src}", class: "ml-2", width: "12" }
                } else if props.carat {
                    div { class: "dropdown-caret" }
                }
            }
            ul {
                tabindex: "0",
                class: "dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52 {direction}",
                {props.children}
            }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct DropDownLinkProps {
    pub children: Element,
    /// All standard HTML anchor attributes (href, target, rel, onclick, etc.)
    #[props(extends = a, extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn DropDownLink(props: DropDownLinkProps) -> Element {
    rsx!(
        li {
            a { class: "dropdown-item", ..props.attributes, {props.children} }
        }
    )
}
