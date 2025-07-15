use std::sync::LazyLock;

use axum::{extract::Query, http, routing::get, Extension, Json};
use cf_dioxus::api::{MultiplyRequest, MultiplyResponse};
use tower_service::Service;
use worker::*;

#[event(fetch)]
async fn fetch(
    mut req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    req.extensions_mut().insert(env);

    Ok(ROUTER.clone().call(req).await?)
}

static ROUTER: LazyLock<axum::Router> = LazyLock::new(|| {
    axum::Router::new()
        .route("/api/multiply", get(multiply))
        .fallback(fallback)
});

async fn multiply(request: Query<MultiplyRequest>) -> Json<MultiplyResponse> {
    let result = request.a * request.b;
    Json(MultiplyResponse { result })
}

#[worker::send] // See https://github.com/cloudflare/workers-rs/issues/485
async fn fallback(
    uri: http::Uri,
    Extension(env): Extension<Env>,
) -> std::result::Result<http::Response<axum::body::Body>, (http::StatusCode, String)> {
    // Usually static resources will be returned without invoking the
    // worker. However, non-browser requests may invoke the worker.
    // This performs the `single-page-application` behavior of returning
    // the named static asset or, if not found, the `index.html` file.
    let response = env
        .assets("ASSETS")
        .map_err(|e| (http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .fetch(uri.to_string(), None)
        .await
        .map_err(|e| (http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    // Wrap the `worker::Body` in an `axum::body::Body`.
    Ok(response.map(axum::body::Body::new))
}
