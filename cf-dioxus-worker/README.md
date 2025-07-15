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

Then the following steps were performed:

Update the `wrangler.toml` file to build the Dioxus bundle before deploying the
worker by adding `./dioxus-build &&` to the build command. The `dioxus-build`
script adds some files to the root path to prevent the Worker being activated
unnecessarily.

Add an `assets` table to the `wrangler.toml` file:
```toml
[assets]
directory = "../cf-dioxus/target/dx/cf-dioxus/release/web/public"
binding = "ASSETS"
not_found_handling = "single-page-application"
```

Add your Dioxus project directory as dependencies in the worker `Cargo.toml`:
```toml
cf-dioxus = { path = "../cf-dioxus" }
```

Change `src/lib.rs` to return static assets if the Worker is invoked. For most
browser requests the static assets will be served without Worker invocation.

## Deploy a local dev site

Run
```
$ npx wrangler dev
```
and [test the code in the browser](http://localhost:8787/). Press Ctrl-C to exit.

## Deploy to Cloudflare Pages

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
