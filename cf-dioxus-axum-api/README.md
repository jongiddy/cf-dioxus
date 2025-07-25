## Support for API calls (not server functions)

This directory was created using the command
```sh
$ cargo generate cloudflare/workers-rs
  Which template should be expanded? · templates/axum
  Project Name: cf-dioxus-axum-api
```

Changes to the `cf-dioxus-worker` configuration:

- add the `api` feature to the cf-dioxus dependency to provide access to the types used for the API:
```toml
cf-dioxus = { path = "../cf-dioxus", features = ["api"] }
```

- add some additional features for the API implementation:
```toml
axum = { version = "0.8", default-features = false, features = ["query", "json"]}
```

- add the `api` feature when building the cf-dioxus client:
```sh
./dioxus-build --features api
```

- add code to `src/lib.rs` to handle the API calls. Once the `http` crate is available, it is
generally easier to expand the type aliases `worker::HttpRequest` to `http::Request<worker::Body>`
and `worker::HttpResponse` to `http::Response<worker::Body>`.

- add `run_worker_first = [ "/api/*" ]` to `wrangler.toml` to ensure that API calls go to the worker
rather than returning the index page.

## Deploy a local dev site

Run
```sh
$ npx wrangler dev
```
and [test the code in the browser](http://localhost:8787/). Press Ctrl-C to exit.

## Deploy to Cloudflare Workers

Run
```sh
$ npx wrangler deploy
```

This will provide a link to a workers.dev website.

To publish the site to a non-dev domain add a route to `wrangler.toml` with a
custom domain from your Cloudflare-managed domains.
```toml
routes = [
  { pattern = "cf-dioxus.example.com", custom_domain = true }
]
```
