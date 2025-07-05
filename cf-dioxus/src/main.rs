use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    let mut m1 = use_signal(|| 1);
    let mut m2 = use_signal(|| 1);
    let product = use_memo(move || m1() * m2());
    rsx! {
        div {
            display: "flex",
            flex: "1 1 auto",
            justify_content: "center",
            div {
                display: "grid",
                grid_template_columns: "2cm 50px 2cm 4cm",
                align_items: "center",
                justify_items: "center",

                // Top row
                div {
                    button {
                        onclick: move |_| {m1 += 1;},
                        "+"
                    }
                }
                div {}
                div {
                    button {
                        onclick: move |_| {m2 += 1;},
                        "+"
                    }
                }
                div {}

                // Middle row
                div {
                    text_align: "center",
                    "{m1}"
                }
                div {
                    text_align: "center",
                    dangerous_inner_html: "&times;"
                }
                div {
                    text_align: "center",
                    "{m2}"
                }
                div {
                    text_align: "center",
                    "= {product}"
                }

                // Bottom row
                div {
                    button {
                        onclick: move |_| {m1 -= 1;},
                        "-"
                    }
                }
                div {}
                div {
                    button {
                        onclick: move |_| {m2 -= 1;},
                        "-"
                    }
                }
                div {}
            }

         }
    }
}
