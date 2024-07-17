use dioxus::prelude::*;

#[component]
pub fn Container(class: Option<String>, children: Element) -> Element {
    let classes = match &class {
        Some(classes) => format!("container {}", classes),
        None => "container".to_string(),
    };

    rsx!(div {
        class: "{classes}",
        {children}
    })
}

#[component]
pub fn Block(children: Element) -> Element {
    rsx!(div {
        class: "block",
        {children}
    })
}

#[component]
pub fn Box(children: Element) -> Element {
    rsx!(div {
        class: "box",
        {children}
    })
}

#[component]
pub fn Content(children: Element, size: Option<String>) -> Element {
    let classes = match &size {
        Some(size) => format!("content is-{}", size),
        None => "content".to_string(),
    };

    rsx!(div {
        class: "{classes}",
        {children}
    })
}

#[component]
pub fn Section(children: Element, size: Option<String>) -> Element {
    let classes = match &size {
        Some(size) => format!("section is-{}", size),
        None => "section".to_string(),
    };

    rsx!(div {
        class: "{classes}",
        {children}
    })
}

#[component]
pub fn Footer(class: Option<String>, children: Element) -> Element {
    let classes = match &class {
        Some(classes) => format!("footer {}", classes),
        None => "footer".to_string(),
    };

    rsx!(div {
        class: "{classes}",
        {children}
    })
}

#[component]
pub fn Hero(class: Option<String>, title: String, subtitle: String) -> Element {
    let classes = match &class {
        Some(classes) => format!("hero {}", classes),
        None => "hero".to_string(),
    };

    rsx!(section {
        class: "{classes}",
        div {
            class: "hero-body",
            p {
                class: "title",
                "{title}"
            }
            p {
                class: "subtitle",
                "{subtitle}"
            }
        }
    })
}

#[component]
pub fn Modal(children: Element, active: Signal<bool>) -> Element {
    let close = move |_| {
        let mut active_clone = active.clone();
        active_clone.set(false);
    };

    rsx!(
        div {
            class: if active() {"modal is-active"},
            class: if !active() {"modal"},
            div {
                class: "modal-background",
                onclick: close
            }
            div {
                class: "modal-content",
                {children}
            }
            button {
                class: "modal-close is-large",
                "aria-label": "close",
                onclick: close
            }
        }
    )
}
