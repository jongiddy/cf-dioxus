# Various ways to host Dioxus on Cloudflare.

All of the installations assume that you have installed the [Dioxus CLI (`dx`)](https://dioxuslabs.com/learn/0.6/getting_started/) and the [Cloudflare Wrangler command (`npx wrangler`)](https://developers.cloudflare.com/workers/wrangler/install-and-update/).

This repo consists of the following directories:

- `cf-dioxus`: A sample Dioxus project. You can see the site in action at https://cf-dioxus.pages.dev/
- `cf-dioxus-pages`: Installation of the Dioxus project in Cloudflare Pages (no SSR or API calls)

Most installations use `release` mode. This is because Cloudflare Pages and
Workers have an individual file size limit of 25 MiB. The `debug` builds of
Dioxus projects quickly exceed this limit.

On a Cloudflare Free plan all these installations will be free of any costs. A
free site will have a [limit on the number of Worker requests served per day](https://developers.cloudflare.com/workers/platform/pricing/#workers). Worker requests include [Dioxus server functions](https://dioxuslabs.com/learn/0.6/guides/fullstack/server_functions/).
Static assets, including the Dioxus bundle assets, are free and unlimited on any plan.

For a more professional site, you may want to purchase a custom domain and consider a paid plan to avoid the request limit.
