use cf_dioxus::api::{MultiplyRequest, MultiplyResponse};
use futures::stream;
use worker::{event, Context, Env};

#[event(fetch)]
async fn fetch(
    req: http::Request<worker::Body>,
    env: Env,
    _ctx: Context,
) -> worker::Result<http::Response<worker::Body>> {
    console_error_panic_hook::set_once();

    let uri = req.uri();

    match uri.path() {
        "/api/multiply" => {
            // Returning `Err` produces a 500 Internal Server Error. Construct
            // an `OK` response if a different status code is required.
            if req.method() != http::Method::GET {
                return Ok(http::Response::builder()
                    .status(http::StatusCode::METHOD_NOT_ALLOWED)
                    .body(worker::Body::empty())?);
            }
            let Some(query) = uri.query() else {
                return Ok(http::Response::builder()
                    .status(http::StatusCode::BAD_REQUEST)
                    .body(worker::Body::from_stream(stream::once(async {
                        Ok::<_, worker::Error>("expected query parameters")
                    }))?)?);
            };
            let Ok(request) = serde_urlencoded::from_str::<MultiplyRequest>(query) else {
                return Ok(http::Response::builder()
                    .status(http::StatusCode::BAD_REQUEST)
                    .body(worker::Body::empty())?);
            };
            let result = request.a * request.b;
            let body = serde_json::to_string(&MultiplyResponse { result })?;
            Ok(http::Response::builder()
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(worker::Body::from_stream(stream::once(async {
                    Ok::<_, worker::Error>(body)
                }))?)?)
        }
        path if path.starts_with("/api/") => {
            return Ok(http::Response::builder()
                .status(http::StatusCode::NOT_FOUND)
                .body(worker::Body::empty())?)
        }
        _ => {
            // Usually static resources will be returned without invoking the
            // worker. However, non-browser requests may invoke the worker.
            // This performs the `single-page-application` behavior of returning
            // the named static asset or, if not found, the `index.html` file.
            env.assets("ASSETS")?.fetch(uri.to_string(), None).await
        }
    }
}
