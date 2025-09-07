A simple worker that mimics the Cloudflare Pages installation, serving static assets only.

The advantages of this approach over Pages include:
- the Dioxus project is bundled before deployment, ensuring that you see the latest build.
- it is simple to add API or server functions later.

This directory was created using the command
```
$ cargo generate cloudflare/workers-rs
  Which template should be expanded? · templates/hello-world-http
  Project Name: cf-dioxus-worker
```

## Changes

Add the Dioxus project directory as a dependency in the worker `Cargo.toml`.

```toml
cf-dioxus = { path = "../cf-dioxus", default-features = false }
```

Update the `wrangler.toml` file to build the Dioxus client before deploying the
worker. In the `assets` table add the path to the client bundle and enable
Single Page Application mode. Enable logs.

```toml
[build]
command = "( cd ../cf-dioxus && ./dioxus-build ) && cargo install -q worker-build && worker-build --release"

[assets]
directory = "../cf-dioxus/target/dx/cf-dioxus/release/web/public"
binding = "ASSETS"
not_found_handling = "single-page-application"

[observability.logs]
enabled = true
```

## Deploy a local dev site

Run
```
$ npx wrangler dev
```
and [test the code in the browser](http://localhost:8787/). Press Ctrl-C to exit.

## Deploy to Cloudflare Workers

Run
```
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
