# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Daisy RSX is a Rust library providing [Dioxus](https://dioxuslabs.com/) components that implement [Daisy UI](https://daisyui.com/) (a Tailwind CSS component library). The components are primarily designed for server-side rendering.

## Commands

```bash
# Build the library
cargo build

# Run tests
cargo test --lib

# Run a single test
cargo test --lib test_button

# Run the demo app (requires Dioxus CLI: curl -sSL http://dioxus.dev/install.sh | sh)
cd demo && dx serve

# Create a release (bumps version, creates tag, pushes, triggers crates.io publish)
cargo release patch           # dry run
cargo release patch --execute # actual release
```

## Architecture

### Component Pattern

All components follow this structure:

```rust
#![allow(non_snake_case)]
use dioxus::prelude::*;

// Configuration enums with Display trait for CSS class generation
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ComponentOption {
    #[default]
    Default,
    Variant,
}

impl Display for ComponentOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentOption::Default => write!(f, "css-class-default"),
            ComponentOption::Variant => write!(f, "css-class-variant"),
        }
    }
}

// Props with #[derive(Props, Clone, PartialEq)]
#[derive(Props, Clone, PartialEq)]
pub struct ComponentProps {
    pub children: Element,
    pub class: Option<String>,      // Custom CSS classes
    pub option: Option<EnumType>,   // Configuration options
}

#[component]
pub fn Component(props: ComponentProps) -> Element {
    rsx! { /* Daisy UI markup */ }
}
```

### Adding New Components

1. Create `src/component_name.rs` following the pattern above
2. Add `pub mod component_name;` to `src/lib.rs`
3. Add `pub use component_name::{Component, ComponentOption};` to `src/lib.rs`

### Testing Pattern

Tests use SSR to verify HTML output:

```rust
#[test]
fn test_component() {
    let props = ComponentProps { /* ... */ };
    let result = dioxus_ssr::render_element(Component(props));
    assert!(result.contains("expected-class"));
}
```

## Dioxus 0.7 Notes

See `demo/AGENTS.md` for comprehensive Dioxus 0.7 API reference. Key points:

- Uses `dioxus::prelude::*` - no `cx`, `Scope`, or `use_state`
- Props must be owned types (`String`, not `&str`)
- Signals for reactive state: `use_signal(|| value)`
- Components are functions with `#[component]` attribute

## CSS Integration

The library outputs Daisy UI classes. Users need to include Tailwind CSS with the DaisyUI plugin. The README recommends [tailwind-cli-extra](https://github.com/dobicinaitis/tailwind-cli-extra) which doesn't require npm.
