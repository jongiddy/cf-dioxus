A website to multiply two numbers.

### Changes

Turn off generation of pre-compressed files in `Dioxus.toml` as Cloudflare Pages and Cloudflare
Workers do not use them:
```toml
[web]
pre_compress = false
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```
