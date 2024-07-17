use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct Button2Props {
    icon: Option<String>,
    children: Element,
    #[props(extends = button)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Button2(props: Button2Props) -> Element {
    rsx!(div {
        ..props.attributes,
        class: "button",
        if let Some(i) = props.icon {
            span {
                class: "icon",
                i {
                    class: "{i}"
                }
            }
        }
        if props.children.is_some() {
            span {
                {props.children}
            }
        }
        else {
            ""
        }
    })
}

#[component]
pub fn Button(
    class: Option<String>,
    icon: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
    prevent_default: Option<String>,
    children: Element,
) -> Element {
    rsx!(div {
        class: "button",
        class: if class.is_some() {class.unwrap()},
        prevent_default: if prevent_default.is_some() {prevent_default.unwrap()},
        onclick: move |event| {
            if let Some(action) = onclick {
                action(event)
            }
        },
        if let Some(i) = icon {
            span {
                class: "icon",
                i {
                    class: "{i}"
                }
            }
        }
        if children.is_some() {
            span {
                {children}
            }
        }
        else {
            ""
        }
    })
}

#[component]
pub fn Delete(
    onclick: Option<EventHandler<MouseEvent>>,
    size: Option<String>,
    prevent_default: Option<String>,
) -> Element {
    let classes = match &size {
        Some(size) => format!("delete is-{}", size),
        None => "delete".to_string(),
    };

    rsx!(div {
        prevent_default: if prevent_default.is_some() {prevent_default.unwrap()},
        class: "{classes}",
        onclick: move |event| {
            if let Some(action) = onclick {
                action(event)
            }
        },
        ""
    })
}
