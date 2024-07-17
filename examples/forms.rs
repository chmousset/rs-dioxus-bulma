use dioxus::prelude::*;
use dioxus_bulma::components::containers::{Box, Container, Content, Footer, Hero, Section};
use dioxus_bulma::components::form::*;
use dioxus_bulma::styles::BulmaStylesheet;
use dioxus_logger::tracing::Level;

fn app() -> Element {
    let text_value = use_signal(|| String::new());
    let string_value = use_signal(|| String::new());
    let dropdown_value = use_signal(|| String::new());

    rsx! {
        // BulmaStylesheet {}
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bulma@1.0.0/css/bulma.css"
        }
        link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/MaterialDesign-Webfont/7.4.47/css/materialdesignicons.min.css"
        }
        Hero {
            class: "is-primary",
            title: "Dioxus Demo",
            subtitle: "How nice is that?"
        }
        Section {
            Container {
                Box {
                    Content {
                        "This demo uses Bulma's "
                        a {
                            href: "https://bulma.io/documentation/form/general/",
                            "forms"
                        }
                        ". "
                    }
                }
            }
            br {}
            Box {
                div {
                    class: "field-body is-horizontal is-expanded",
                    StringInput {
                        placeholder: "string_value",
                        value: string_value,
                        addon_left: None,
                        addon_right: None,
                    }
                    StringInput {
                        placeholder: "string_value",
                        value: string_value,
                        addon_left: None,
                        addon_right: None,
                    }
                }
            }
            Box {
                a {
                    class: "title is-3",
                    href: "https://bulma.io/documentation/form/general/#with-icons",
                    "Icons"
                }
                p {
                    "Use 'icon_left' and 'icon_right properties to add icons (ex: 'mdi mdi-account')"
                }
                StringInput {
                    placeholder: "string_value",
                    value: string_value,
                    icon_left: "mdi mdi-account",
                    addon_left: None,
                    addon_right: None,
                }
            }
            //     Field {
            //         label: "my beautiful text",
            //         help: "type your next roman",
            //         horizontal: true,
            //         TextInput {
            //             value: text_value,
            //         }
            //     }
            //     Field {
            //         label: "what is your name?",
            //         help: "type your next roman",
            //         StringInput {
            //             value: string_value,
            //         }
            //     }
            //     Field {
            //         label: "Choose your sex",
            //         Dropdown {
            //             value: dropdown_value,
            //             options: vec!["Male".to_string(), "Female".to_string()],
            //         }
            //     }
            //     Field {
            //         label: "what is your name?",
            //         help: "type your next roman",
            //         FieldAddon {
            //             StringInput {
            //                 value: string_value,
            //             }
            //             ButtonInput {
            //                 // class: "is-static",
            //                 "toto"
            //             }
            //         }
            //     }
            //     div {
            //         class: "field is-horizontal",
            //         div {
            //             class: "field-label",
            //             ""
            //         }
            //         div {
            //             class: "field-body",
            //             div {
            //                 class: "field is-expanded",
            //                 div {
            //                     class: "field has-addon",
            //                     p {
            //                         class: "control is-expanded",
            //                         input {
            //                             class: "input",
            //                         }
            //                     }
            //                     p {
            //                         class: "control",
            //                         a {
            //                             class: "button is-static",
            //                             "toto"
            //                         }
            //                     }
            //                 }
            //                 p {
            //                     class: "help",
            //                     "help"
            //                 }
            //             }
            //         }
            //     }
            // }
            Box {
                h1 {"Values"}
                ul {
                    li {
                        "text_value: "
                        b {{text_value}}
                    }
                    li {
                        "string_value: "
                        b {{string_value}}
                    }
                    li {
                        "dropdown_value: "
                        b {{dropdown_value}}
                    }
                }
            }
            hr {}
            Box {
                form {
                    StringInput {
                        horizontal: true,
                        expanded: true,
                        label: "From",
                        help: "enter email",
                        value: string_value,
                        addon_left: None,
                        addon_right: rsx! (ButtonAddon {
                            "click me"
                        })
                    }
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
    }
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(app);
}
