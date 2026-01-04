#![allow(non_snake_case)]

use daisy_rsx::{
    Accordian,
    Alert, AlertColor,
    Avatar, AvatarSize, AvatarType,
    Badge, BadgeColor, BadgeSize, BadgeStyle,
    BlankSlate,
    Breadcrumb, BreadcrumbItem,
    Button, ButtonScheme, ButtonShape, ButtonSize, ButtonStyle,
    Card, CardBody, CardHeader,
    CheckBox, CheckBoxScheme,
    Drawer, DrawerBody, DrawerFooter,
    Direction, DropDown, DropDownLink,
    Fieldset,
    FileInput, FileInputColor,
    Input, InputType,
    Modal, ModalAction, ModalBody,
    Pagination,
    Range, RangeColor,
    RelativeTime, RelativeTimeFormat,
    Select, SelectOption,
    TabContainer, TabPanel,
    TextArea,
    TimeLine, TimeLineBadge, TimeLineBody,
    Timeline, TimelineDirection, TimelineEnd, TimelineItem, TimelineMiddle, TimelineStart,
    ToolTip, ToolTipColor,
};
use daisy_rsx::marketing::{
    benefits::Benefits,
    customer_logos::Customers,
    extra_footer::ExtraFooter,
    faq_accordian::{Faq, FaqText},
    features::{Feature, Features},
    hero::Hero,
    image_feature::ImageFeature,
    problem_solution::ProblemSolution,
    quad_feature::QuadFeature,
    security::Security,
    small_image_feature::SmallImageFeature,
    team::Team,
    testamonials::Testamonials,
    video_hero::VideoHero,
    webinar::WebinarHeader,
};
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/tailwind.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        script { r#type: "module", src: "https://esm.sh/@github/relative-time-element@4" }
        DaisyRsxKitchenSink {}
    }
}

// Helper component for consistent section layout
#[component]
fn Section(title: String, description: String, children: Element) -> Element {
    rsx! {
        section { class: "mb-16",
            div { class: "prose prose-lg max-w-none mb-8",
                h2 { "{title}" }
                p { class: "text-base-content/70", "{description}" }
            }
            div { class: "space-y-8",
                {children}
            }
        }
    }
}

// Helper component for individual component demos
#[component]
fn ComponentDemo(label: String, children: Element) -> Element {
    rsx! {
        div { class: "border border-base-300 rounded-lg p-6",
            h3 { class: "font-semibold text-lg mb-4 text-primary", "{label}" }
            {children}
        }
    }
}

