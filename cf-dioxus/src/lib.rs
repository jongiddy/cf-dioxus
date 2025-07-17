use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    let mut m1 = use_signal(|| 1i32);
    let mut m2 = use_signal(|| 1i32);

    #[cfg(not(feature = "api"))]
    let product = use_memo(move || match m1().checked_mul(m2()) {
        Some(product) => format!("= {product}"),
        None => "overflow".to_string(),
    });

    #[cfg(feature = "api")]
    let product = {
        let multiplication = use_resource(move || api::multiply(m1(), m2()));
        let mut product = use_signal(|| "= ?".to_string());
        use_effect(move || {
            product.set(match &*multiplication.value().read() {
                Some(Ok(value)) => format!("= {value}"),
                Some(Err(err)) => err.to_string(),
                None => "= ?".to_string(),
            })
        });
        product
    };

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
                    "{product}"
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

#[cfg(feature = "api")]
pub mod api {
    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct MultiplyRequest {
        pub a: i32,
        pub b: i32,
    }

    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct MultiplyResponse {
        pub result: i32,
    }

    pub async fn multiply(a: i32, b: i32) -> Result<i32, std::io::Error> {
        let location = ::web_sys::window().unwrap().location().origin().unwrap();
        let mut url = reqwest::Url::parse(&location).map_err(std::io::Error::other)?;
        url.set_path("api/multiply");
        let query =
            serde_urlencoded::to_string(MultiplyRequest { a, b }).map_err(std::io::Error::other)?;
        url.set_query(Some(&query));
        let response = reqwest::get(url).await.map_err(std::io::Error::other)?;

        if !response.status().is_success() {
            return Err(std::io::Error::other(response.status().to_string()));
        }

        let multiplication = response
            .json::<MultiplyResponse>()
            .await
            .map_err(std::io::Error::other)?;

        Ok(multiplication.result)
    }
}
