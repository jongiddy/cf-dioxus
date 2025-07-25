#![allow(non_snake_case)]

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
    let mut factor1 = use_signal(|| 1i32);
    let mut factor2 = use_signal(|| 1i32);
    #[allow(unused_mut)]
    let mut opacity = use_signal(|| 1.0);

    #[cfg(not(any(feature = "api", feature = "server-fn")))]
    let answer = use_memo(move || match factor1().checked_mul(factor2()) {
        Some(product) => format!("= {product}"),
        None => "overflow".to_string(),
    });

    #[cfg(feature = "api")]
    let answer = {
        let multiplication = use_resource(move || api::multiply(factor1(), factor2()));
        let mut answer = use_signal(|| "= ?".to_string());
        use_effect(move || {
            answer.set(match &*multiplication.read() {
                Some(Ok(product)) => format!("= {product}"),
                Some(Err(err)) => err.to_string(),
                None => "= ?".to_string(),
            });
        });
        use_effect(move || {
            opacity.set(match &*multiplication.state().read() {
                UseResourceState::Ready => 1.0,
                _ => 0.5
            });
        });
        answer
    };

    #[cfg(feature = "server-fn")]
    let answer = {
        let multiplication = use_resource(move || server_function::multiply(factor1(), factor2()));
        let mut answer = use_signal(|| "= ?".to_string());
        use_effect(move || {
            answer.set(match &*multiplication.read() {
                Some(Ok(product)) => format!("= {product}"),
                Some(Err(err)) => err.to_string(),
                None => "= ?".to_string(),
            });
        });
        use_effect(move || {
            opacity.set(match &*multiplication.state().read() {
                UseResourceState::Ready => 1.0,
                _ => 0.5
            });
        });
        answer
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
                        onclick: move |_| { factor1 += 1; },
                        "+"
                    }
                }
                div {}
                div {
                    button {
                        onclick: move |_| { factor2 += 1; },
                        "+"
                    }
                }
                div {}

                // Middle row
                div {
                    "{factor1}"
                }
                div {
                    dangerous_inner_html: "&times;"
                }
                div {
                    "{factor2}"
                }
                div {
                    opacity: "{opacity}",
                    "{answer}"
                }

                // Bottom row
                div {
                    button {
                        onclick: move |_| { factor1 -= 1; },
                        "-"
                    }
                }
                div {}
                div {
                    button {
                        onclick: move |_| { factor2 -= 1; },
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
        pub factor1: i32,
        pub factor2: i32,
    }

    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct MultiplyResponse {
        pub product: i32,
    }

    pub async fn multiply(factor1: i32, factor2: i32) -> Result<i32, std::io::Error> {
        let location = ::web_sys::window().unwrap().location().origin().unwrap();
        let mut url = reqwest::Url::parse(&location).map_err(std::io::Error::other)?;
        url.set_path("api/multiply");
        let query = serde_urlencoded::to_string(MultiplyRequest { factor1, factor2 })
            .map_err(std::io::Error::other)?;
        url.set_query(Some(&query));
        let response = reqwest::get(url).await.map_err(std::io::Error::other)?;

        if !response.status().is_success() {
            return Err(std::io::Error::other(response.status().to_string()));
        }

        let multiplication = response
            .json::<MultiplyResponse>()
            .await
            .map_err(std::io::Error::other)?;

        Ok(multiplication.product)
    }
}

#[cfg(feature = "server-fn")]
pub mod server_function {
    use server_fn::ServerFnError;
    use server_fn_macro_default::server;

    #[server(endpoint = "/multiply")]
    pub async fn multiply(factor1: i32, factor2: i32) -> Result<i32, ServerFnError> {
        match factor1.checked_mul(factor2) {
            Some(product) => Ok(product),
            None => Err(ServerFnError::new("overflow")),
        }
    }
}
