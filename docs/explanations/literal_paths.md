# To literal-path or not?

By default, `compile-typst-site` will convert `path/file.typ` files to `path/file/index.html`.

When browsers are asked to visit `some-path/`, they will look for `some-path/index.html`. So, this conversion lets you see `path/file.typ` on the browser at `path/file/`. For example, notice how you visit <https://google.com>, not <https://google.com/index.html>---even though both work.

Some people think this looks nice. If you agree, leave the setting on. If you want vistors to use `path/file.html` instead, set `literal_paths = true` in your config file.
