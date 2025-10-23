A website to multiply two numbers.

### Changes

Turn off generation of pre-compressed files in `Dioxus.toml` as Cloudflare Pages and Cloudflare
Workers do not use them:
```toml
[application]
public_dir = "public"

[web]
pre_compress = false
```

Files in the `public_dir` directory will be copied into the site directory, next
to `index.html`. This can be used, for example, to add a `/favicon.ico` or
`/robots.txt` path.

Cloudflare Pages and Cloudflare Workers use special files in the top-level
static asset directory to allow configuration of additional headers and
redirects for static assets.

Additional headers are specified using the `_headers` file. See
https://developers.cloudflare.com/workers/static-assets/headers/

For example, files in the `assets` directory include a content-based hash in the
name. To add a `Cache-Control` header to allow caches to provide these files for
up to 6 months without revalidating create a `_headers` file:
```
/assets/*
    Cache-Control: public, max-age=15552000, immutable
```

Redirects are specified using the `_redirects` file. See
https://developers.cloudflare.com/workers/static-assets/redirects/

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```
