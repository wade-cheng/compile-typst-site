# Ignoring local fonts

You can tell `compile-typst-site` to not scan for local fonts by appending custom arguments to the internal Typst calls:

```toml
# compile-typst-site.toml

# Don't scan for or use system fonts for internal `typst compile` invocations.
compilation_extra_args = ["--ignore-system-fonts"]

# If you use the `file_listing = "include-data"` setting,
# we run a bunch of `typst query`s under the hood.
# You're able to modify those as well.
file_listing_extra_args = ["--ignore-system-fonts"]
```

Typst looks for fonts by default, because as a program dedicated to good paged output, users will often use fonts.[^1] But since HTML output from Typst often doesn't have any font information at all, disabling this step can speed up `compile-typst-site` compilation times.

[^1]: Typst [defines](https://typst.app/docs/reference/foundations/target/) paged output as PDF, PNG, and SVG export. People use paged output when they want careful control of exactly what a document looks like. For example: the width and height of the document---something HTML isn't even designed to represent.

At some point in the future, Typst will try to lazily scan fonts: [typst/7380](https://github.com/typst/typst/pull/7380). That is, it will only scan for fonts if a document uses external fonts.
