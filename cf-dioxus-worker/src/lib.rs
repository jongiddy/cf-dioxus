use worker::*;

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> Result<HttpResponse> {
    console_error_panic_hook::set_once();

    let uri = req.uri();

    // Usually static resources will be returned without invoking the
    // worker. However, non-browser requests may invoke the worker.
    // This performs the `single-page-application` behavior of returning
    // the named static asset or, if not found, the `index.html` file.
    env.assets("ASSETS")?.fetch(uri.to_string(), None).await
}
