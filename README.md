# compile-typst-site

> [!CAUTION]
> This is an alpha release. Consider pinning to a commit for stability.

`compile-typst-site` is a binary utility for static site generation using Typst. It handles passing binary data like CSS files and compilation of Typst files to HTML. It can watch for changes and only recompile Typst files that have changed, or the entire project if a template has been changed.

To be specific, we do not supply a templating engine. The use case for one can be solved with native Typst. _All this CLI does is try to copy and compile files in a smart way_.

## installation

This is alpha software.

[Install Typst](https://typst.app/open-source/#download) and run

```
cargo install --git https://github.com/wade-cheng/compile-typst-site.git
```

## examples

With `just` and `python` installed, `cd` to `examples/typst-site-full` and use `compile-typst-site`. You will need to supply an HTML server. Try `python -m http.server`---the site will then be available at <http://localhost:8000/>.

## reference

`compile-typst-site` expects to be called somewhere under the following directory structure:

```
.
├── src/
├── templates/
└── compile-typst-site.toml
```

That is, just like `cargo`, `just`, `uv`, and so on, you can use the binary while in a subdirectory for the project root.

When you do so, it compiles every Typst file in `src` by calling your local Typst CLI, so you should have one installed. It also copies over files in the `passthrough_copy` array in `compile-typst-site.toml`. This uses globs, processed with the `globset` crate.

If file watching is turned on, changes or additions to `src` will only recompile that file. Changes or additions to `templates` will recompile the entire project.

See `compile-typst-site --help` and the example for more details.
