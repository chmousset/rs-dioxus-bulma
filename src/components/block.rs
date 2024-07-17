use dioxus::prelude::*;

#[component]
pub fn Block(children: Element) -> Element {
    rsx!(div {
        class: "block",
        {children}
    })
}
