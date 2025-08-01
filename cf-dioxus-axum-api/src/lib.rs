use std::sync::LazyLock;

use axum::{extract::Query, http, routing::get, Extension, Json};
use cf_dioxus::api::{MultiplyRequest, MultiplyResponse};
use tower_service::Service as _;
use worker::{event, Context, Env};

#[event(fetch)]
async fn fetch(
    mut req: http::Request<worker::Body>,
    env: Env,
    _ctx: Context,
) -> worker::Result<http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    req.extensions_mut().insert(env);

    Ok(ROUTER.clone().call(req).await?)
}

static ROUTER: LazyLock<axum::Router> = LazyLock::new(|| {
    axum::Router::new()
        .nest(
            "/api",
            axum::Router::new()
                .route("/multiply", get(multiply))
                .fallback(async || http::StatusCode::NOT_FOUND),
        )
        .fallback(static_asset_or_index_html)
});

async fn multiply(
    request: Query<MultiplyRequest>,
) -> Result<Json<MultiplyResponse>, http::StatusCode> {
    match request.factor1.checked_mul(request.factor2) {
        Some(product) => Ok(Json(MultiplyResponse { product })),
        None => Err(http::StatusCode::BAD_REQUEST),
    }
}

// Handlers that extract `Extension<Env>` require `#[worker::send]`.
// See https://github.com/cloudflare/workers-rs/issues/485
#[worker::send]
async fn static_asset_or_index_html(
    uri: http::Uri,
    Extension(env): Extension<Env>,
) -> Result<http::Response<axum::body::Body>, (http::StatusCode, String)> {
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
