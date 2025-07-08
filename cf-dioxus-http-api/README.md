Support for API calls (nto server functions)

This directory was created using the command
```sh
$ cargo generate cloudflare/workers-rs
  Which template should be expanded? · templates/hello-world-http
  Project Name: cf-dioxus-http-api
```

Changes to the `cf-dioxus-worker` configuration:

- add the `api` feature to the cf-dioxus dependency to provide access to the types used for the API:
```toml
cf-dioxus = { path = "../cf-dioxus", features = ["api"] }
```
- add some additional crates for the API implementation:
```toml
futures = "0.3"
serde_json = "1"
serde_urlencoded = "0.7"
```

- add the `api` feature when building the cf-dioxus client:
```
dx bundle --release --platform web --features api
```

- add code to `src/lib.rs` to handle the API calls.

- add `run_worker_first = [ "/api/*" ]` to `wrangler.toml` to ensure that API calls go to the worker
rather than returning the index page.

## Deploy a local dev site

Run
```sh
$ npx wrangler dev
```
and [test the code in the browser](http://localhost:8787/). Press Ctrl-C to exit.

## Deploy to Cloudflare Pages

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
