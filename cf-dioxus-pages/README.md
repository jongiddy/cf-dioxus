Cloudflare Pages are an easy way to deploy a self-contained Dioxus app that does
not make calls back to the origin.

You must manually build the Dioxus project before deployment to Cloudflare Pages.
An advantage of using Cloudflare Workers is that they can be configured to
automatically build the Dioxus project, ensuring that the most recent build is
always deployed.

# Building for Cloudflare Pages

## Create a Cloudflare Pages project

```sh
$ npx wrangler pages project create cf-dioxus-pages

───────────────────
✔ Enter the production branch name: … production
✨ Successfully created the 'cf-dioxus-pages' project. It will be available at https://cf-dioxus-pages-4ax.pages.dev/ once you create your first deployment.
To deploy a folder of assets, run 'wrangler pages deploy [directory]'.
```

This only needs to be done once. The production branch name is not important because we are deploying using [direct upload](https://developers.cloudflare.com/pages/get-started/direct-upload/).

## Build the Dioxus bundle in release mode

In the Dioxus project directory (`cf-dioxus`) run
```sh
$ dx bundle --release --web
```

The last line shows the path for the assets:
```
 INFO Bundled app at: /home/user/my-site/target/dx/my-site/release/web/public
```

The bundle command must be run after any changes to the Dioxus project.
Cloudflare Workers projects can be configured to do this automatically.

## Create a `wrangler.toml` file

Create a `wrangler.toml` file with the name of the Cloudflare Pages project and
the path from the `dx bundle` command. You can trim the absolute path name to be
relative to the `wrangler.toml` file.

See the example `wrangler.toml` file in this directory.

## Deploy a local dev site

Run
```sh
$ npx wrangler pages dev
```
and test the code in the browser. Press x to exit.

## Deploy to Cloudflare Pages

Run
```sh
$ npx wrangler pages deploy
```

This will provide a link to a pages.dev website.

To publish the site to a non-dev domain go to your Cloudflare account, click on Compute (Workers) then click on the pages site (e.g. cf-dioxus-pages), click on Custom Domains, and add a domain from your Cloudflare-managed domains (e.g. `www.example.com/*`).
