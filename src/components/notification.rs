use crate::components::button::Delete;
use dioxus::prelude::*;
use std::collections::BTreeMap;

#[component]
fn Notification(
    class: Option<String>,
    mut notifications: Signal<BTreeMap<u32, String>>,
    id: u32,
) -> Element {
    let content = use_memo(move || notifications.read().get(&id).unwrap().clone());

    let classes = match &class {
        Some(classes) => format!("notification {}", classes),
        None => "notification".to_string(),
    };

    rsx!(div {
        class: "{classes}",
        Delete {
            prevent_default: "onclick",
            onclick: move |_| {
                notifications.write().remove(&id);
            }
        }
        "{content}"
    })
}

#[component]
pub fn NotificationDisplay(mut notifications: Signal<BTreeMap<u32, String>>) -> Element {
    rsx!(div {
        class: "container",
        style: "hidth: 50%; position: top",
        for id in notifications.read().keys() {
            Notification {
                key: "{id}",
                notifications,
                id: *id,
            }
        }
    })
}
