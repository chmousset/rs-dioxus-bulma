mod components;
use components::button::{Button, Delete};
use components::containers::{Block, Box, Container, Footer, Hero, Modal, Section};
use components::form::Field;
use components::notification::NotificationDisplay;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
struct User {
    name: String,
}

#[component]
fn UserEntry(mut users: Signal<BTreeMap<u32, User>>, id: u32) -> Element {
    let name = use_memo(move || users.read().get(&id).unwrap().name.clone());

    rsx!(
        li {
            "({id}){name}"
            Delete {
                prevent_default: "onclick",
                onclick: move |_| {
                    users.write().remove(&id);
                }
            }
        }
    )
}

#[component]
fn UserAdder(mut users: Signal<BTreeMap<u32, User>>) -> Element {
    let mut todo_id = use_signal(|| 0);
    let username = use_signal(|| String::new());

    rsx!(
        Field {
            // icon_left: "mdi mdi-account",
            // icon_right: "mdi mdi-plus",
            Button {
                onclick: move |_| {
                    let user = User {
                        name: username.to_string(),
                    };
                    users.write().insert(todo_id(), user);
                    todo_id += 1;
                },
                "Add user"
            }
        }
    )
}

#[component]
fn Navbar() -> Element {
    // let search_text = use_signal(|| String::new());
    let mut is_active = use_signal(|| false);

    rsx! (
        nav {
            class: "navbar",
            role: "navigation",
            div {
                class: "navbar-brand",
                a {
                    class: "navbar-item",
                    href: "https://bulma.io",
                }
                a {
                    role: "button",
                    class: "navbar-burger",
                    class: if is_active() {"is-active"},
                    "aria-label": "menu",
                    "aria-expanded": "false",
                    "data-target": "navbarBasicExample",
                    onclick: move |_| {
                        is_active.set(!is_active());
                    },
                    span {"aria-hidden": "true"}
                    span {"aria-hidden": "true"}
                    span {"aria-hidden": "true"}
                    span {"aria-hidden": "true"}
                    span {"aria-hidden": "true"}
                }
            }
            div {
                class: "navbar-menu",
                class: if is_active() {"is-active"},
                div {
                    class: "navbar-start",
                    a {
                        class: "navbar-item",
                        "one"
                    }
                    a {
                        class: "navbar-item",
                        "doc"
                    }
                    div {
                        class: "navbar-item has-dropdown is-hoverable",
                        a {
                            class: "navbar-link",
                            "products"
                        }
                        div {
                            class: "navbar-dropdown",
                            a {
                                class: "navbar-item",
                                "product 1"
                            }
                            hr {
                                class: "navbar-divider",
                            }
                            a {
                                class: "navbar-item",
                                "product 2"
                            }
                        }
                    }
                }
                div {
                    class: "navbar-end",
                    // div {
                    //     class: "navbar-item",
                    //     Field {
                    //         value: search_text,
                    //         class: "is-small",
                    //         Button {
                    //             class: "is-small",
                    //             "search"
                    //         }
                    //     }
                    // }
                    a {
                        class: "navbar-item",
                        "About"
                    }
                }
            }
        }
    )
}

fn app() -> Element {
    let mut cnt = use_signal_sync(|| 0);
    let users = use_signal(|| BTreeMap::<u32, User>::new());
    let mut notifications = use_signal(|| BTreeMap::<u32, String>::new());
    let mut modal_active = use_signal(|| false);
    let search_text = use_signal(|| String::new());

    rsx! {
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bulma@1.0.0/css/bulma.css"
        }
        link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/MaterialDesign-Webfont/7.4.47/css/materialdesignicons.min.css"
        }
        // Container {
        //     nav {
        //         class: "level",
        //         div {
        //             class: "level-left",
        //             div {
        //                 class: "level-item",
        //                 a {
        //                     href: "/",
        //                     "Home"
        //                 }
        //             }
        //             div {
        //                 class: "level-item",
        //                 a {
        //                     class: "is-primary is-active",
        //                     href: "/",
        //                     "Products"
        //                 }
        //             }
        //             div {
        //                 class: "level-item",
        //                 Field {
        //                     value: search_text,
        //                     class: "is-small",
        //                     Button {
        //                         class: "is-small",
        //                         "search"
        //                     }
        //                 }
        //             }
        //         }
        //         div {
        //             class: "level-right",
        //             div {
        //                 class: "level-item",
        //                 "About"
        //             }
        //         }
        //     }
        // }
        Navbar {}
        Hero {
            class: "is-primary",
            title: "Dioxus Demo",
            subtitle: "How nice is that?"
        }
        Section {
            NotificationDisplay {
                notifications
            }
            Container {
                Box {
                    Block {
                        div {
                            class: "buttons",
                            Button {
                                class: "is-primary",
                                icon: "mdi mdi-24px mdi-content-save",
                                onclick: move |event| {
                                    println!("clicked! {event:?}");
                                    cnt += 1;
                                    notifications.write().insert(cnt() as u32, format!("Notification {cnt}"));
                                },
                                "click me! {cnt}"
                            }
                            UserAdder { users }
                        }
                    }
                    Block {
                        ol {
                            for id in users.read().keys() {
                                UserEntry {
                                    key: "{id}",
                                    users,
                                    id: *id,
                                }
                            }
                        }
                    }
                }
            }
        }
        Button {
            onclick: move |_| {
                modal_active.set(true);
            },
            "Open Modal"
        }
        Modal {
            active: modal_active,
            Box {
                p {
                    "Hello world!"
                }
            }
        }
        Footer {
            div {
                class: "has-text-centered",
                p {
                    "simple demo!"
                }
            }
        }
        div {
            class: "block",
            p {
                class: "block is-small",
                "(C) Charles-Henri Mousset"
            }
        }
    }
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(app);
}