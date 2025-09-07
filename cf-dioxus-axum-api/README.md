## Support for API calls (not server functions)

This directory was created using the command
```sh
$ cargo generate cloudflare/workers-rs
  Which template should be expanded? · templates/axum
  Project Name: cf-dioxus-axum-api
```

## Changes

Add the Dioxus project directory as a dependency in the worker `Cargo.toml`.
Enable the API by adding the `api` feature.

```toml
cf-dioxus = { path = "../cf-dioxus", default-features = false, features = ["api"] }
```

Add Axum features for the API implementation:
```toml
axum = { version = "0.8", default-features = false, features = ["json", "query"]}
```

Update the `wrangler.toml` file to build the Dioxus client before deploying the
worker. Enable the `api` feature when building the Dioxus client. In the `assets`
table add the path to the client bundle and enable Single Page Application mode
with an exception for the API paths. Enable logs.

```toml
[build]
command = "( cd ../cf-dioxus && ./dioxus-build --features api ) && cargo install -q worker-build && worker-build --release"

[assets]
directory = "../cf-dioxus/target/dx/cf-dioxus/release/web/public"
binding = "ASSETS"
not_found_handling = "single-page-application"
run_worker_first = [ "/api/*" ]

[observability.logs]
enabled = true
```

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
