## Support for server functions

The standard Dioxus approach to server functions does not work on Cloudflare
Workers. Feature `dioxus/fullstack` enables `axum/ws` which enables `axum/tokio`
which enables `tokio/net` which enables `mio/net` which is incompatible with WASM.

However we can use the underlying crate `server_fn` directly.

A limitation of this deployment is that server functions must have an explicit
name using the `#[server(endpoint = "...")]` attribute.

This directory was created using the command
```sh
$ cargo generate cloudflare/workers-rs
  Which template should be expanded? · templates/axum
  Project Name: cf-dioxus-server-fn
```

## Changes

Add the Dioxus project directory as a dependency in the worker `Cargo.toml`.
Enable the API by adding the `server-fn` feature.

```toml
cf-dioxus = { path = "../cf-dioxus", features = ["server-fn"] }
```

Add the `server-fn` dependencies:

```toml
server_fn = { version = "0.8", features = ["axum-no-default"] }
server_fn_macro_default = { version = "0.8", features = ["axum", "ssr"] }
```

Update the `wrangler.toml` file to build the Dioxus client before deploying the
worker. Enable the `server-fn` feature when building the Dioxus client. In the
`assets` table add the path to the client bundle and enable Single Page
Application mode with an exception for the API paths. Enable logs.

```toml
[build]
command = "( cd ../cf-dioxus && ./dioxus-build --features server-fn ) && cargo install -q worker-build && worker-build --release"

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
