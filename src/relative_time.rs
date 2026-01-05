#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum RelativeTimeFormat {
    Datetime,
    #[default]
    Relative,
    Duration,
    Auto,
    Micro,
    Elapsed,
}

impl Display for RelativeTimeFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelativeTimeFormat::Datetime => write!(f, "datetime"),
            RelativeTimeFormat::Relative => write!(f, "relative"),
            RelativeTimeFormat::Duration => write!(f, "duration"),
            RelativeTimeFormat::Auto => write!(f, "auto"),
            RelativeTimeFormat::Micro => write!(f, "micro"),
            RelativeTimeFormat::Elapsed => write!(f, "elapsed"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct RelativeTimeProps {
    format: Option<RelativeTimeFormat>,
    datetime: String,
}

#[component]
pub fn RelativeTime(props: RelativeTimeProps) -> Element {
    let format = props.format.unwrap_or_default();

    rsx!(
        relative
            - time {
                datetime: props.datetime,
                format: format.to_string()
            }
    )
}