#[component]
pub fn DaisyRsxKitchenSink() -> Element {
    rsx! {
        div { class: "min-h-screen bg-base-100",
            // Main container
            div { class: "container mx-auto px-4 py-8 max-w-6xl",

                // Page Header
                div { class: "prose prose-lg max-w-none mb-12 text-center",
                    h1 { class: "text-4xl font-bold", "Daisy RSX Kitchen Sink" }
                    p { "A comprehensive showcase of all daisy-rsx components with default theme." }
                }

                // ============================================
                // SECTION: Buttons & Actions
                // ============================================
                Section {
                    title: "Buttons & Actions".to_string(),
                    description: "Interactive button components, tooltips, and dropdowns.".to_string(),

                    ComponentDemo { label: "Button Schemes".to_string(),
                        div { class: "flex flex-wrap gap-2",
                            Button { button_scheme: ButtonScheme::Primary, "Primary" }
                            Button { button_scheme: ButtonScheme::Secondary, "Secondary" }
                            Button { button_scheme: ButtonScheme::Accent, "Accent" }
                            Button { button_scheme: ButtonScheme::Neutral, "Neutral" }
                            Button { button_scheme: ButtonScheme::Info, "Info" }
                            Button { button_scheme: ButtonScheme::Success, "Success" }
                            Button { button_scheme: ButtonScheme::Warning, "Warning" }
                            Button { button_scheme: ButtonScheme::Error, "Error" }
                        }
                    }

                    ComponentDemo { label: "Button Sizes".to_string(),
                        div { class: "flex flex-wrap gap-2 items-center",
                            Button { button_size: ButtonSize::Large, "Large" }
                            Button { button_size: ButtonSize::Medium, "Medium" }
                            Button { button_size: ButtonSize::Small, "Small" }
                            Button { button_size: ButtonSize::ExtraSmall, "Extra Small" }
                        }
                    }

                    ComponentDemo { label: "Button Styles".to_string(),
                        div { class: "flex flex-wrap gap-2",
                            Button { button_style: ButtonStyle::Outline, "Outline" }
                            Button { button_style: ButtonStyle::Dash, "Dash" }
                            Button { button_style: ButtonStyle::Soft, "Soft" }
                            Button { button_style: ButtonStyle::Ghost, "Ghost" }
                            Button { button_style: ButtonStyle::Link, "Link" }
                        }
                    }

                    ComponentDemo { label: "Button Shapes".to_string(),
                        div { class: "flex flex-wrap gap-2",
                            Button { button_shape: ButtonShape::Square, "S" }
                            Button { button_shape: ButtonShape::Circle, "C" }
                        }
                    }

                    ComponentDemo { label: "ToolTip".to_string(),
                        div { class: "flex gap-4",
                            ToolTip { text: "Default tooltip".to_string(),
                                Button { "Hover me" }
                            }
                            ToolTip { text: "Info tooltip".to_string(), alert_color: ToolTipColor::Info,
                                Button { button_scheme: ButtonScheme::Info, "Info" }
                            }
                            ToolTip { text: "Success tooltip".to_string(), alert_color: ToolTipColor::Success,
                                Button { button_scheme: ButtonScheme::Success, "Success" }
                            }
                            ToolTip { text: "Warning tooltip".to_string(), alert_color: ToolTipColor::Warn,
                                Button { button_scheme: ButtonScheme::Warning, "Warning" }
                            }
                            ToolTip { text: "Error tooltip".to_string(), alert_color: ToolTipColor::Error,
                                Button { button_scheme: ButtonScheme::Error, "Error" }
                            }
                        }
                    }

                    ComponentDemo { label: "DropDown".to_string(),
                        div { class: "flex gap-4",
                            DropDown { button_text: "Select Option".to_string(),
                                DropDownLink { href: "#".to_string(), "Option 1" }
                                DropDownLink { href: "#".to_string(), "Option 2" }
                                DropDownLink { href: "#".to_string(), "Option 3" }
                            }
                            DropDown { button_text: "With Carat".to_string(), carat: true,
                                DropDownLink { href: "#".to_string(), "Item A" }
                                DropDownLink { href: "#".to_string(), "Item B" }
                            }
                            DropDown { button_text: "Direction Top".to_string(), direction: Direction::Top,
                                DropDownLink { href: "#".to_string(), "Top Item 1" }
                                DropDownLink { href: "#".to_string(), "Top Item 2" }
                            }
                        }
                    }
                }

                // ============================================
                // SECTION: Feedback & Alerts
                // ============================================
                Section {
                    title: "Feedback & Alerts".to_string(),
                    description: "Components for displaying status, notifications, and empty states.".to_string(),

                    ComponentDemo { label: "Alert".to_string(),
                        div { class: "flex flex-col gap-2",
                            Alert { alert_color: AlertColor::Info, "This is an info alert - providing helpful information." }
                            Alert { alert_color: AlertColor::Success, "This is a success alert - operation completed!" }
                            Alert { alert_color: AlertColor::Warn, "This is a warning alert - proceed with caution." }
                            Alert { alert_color: AlertColor::Error, "This is an error alert - something went wrong." }
                        }
                    }

                    ComponentDemo { label: "Badge Colors".to_string(),
                        div { class: "flex flex-wrap gap-2",
                            Badge { "Default" }
                            Badge { badge_color: BadgeColor::Neutral, "Neutral" }
                            Badge { badge_color: BadgeColor::Primary, "Primary" }
                            Badge { badge_color: BadgeColor::Secondary, "Secondary" }
                            Badge { badge_color: BadgeColor::Accent, "Accent" }
                            Badge { badge_color: BadgeColor::Info, "Info" }
                            Badge { badge_color: BadgeColor::Success, "Success" }
                            Badge { badge_color: BadgeColor::Warning, "Warning" }
                            Badge { badge_color: BadgeColor::Error, "Error" }
                        }
                    }

                    ComponentDemo { label: "Badge Styles".to_string(),
                        div { class: "flex flex-wrap gap-2",
                            Badge { badge_style: BadgeStyle::Outline, "Outline" }
                            Badge { badge_style: BadgeStyle::Dash, "Dash" }
                            Badge { badge_style: BadgeStyle::Soft, "Soft" }
                            Badge { badge_style: BadgeStyle::Ghost, "Ghost" }
                        }
                    }

                    ComponentDemo { label: "Badge Sizes".to_string(),
                        div { class: "flex flex-wrap gap-2 items-center",
                            Badge { badge_size: BadgeSize::Xl, "XL" }
                            Badge { badge_size: BadgeSize::Lg, "Large" }
                            Badge { badge_size: BadgeSize::Md, "Medium" }
                            Badge { badge_size: BadgeSize::Sm, "Small" }
                            Badge { badge_size: BadgeSize::Xs, "XS" }
                        }
                    }

                    ComponentDemo { label: "BlankSlate".to_string(),
                        BlankSlate {
                            heading: "No items found".to_string(),
                            visual: HEADER_SVG.to_string(),
                            description: "Get started by creating your first item. Click the button below to begin.".to_string(),
                            primary_action: ("Create Item".to_string(), "#".to_string()),
                            secondary_action: ("Learn More".to_string(), "#".to_string()),
                        }
                    }
                }

                // ============================================
                // SECTION: Form Elements
                // ============================================
                Section {
                    title: "Form Elements".to_string(),
                    description: "Input controls and form components for data entry.".to_string(),

                    ComponentDemo { label: "Input Types".to_string(),
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            Input { name: "text-input".to_string(), label: "Text Input".to_string(), placeholder: "Enter text...".to_string() }
                            Input { name: "email-input".to_string(), input_type: InputType::Email, label: "Email".to_string(), placeholder: "email@example.com".to_string() }
                            Input { name: "password-input".to_string(), input_type: InputType::Password, label: "Password".to_string(), placeholder: "********".to_string() }
                            Input { name: "number-input".to_string(), input_type: InputType::Number, label: "Number".to_string(), placeholder: "0".to_string() }
                        }
                    }

                    ComponentDemo { label: "Input with Help Text".to_string(),
                        Input {
                            name: "username".to_string(),
                            label: "Username".to_string(),
                            placeholder: "Enter username".to_string(),
                            help_text: "Your username must be 3-20 characters long.".to_string()
                        }
                    }

                    ComponentDemo { label: "Input with Fieldset Legend".to_string(),
                        Fieldset {
                            legend: "What is your name?".to_string(),
                            help_text: "Optional".to_string(),
                            input {
                                r#type: "text",
                                class: "input",
                                placeholder: "Type here"
                            }
                        }
                    }

                    ComponentDemo { label: "TextArea".to_string(),
                        TextArea {
                            name: "textarea-demo".to_string(),
                            label: "Description".to_string(),
                            placeholder: "Enter your description here...".to_string(),
                            rows: "4".to_string(),
                            help_text: "Maximum 500 characters.".to_string()
                        }
                    }

                    ComponentDemo { label: "Select".to_string(),
                        Select {
                            name: "select-demo".to_string(),
                            label: "Choose an option".to_string(),
                            SelectOption { value: "".to_string(), "-- Select --" }
                            SelectOption { value: "1".to_string(), "Option 1" }
                            SelectOption { value: "2".to_string(), "Option 2" }
                            SelectOption { value: "3".to_string(), "Option 3" }
                        }
                    }

                    ComponentDemo { label: "CheckBox".to_string(),
                        div { class: "flex flex-col gap-3",
                            div { class: "flex items-center gap-2",
                                CheckBox { name: "check1".to_string(), value: "1".to_string(), "Default checkbox" }
                            }
                            div { class: "flex items-center gap-2",
                                CheckBox { name: "check2".to_string(), value: "2".to_string(), checkbox_scheme: CheckBoxScheme::Primary, "Primary checkbox" }
                            }
                            div { class: "flex items-center gap-2",
                                CheckBox { name: "check3".to_string(), value: "3".to_string(), checked: true, "Checked checkbox" }
                            }
                        }
                    }

                    ComponentDemo { label: "Range".to_string(),
                        div { class: "space-y-4",
                            Range {
                                name: "range-default".to_string(),
                                min: 0,
                                max: 100,
                                value: 50,
                                label: "Default Range".to_string(),
                            }
                            Range {
                                name: "range-success".to_string(),
                                min: 0,
                                max: 100,
                                value: 75,
                                label: "Success Range".to_string(),
                                range_color: RangeColor::Success,
                            }
                            Range {
                                name: "range-error".to_string(),
                                min: 0,
                                max: 100,
                                value: 25,
                                label: "Error Range".to_string(),
                                range_color: RangeColor::Error,
                            }
                        }
                    }

                    ComponentDemo { label: "FileInput".to_string(),
                        div { class: "space-y-4",
                            FileInput { name: "file-default".to_string() }
                            FileInput { name: "file-primary".to_string(), file_input_color: FileInputColor::Primary }
                            FileInput { name: "file-secondary".to_string(), file_input_color: FileInputColor::Secondary }
                        }
                    }

                    ComponentDemo { label: "Fieldset".to_string(),
                        Fieldset {
                            legend: "User Information".to_string(),
                            help_text: "Please fill in your details below.".to_string(),
                            div { class: "space-y-4",
                                Input { name: "fname".to_string(), label: "First Name".to_string(), placeholder: "John".to_string() }
                                Input { name: "lname".to_string(), label: "Last Name".to_string(), placeholder: "Doe".to_string() }
                                Input { name: "email".to_string(), input_type: InputType::Email, label: "Email".to_string(), placeholder: "john@example.com".to_string() }
                            }
                        }
                    }
                }

                // ============================================
                // SECTION: Data Display
                // ============================================
                Section {
                    title: "Data Display".to_string(),
                    description: "Components for presenting data and content.".to_string(),

                    ComponentDemo { label: "Avatar".to_string(),
                        div { class: "flex gap-4 items-end",
                            Avatar { name: "John Doe".to_string(), avatar_size: AvatarSize::Small }
                            Avatar { name: "Jane Smith".to_string(), avatar_size: AvatarSize::Medium }
                            Avatar { name: "Bob Wilson".to_string(), avatar_size: AvatarSize::Large }
                            Avatar { name: "Alice Brown".to_string(), avatar_size: AvatarSize::ExtraLarge }
                        }
                    }

                    ComponentDemo { label: "Avatar Types".to_string(),
                        div { class: "flex gap-4 items-center",
                            Avatar { avatar_type: AvatarType::User, name: "User".to_string(), avatar_size: AvatarSize::Medium }
                            Avatar { avatar_type: AvatarType::Team, name: "Team Alpha".to_string(), avatar_size: AvatarSize::Medium }
                        }
                    }

                    ComponentDemo { label: "Card".to_string(),
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            Card { class: "bg-base-200".to_string(),
                                CardHeader { title: "Card Title".to_string(),
                                    Badge { badge_color: BadgeColor::Primary, "New" }
                                }
                                CardBody {
                                    p { "This is the card body content. Cards are useful for grouping related information together." }
                                }
                            }
                            Card { class: "bg-base-200".to_string(),
                                CardHeader { title: "Another Card".to_string(),
                                    Badge { badge_color: BadgeColor::Success, "Active" }
                                }
                                CardBody {
                                    p { "Cards can contain any content including forms, images, lists, and more." }
                                }
                            }
                        }
                    }

                    ComponentDemo { label: "Breadcrumb".to_string(),
                        Breadcrumb {
                            items: vec![
                                BreadcrumbItem { text: "Home".to_string(), href: Some("#".to_string()) },
                                BreadcrumbItem { text: "Components".to_string(), href: Some("#".to_string()) },
                                BreadcrumbItem { text: "Kitchen Sink".to_string(), href: None },
                            ]
                        }
                    }
                }

                // ============================================
                // SECTION: Navigation & Structure
                // ============================================
                Section {
                    title: "Navigation & Structure".to_string(),
                    description: "Components for organizing content and navigation.".to_string(),

                    ComponentDemo { label: "TabContainer".to_string(),
                        TabContainer {
                            TabPanel { name: "tabs1".to_string(), tab_name: "Tab 1".to_string(), checked: true,
                                div { class: "p-4 bg-base-200 rounded-b-lg",
                                    p { "Content for Tab 1. This is the default active tab." }
                                }
                            }
                            TabPanel { name: "tabs1".to_string(), tab_name: "Tab 2".to_string(),
                                div { class: "p-4 bg-base-200 rounded-b-lg",
                                    p { "Content for Tab 2. Click to see this content." }
                                }
                            }
                            TabPanel { name: "tabs1".to_string(), tab_name: "Tab 3".to_string(),
                                div { class: "p-4 bg-base-200 rounded-b-lg",
                                    p { "Content for Tab 3. Tabs are great for organizing content." }
                                }
                            }
                        }
                    }

                    ComponentDemo { label: "Pagination".to_string(),
                        Pagination {
                            prev_page_url: "#prev".to_string(),
                            next_page_url: "#next".to_string(),
                        }
                    }

                    ComponentDemo { label: "Accordian".to_string(),
                        div { class: "space-y-1",
                            Accordian { name: "accordion1".to_string(), title: "Section 1 - What is Daisy RSX?".to_string(), checked: true,
                                p { class: "p-4", "Daisy RSX is a Rust library providing Dioxus components that implement DaisyUI (a Tailwind CSS component library)." }
                            }
                            Accordian { name: "accordion1".to_string(), title: "Section 2 - How do I install it?".to_string(),
                                p { class: "p-4", "Add daisy_rsx to your Cargo.toml dependencies and include the DaisyUI stylesheet in your project." }
                            }
                            Accordian { name: "accordion1".to_string(), title: "Section 3 - Is it production ready?".to_string(),
                                p { class: "p-4", "Yes! Daisy RSX is designed for server-side rendering and is suitable for production use." }
                            }
                        }
                    }
                }

                // ============================================
                // SECTION: Overlays & Modals
                // ============================================
                Section {
                    title: "Overlays & Modals".to_string(),
                    description: "Popup dialogs and slide-out drawers for contextual content.".to_string(),

                    ComponentDemo { label: "Modal".to_string(),
                        div {
                            Button {
                                popover_target: "demo-modal".to_string(),
                                button_scheme: ButtonScheme::Primary,
                                "Open Modal"
                            }
                            Modal { trigger_id: "demo-modal".to_string(),
                                ModalBody {
                                    h3 { class: "font-bold text-lg", "Modal Title" }
                                    p { class: "py-4", "This is the modal content. Modals are useful for confirmations, forms, and displaying additional information without leaving the page." }
                                }
                                ModalAction {
                                    Button { button_style: ButtonStyle::Ghost, "Cancel" }
                                    Button { button_scheme: ButtonScheme::Primary, "Confirm" }
                                }
                            }
                        }
                    }

                    ComponentDemo { label: "Drawer".to_string(),
                        div {
                            a { href: "#demo-drawer", class: "btn btn-secondary", "Open Drawer" }
                            Drawer {
                                trigger_id: "demo-drawer".to_string(),
                                label: "Drawer Title".to_string(),
                                DrawerBody {
                                    p { "This is the drawer body. Drawers slide in from the side and are great for navigation menus, settings panels, or detailed information." }
                                    div { class: "mt-4",
                                        Input { name: "drawer-input".to_string(), label: "Example Input".to_string(), placeholder: "Type something...".to_string() }
                                    }
                                }
                                DrawerFooter {
                                    Button { button_style: ButtonStyle::Ghost, "Cancel" }
                                    Button { button_scheme: ButtonScheme::Primary, "Save" }
                                }
                            }
                        }
                    }
                }

                // ============================================
                // SECTION: Timelines
                // ============================================
                Section {
                    title: "Timelines".to_string(),
                    description: "Components for displaying chronological events and progress.".to_string(),

                    ComponentDemo { label: "Timeline (Vertical)".to_string(),
                        Timeline { direction: TimelineDirection::Vertical,
                            TimelineItem {
                                hr {}
                                TimelineStart { "Step 1" }
                                TimelineMiddle {
                                    svg {
                                        class: "w-5 h-5 text-primary",
                                        fill: "currentColor",
                                        view_box: "0 0 20 20",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path {
                                            fill_rule: "evenodd",
                                            d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z",
                                            clip_rule: "evenodd"
                                        }
                                    }
                                }
                                TimelineEnd { boxed: true, "Project Started" }
                                hr {}
                            }
                            TimelineItem {
                                hr {}
                                TimelineStart { "Step 2" }
                                TimelineMiddle {
                                    svg {
                                        class: "w-5 h-5 text-primary",
                                        fill: "currentColor",
                                        view_box: "0 0 20 20",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path {
                                            fill_rule: "evenodd",
                                            d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z",
                                            clip_rule: "evenodd"
                                        }
                                    }
                                }
                                TimelineEnd { boxed: true, "Design Complete" }
                                hr {}
                            }
                            TimelineItem {
                                hr {}
                                TimelineStart { "Step 3" }
                                TimelineMiddle {
                                    svg {
                                        class: "w-5 h-5 text-warning",
                                        fill: "currentColor",
                                        view_box: "0 0 20 20",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        circle { cx: "10", cy: "10", r: "8" }
                                    }
                                }
                                TimelineEnd { boxed: true, "In Progress" }
                                hr {}
                            }
                            TimelineItem {
                                hr {}
                                TimelineStart { "Step 4" }
                                TimelineMiddle {
                                    svg {
                                        class: "w-5 h-5 text-base-300",
                                        fill: "currentColor",
                                        view_box: "0 0 20 20",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        circle { cx: "10", cy: "10", r: "8" }
                                    }
                                }
                                TimelineEnd { boxed: true, "Pending Review" }
                            }
                        }
                    }

                    ComponentDemo { label: "TimeLine (Legacy Style)".to_string(),
                        div {
                            TimeLine {
                                TimeLineBadge { image_src: HEADER_SVG.to_string() }
                                TimeLineBody {
                                    p { class: "font-semibold", "Event 1" }
                                    p { "Initial commit and project setup completed." }
                                }
                            }
                            TimeLine { condensed: true,
                                TimeLineBadge { image_src: HEADER_SVG.to_string() }
                                TimeLineBody {
                                    p { class: "font-semibold", "Event 2" }
                                    p { "Added core functionality and tests." }
                                }
                            }
                            TimeLine { condensed: true,
                                TimeLineBadge { image_src: HEADER_SVG.to_string() }
                                TimeLineBody {
                                    p { class: "font-semibold", "Event 3" }
                                    p { "Released version 1.0.0" }
                                }
                            }
                        }
                    }
                }

                // ============================================
                // SECTION: Utility
                // ============================================
                Section {
                    title: "Utility".to_string(),
                    description: "Helper components for common use cases.".to_string(),

                    ComponentDemo { label: "RelativeTime".to_string(),
                        div { class: "space-y-2",
                            div { class: "flex items-center gap-2",
                                span { class: "font-semibold", "Default (Relative):" }
                                RelativeTime { datetime: "2024-01-01T00:00:00Z".to_string() }
                            }
                            div { class: "flex items-center gap-2",
                                span { class: "font-semibold", "Datetime Format:" }
                                RelativeTime { datetime: "2024-06-15T12:30:00Z".to_string(), format: RelativeTimeFormat::Datetime }
                            }
                            div { class: "flex items-center gap-2",
                                span { class: "font-semibold", "Micro Format:" }
                                RelativeTime { datetime: "2024-12-25T00:00:00Z".to_string(), format: RelativeTimeFormat::Micro }
                            }
                        }
                    }
                }

                // ============================================
                // SECTION: Marketing Components
                // ============================================
                Section {
                    title: "Marketing Components".to_string(),
                    description: "Pre-built sections for marketing and landing pages.".to_string(),

                    ComponentDemo { label: "Hero".to_string(),
                        Hero {
                            title: "Welcome to Our Product".to_string(),
                            subtitle: "The best solution for your software development needs. Build faster, ship smarter.".to_string(),
                            cta: "Get Started".to_string(),
                            cta_link: "#".to_string(),
                        }
                    }

                    ComponentDemo { label: "VideoHero".to_string(),
                        VideoHero {
                            title: "See It In Action".to_string(),
                            subtitle: "Watch our quick demo to see how easy it is to get started with our platform.".to_string(),
                            video: "https://www.youtube.com/embed/dQw4w9WgXcQ".to_string(),
                            claim: "Trusted by 10,000+ developers".to_string(),
                            cta: "Start Free Trial".to_string(),
                            cta_link: "#".to_string(),
                        }
                    }

                    ComponentDemo { label: "Features".to_string(),
                        Features {
                            title: "Our Features".to_string(),
                            description: "Everything you need to build amazing applications.".to_string(),
                            features: vec![
                                Feature { title: "Lightning Fast".to_string(), description: "Optimized for performance with zero runtime overhead.".to_string() },
                                Feature { title: "Type Safe".to_string(), description: "Catch errors at compile time, not in production.".to_string() },
                                Feature { title: "Easy to Use".to_string(), description: "Simple and intuitive API that feels natural.".to_string() },
                            ],
                        }
                    }

                    ComponentDemo { label: "Benefits".to_string(),
                        Benefits {
                            title: "Why Choose Us".to_string(),
                            subtitle: "Three key benefits that set us apart".to_string(),
                            benefit1: "Save Time".to_string(),
                            benefit1_desc: "Automate repetitive tasks and focus on what matters.".to_string(),
                            benefit2: "Reduce Costs".to_string(),
                            benefit2_desc: "Lower operational costs with efficient workflows.".to_string(),
                            benefit3: "Scale Easily".to_string(),
                            benefit3_desc: "Grow your business without growing pains.".to_string(),
                        }
                    }

                    ComponentDemo { label: "Faq".to_string(),
                        Faq {
                            questions: vec![
                                FaqText { question: "What is Daisy RSX?".to_string(), answer: "Daisy RSX is a Rust library providing Dioxus components that implement DaisyUI, a Tailwind CSS component library.".to_string() },
                                FaqText { question: "Is it open source?".to_string(), answer: "Yes! Daisy RSX is fully open source and available on crates.io.".to_string() },
                                FaqText { question: "Can I use it in production?".to_string(), answer: "Absolutely! It's designed for server-side rendering and production use.".to_string() },
                            ],
                        }
                    }

                    ComponentDemo { label: "Testamonials".to_string(),
                        Testamonials {
                            text1: "Daisy RSX has transformed how we build UIs in Rust. The components are beautiful and the API is intuitive.".to_string(),
                            job1: "Senior Developer".to_string(),
                            person1: "John Doe".to_string(),
                            img1: "https://api.dicebear.com/7.x/avataaars/svg?seed=John".to_string(),
                            text2: "Finally, a component library that feels natural in Rust. We've reduced our frontend development time by 50%.".to_string(),
                            job2: "Tech Lead".to_string(),
                            person2: "Jane Smith".to_string(),
                            img2: "https://api.dicebear.com/7.x/avataaars/svg?seed=Jane".to_string(),
                        }
                    }

                    ComponentDemo { label: "Customers".to_string(),
                        Customers {}
                    }

                    ComponentDemo { label: "Security".to_string(),
                        Security {}
                    }

                    ComponentDemo { label: "ProblemSolution".to_string(),
                        ProblemSolution {
                            title: "The Challenge".to_string(),
                            problem: "Development teams waste countless hours wrestling with complex UI frameworks that don't integrate well with Rust's type system.".to_string(),
                            solution: "Our platform provides type-safe, compile-time checked components that integrate seamlessly with your Rust codebase, eliminating runtime errors and reducing debugging time.".to_string(),
                            image: "https://placehold.co/560x420/e2e8f0/475569?text=Solution".to_string(),
                        }
                    }

                    ComponentDemo { label: "SmallImageFeature".to_string(),
                        SmallImageFeature {
                            title: "Feature".to_string(),
                            sub_title: "Powerful Development Tools".to_string(),
                            text: "Build beautiful, responsive interfaces with our comprehensive component library. Every component is designed with accessibility and performance in mind.".to_string(),
                            image: "https://placehold.co/728x610/e2e8f0/475569?text=Feature".to_string(),
                            flip: false,
                        }
                    }

                    ComponentDemo { label: "QuadFeature".to_string(),
                        QuadFeature {
                            title: "Platform".to_string(),
                            sub_title: "Everything You Need".to_string(),
                            text: "A complete solution for modern web development.".to_string(),
                            title1: "Cloud Deploy".to_string(),
                            text1: "Deploy anywhere with one click.".to_string(),
                            title2: "Secure by Default".to_string(),
                            text2: "Enterprise-grade security built in.".to_string(),
                            title3: "Auto Scaling".to_string(),
                            text3: "Handle any load automatically.".to_string(),
                            title4: "Analytics".to_string(),
                            text4: "Understand your users better.".to_string(),
                        }
                    }

                    ComponentDemo { label: "ImageFeature".to_string(),
                        ImageFeature {
                            title: "Deploy".to_string(),
                            sub_title: "Ship with Confidence".to_string(),
                            text: "Our deployment pipeline ensures your code reaches production safely.".to_string(),
                            title1: "Push to Deploy. ".to_string(),
                            text1: "Git push and watch your changes go live automatically.".to_string(),
                            title2: "SSL Certificates. ".to_string(),
                            text2: "Automatic HTTPS for all your domains.".to_string(),
                            title3: "Database Backups. ".to_string(),
                            text3: "Daily automated backups with point-in-time recovery.".to_string(),
                            image: "https://placehold.co/800x600/e2e8f0/475569?text=Deploy".to_string(),
                        }
                    }

                    ComponentDemo { label: "ExtraFooter".to_string(),
                        ExtraFooter {
                            title: "Ready to get started? Join thousands of developers today.".to_string(),
                            image: "https://placehold.co/400x300/e2e8f0/475569?text=CTA".to_string(),
                            cta: "Start Building".to_string(),
                            cta_url: "#".to_string(),
                        }
                    }

                    ComponentDemo { label: "WebinarHeader".to_string(),
                        WebinarHeader {}
                    }

                    ComponentDemo { label: "Team".to_string(),
                        Team {}
                    }
                }

                // Footer
                div { class: "text-center py-8 border-t border-base-300 mt-16",
                    p { class: "text-base-content/60",
                        "Built with Daisy RSX - A Dioxus component library implementing DaisyUI"
                    }
                }
            }
        }
    }
}
