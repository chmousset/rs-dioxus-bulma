use dioxus::prelude::*;
use dioxus_bulma::components::containers::{Box, Container, Content, Footer, Hero, Section};
use dioxus_bulma::styles::BulmaStylesheet;
use dioxus_logger::tracing::Level;

fn app() -> Element {
    rsx! {
        BulmaStylesheet {}
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
                        "This simple demo uses Bulma's "
                        a {
                            href: "https://bulma.io/documentation/grid/fixed-grid/",
                            "fixed grid"
                        }
                        ". Here, we use `has-auto-count` to make it responsive. Try resizing the window!"
                    }
                }
            }
            br {}
            div {
                class: "box fixed-grid has-auto-count",
                div {
                    class: "grid",
                    for n in 1..20 {
                        div {
                            style: "height: 100%",
                            class: "box cell",
                            class: if (n%3) == 1 {"is-row-span-2"},
                            class: if (n%4) == 1 {"is-col-span-2"},
                            {format!("Cell {n}")}
                        }
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
