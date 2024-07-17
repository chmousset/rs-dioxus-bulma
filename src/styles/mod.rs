use dioxus::prelude::*;

pub const BULMA_CSS: &str = include_str!("bulma.min.css");

#[component]
pub fn BulmaStylesheet() -> Element {
    rsx!(
        style {
            "{BULMA_CSS}"
        }
    )
}
