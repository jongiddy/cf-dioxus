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

Changes to the `cf-dioxus-worker` configuration:

- add some additional crates for the server_fn implementation:
```toml
server_fn = { version = "0.8", features = ["axum-no-default"] }
server_fn_macro_default = { version = "0.8", features = ["axum", "ssr"] }
```

- add the `server-fn` feature to the cf-dioxus dependency to provide access to the types used for the API:
```toml
cf-dioxus = { path = "../cf-dioxus", features = ["server-fn"] }
```

- add the `server-fn` feature when building the cf-dioxus client:
```sh
dx bundle --release --platform web --features server-fn
```

- add code to `src/lib.rs` to handle the server function calls.

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
