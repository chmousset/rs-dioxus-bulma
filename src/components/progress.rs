use dioxus::prelude::*;

#[component]
pub fn Progress(class: Option<String>, value: u32, max: u32) -> Element {
    let classes = match &class {
        Some(classes) => format!("progress {}", classes),
        None => "progress".to_string(),
    };

    rsx!(progress {
        class: "{classes}",
        value: "{value}",
        max: "{max}",
        ""
    })
}
