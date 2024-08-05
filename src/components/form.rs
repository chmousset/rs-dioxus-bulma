use dioxus::{dioxus_core::VPlaceholder, prelude::*};

#[component]
pub fn Form(children: Element) -> Element {
    rsx!()
}

#[component]
pub fn Field(
    class: Option<String>,
    label: Option<String>,
    help: Option<String>,
    horizontal: Option<bool>,
    children: Element,
) -> Element {
    rsx! (
        div {
            class: "field",
            class: if horizontal.unwrap_or(false) {"is-horizontal"},
            if let Some(l) = label {
                div {
                    class: "field-label is-normal",
                    label {
                        class: "label",
                        {l}
                    }
                }
            }
            div {
                class: "field-body",
                {children}
            }
        }
    )
}

#[component]
fn AddonWrapper(
    has_addon: bool,
    expanded: Option<bool>,
    help: Option<String>,
    children: Element,
) -> Element {
    rsx! (
        if has_addon {
            div {
                class: "field",
                class: if expanded.unwrap_or(false) {"is-expanded"},
                div {
                    class: "field has-addons",
                    {children}
                }
                if let Some(h) = help {
                    p {
                        class: "help",
                        {h}
                    }
                }
            }
        }
        else {
            div {
                class: "field",
                class: if expanded.unwrap_or(false) {"is-expanded"},
                {children}
                if let Some(h) = help {
                    p {
                        class: "help",
                        {h}
                    }
                }
            }
        }
    )
}

#[component]
pub fn FieldWrapper(
    label: Option<String>,
    help: Option<String>,
    icon_left: Option<String>,
    icon_right: Option<String>,
    horizontal: Option<bool>,
    expanded: Option<bool>,
    addon_left: Element,
    addon_right: Element,
    children: Element,
) -> Element {
    let has_icon_left = icon_left.is_some();
    let has_icon_right = icon_right.is_some();

    rsx! (
        div {
            class: "field",
            class: if horizontal.unwrap_or(false) {"is-horizontal"},
            if let Some(l) = label {
                div {
                    class: "field-label is-normal",
                    label {
                        class: "label",
                        {l}
                    }
                }
            }
            div {
                class: "field-body",
                AddonWrapper {
                    has_addon: addon_left.is_some() | addon_right.is_some(),
                    help,
                    expanded,
                    {addon_left}
                    div {
                        class: "control",
                        class: if expanded.unwrap_or(false) {"is-expanded"},
                        class: if has_icon_left {"has-icons-left"},
                        class: if has_icon_right {"has-icons-right"},
                        if let Some(icon) = icon_left {
                            span {
                                class: "icon is-left",
                                // class: if let Some(c) = class.clone() {c},
                                i {
                                    class: "{icon}",
                                }
                            }
                        }
                        if let Some(icon) = icon_right {
                            span {
                                class: "icon is-right",
                                // class: if let Some(c) = class.clone() {c},
                                i {
                                    class: "{icon}",
                                }
                            }
                        }
                        {children}
                    }
                    {addon_right}
                }
            }
        }
    )
}

#[component]
pub fn Dropdown(
    label: Option<String>,
    help: Option<String>,
    icon_left: Option<String>,
    icon_right: Option<String>,
    value: Signal<String>,
    options: Vec<String>,
    addon_left: Element,
    addon_right: Element,
) -> Element {
    rsx! (
        FieldWrapper {
            icon_left,
            icon_right,
            addon_left,
            addon_right,
            DropdownAddon {
                value,
                options
            }
        }
    )
}

#[component]
pub fn DropdownAddon(
    icon_left: Option<String>,
    icon_right: Option<String>,
    value: Signal<String>,
    options: Vec<String>,
) -> Element {
    let oninput = move |event: Event<FormData>| {
        let mut value_clone = value.clone();
        value_clone.set(event.value());
    };

    rsx! (
        span {
            class: "select",
            select {
                oninput,
                for o in options {
                    option {
                        selected: if o.clone() == value() {true},
                        {o.clone()}
                    }
                }
            }
        }
    )
}

#[component]
pub fn StringInput(
    label: Option<String>,
    help: Option<String>,
    icon_left: Option<String>,
    icon_right: Option<String>,
    horizontal: Option<bool>,
    expanded: Option<bool>,
    value: Signal<String>,
    placeholder: Option<String>,
    addon_left: Element,
    addon_right: Element,
) -> Element {
    let oninput = move |event: Event<FormData>| {
        let mut value_clone = value.clone();
        value_clone.set(event.value());
    };

    rsx! (
        FieldWrapper {
            label,
            help,
            icon_left,
            icon_right,
            addon_left,
            addon_right,
            horizontal,
            expanded,
            input {
                oninput,
                class: "input",
                "type": "text",
                placeholder,
                value,
            }
        }
    )
}

#[component]
pub fn TextInput(
    label: Option<String>,
    help: Option<String>,
    icon_left: Option<String>,
    icon_right: Option<String>,
    value: Signal<String>,
    placeholder: Option<String>,
    addon_left: Element,
    addon_right: Element,
) -> Element {
    let oninput = move |event: Event<FormData>| {
        let mut value_clone = value.clone();
        value_clone.set(event.value());
    };

    rsx!(
        FieldWrapper {
            label,
            help,
            icon_left,
            icon_right,
            addon_left,
            addon_right,
            textarea {
                oninput,
                class: "textarea",
                placeholder,
                value,
            }
        }
    )
}

#[component]
pub fn ButtonInput(
    icon_left: Option<String>,
    icon_right: Option<String>,
    value: Signal<String>,
    placeholder: Option<String>,
    addon_left: Element,
    addon_right: Element,
    children: Element,
    // FIXME: onclick
) -> Element {
    rsx!(
        FieldWrapper {
            icon_left,
            icon_right,
            addon_left,
            addon_right,
            a {
                class: "button is-static",
                {children}
            }
        }
    )
}

#[component]
pub fn ButtonAddon(
    children: Element,
    // FIXME: onclick
) -> Element {
    rsx!(
        p {
            class: "control",
            a {
                class: "button",
                {children}
            }
        }
    )
}
