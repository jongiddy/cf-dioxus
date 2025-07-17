# Hosting Dioxus on Cloudflare Workers

All of the deployments assume that you have installed the [Dioxus CLI (`dx`)](https://dioxuslabs.com/learn/0.6/getting_started/) and the [Cloudflare Wrangler command (`npx wrangler`)](https://developers.cloudflare.com/workers/wrangler/install-and-update/).

This repo consists of the following directories:

- `cf-dioxus`: A sample Dioxus project. See the site in action at https://cf-dioxus.pages.dev/
- `cf-dioxus-pages`: Deployment of the Dioxus project on Cloudflare Pages (no SSR or API calls).
- `cf-dioxus-worker`: Deployment of the Dioxus project on Cloudflare Workers (no SSR or API calls) - this corresponds to the Cloudflare Pages deployment.
- `cf-dioxus-http-api`: Deployment of the Dioxus project on Cloudflare Workers using `http` to serve an API.
- `cf-dioxus-axum-api`: Deployment of the Dioxus project on Cloudflare Workers using `axum` to serve an API.

The deployments here keep the Dioxus project separate from the Cloudflare
deployment method. This allows easy application of alternative methods. To make
this work more cleanly, most of the Dioxus code is moved into a `lib.rs` file,
with just the `main` function remaining in the `main.rs` file. This allows the
deployments to use the Dioxus project as a library.

The deployments use `release` mode because Cloudflare Pages and Workers have an
individual file size limit of 25 MiB. The `debug` builds of Dioxus projects
quickly exceed this limit.

On a Cloudflare Free plan all these deployments will be free of any costs. A
free site will have a [limit on the number of Worker requests served per day](https://developers.cloudflare.com/workers/platform/pricing/#workers). Worker requests include [Dioxus server functions](https://dioxuslabs.com/learn/0.6/guides/fullstack/server_functions/).
Static assets, including the Dioxus bundle assets, are free and unlimited on any plan.

For a more professional site, you may want to purchase a custom domain and
consider a paid plan to avoid the request limit.
