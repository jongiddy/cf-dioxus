# Hosting Dioxus on Cloudflare Workers

This repo consists of the following directories:

- `cf-dioxus`: A sample Dioxus project. See the site in action at https://cf-dioxus.pages.dev/
- `cf-dioxus-pages`: Deploy the Dioxus project on Cloudflare Pages (no SSR or API calls).
- `cf-dioxus-worker`: Deploy the Dioxus project on Cloudflare Workers (no SSR or API calls) - this corresponds to the Cloudflare Pages deployment.
- `cf-dioxus-http-api`: Deploy the Dioxus project on Cloudflare Workers using `http` to serve an API.
- `cf-dioxus-axum-api`: Deploy the Dioxus project on Cloudflare Workers using `axum` to serve an API.
- `cf-dioxus-server-fn`: Deploy the Dioxus project on Cloudflare Workers with [server functions](https://dioxuslabs.com/learn/0.6/guides/fullstack/server_functions/).

These deployments use the Dioxus `main` branch that will become Dioxus v0.7. For Dioxus v0.6 change to the `dioxus-0.6` branch.

The deployments assume that you have installed the development build of [Dioxus CLI (`dx`)](https://github.com/DioxusLabs/dioxus/tree/main/packages/cli#install-the-latest-development-build-through-git)
and the [Cloudflare Wrangler command (`npx wrangler`)](https://developers.cloudflare.com/workers/wrangler/install-and-update/).
There is a `package-lock.json` file in the top-level directory containing the versions of Wrangler
and its dependencies with which these deployments are tested. The command `npm ci` will install the same versions.

The deployments keep the Dioxus project separate from the Cloudflare
deployment method. This allows easy application of alternative methods. To make
this work more cleanly, most of the Dioxus code is moved into a `lib.rs` file,
with just the `main` function remaining in the `main.rs` file. This allows the
deployments to use the Dioxus project as a library.

The deployments use `release` mode because Cloudflare Pages and Workers have an
individual file size limit of 25 MiB. The `debug` builds of Dioxus projects
quickly exceed this limit.

On a Cloudflare Free plan all these deployments will be free of any costs. A
free site will have a [limit on the number of Worker requests served per day](https://developers.cloudflare.com/workers/platform/pricing/#workers).
Worker requests include API calls and Dioxus server functions. Static assets,
including the Dioxus bundle assets, are free and unlimited on any plan.

For a more professional site, you may want to purchase a custom domain and
consider a paid plan to avoid the request limit.
