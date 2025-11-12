# compile-typst-site

`compile-typst-site` is a binary utility for static site generation using Typst. It's built to be dead simple (at least conceptually). Let's explain and motivate its use case with an example. Say we've created a directory tree that maps to a website:

```
$ tree
.
├── src/
│   ├── about.typ
│   ├── blog/
│   │   ├── post-1.typ
│   │   ├── post-2.typ
│   │   ├── post-3.typ
│   │   └── ...
│   ├── blog.typ
│   ├── data.json
│   ├── index.typ
│   └── style.css
└── templates/
    └── base.typ
```

Converting this to a website by hand would mean running `typst compile` on `index.typ`, `about.typ`, the first blog post, the second, and so on and so on. Also, our index file uses `data.json` to draw something cool on the home page, but we don't need it after that. If I want to generate the site into another folder, clean of unnecessary data, I would need to remember to copy over `style.css` but not `data.json`.

All `compile-typst-site` does is automate this process. It'll compile all `.typ` files it finds, and you can supply a configuration file to tell it what else to copy over. After adding such a configuration file at `compile-typst-site.toml`, we can compile the project and inspect our newly built `_site`:

```
$ compile-typst-site
INFO  [compile_typst_site] compiled project from scratch

$ tree
.
├── compile-typst-site.toml
├── _site/
│   ├── about/
│   │   └── index.html
│   ├── blog/
│   │   ├── index.html
│   │   ├── post-1/
│   │   │   └── index.html
│   │   ├── post-2/
│   │   │   └── index.html
│   │   ├── post-3/
│   │   │   └── index.html
│   │   └── ...
│   ├── index.html
│   └── style.css
├── src/
│   ├── about.typ
│   ├── blog/
│   │   ├── post-1.typ
│   │   ├── post-2.typ
│   │   ├── post-3.typ
│   │   └── ...
│   ├── blog.typ
│   ├── data.json
│   ├── index.typ
│   └── style.css
└── templates/
    └── base.typ
```

`compile-typst-site` can also watch your project directory. If a source file is changed, it'll only recompile that file. If a template file is changed, it'll recompile all of its dependents.

To be clear, we do not supply a templating engine. Native Typst is able to be your templating engine. All this CLI does is free you from calling `typst compile` one bajillion times. We also do not supply an HTTP server for you to view your generated files. You may need one to view your site locally, but if you are uploading your files straight to GitHub Pages, Neocities, etc, you will not.

## installation

You must have Typst [installed](https://typst.app/open-source/#download).

After Typst is installed, use one of the `compile-typst-site` installation methods below.

### precompiled binary (for beginners)

See the [releases](https://github.com/wade-cheng/compile-typst-site/releases) to install from a precompiled binary.

### cargo-binstall

If you know what this is, yes, we support this.

```
cargo binstall compile-typst-site
```

### compile from source

You probably know what you're doing.

```
# from github
cargo install --git https://github.com/wade-cheng/compile-typst-site.git
```

```
# from latest release on crates.io
cargo install compile-typst-site
```

## examples

The full example additionally requires `just` and `python` to run preprocessing and postprocessing. With them installed, `cd` to `examples/typst-site-full` and use `compile-typst-site`. You will need to supply an HTML server. Try `python -m http.server --directory _site`. The site will then be available at <http://localhost:8000/>.

## reference documentation

`compile-typst-site` expects to be called somewhere under the following directory structure:

```
.
├── src/
├── templates/
└── compile-typst-site.toml
```

That is, just like `cargo`, `just`, `uv`, and so on, you can use `compile-typst-site` while in a subdirectory of the project root.

When you do so, it looks at every file in `src`. One of the following happens:

- Typst files are compiled by calling your local Typst CLI; we expect one to be installed.
- Files matching those in the `passthrough_copy` array in `compile-typst-site.toml` are copied over. Matching can use globs, processed with the `globset` crate. This is probably unidiomatic to file paths (`*` matches recursively instead of just the first layer).
- Other files are ignored.

If file watching is turned on, changes in `src` will only recompile that file. Changes in `templates` will recompile the entire project (all of `src`). We aren't smart enough to detect exactly which dependents to be changed.

See `compile-typst-site --help` and the example for more details.

The config file at `compile-typst-site.toml` is specified as such:

```rust
struct ConfigFile {
    /// Array of globs to match for passthrough-copying.
    ///
    /// Example in the TOML config file: `passthrough_copy = ["*.css", "*.js", "assets/*"]
    passthrough_copy: Option<Vec<String>>,
    /// Command to run before a full rebuild.
    ///
    /// E.g., `passthrough_copy = ["echo", "rebuilding"]`.
    init: Option<Vec<String>>,
    /// Command to run to post-process HTML files generated by Typst.
    ///
    /// Must take in stdin and return via stdout.
    ///
    /// Example in the TOML config file: `post_processing_typ = ["python", "post_processing_script.py"]`.
    post_processing_typ: Option<Vec<String>>,
    /// Convert paths literally instead of magically tranforming to index.html.
    ///
    /// i.e., ./content.typ goes to ./content.html instead of defaulting to ./content/index.html.
    ///
    /// Example in the TOML config file: `literal_paths = true`
    literal_paths: Option<bool>,
    /// Typst cannot yet glob-find multiple files, which is a problem if one wants to list, e.g., all blog posts on a page.
    /// To work around this, we write all Typst files(?) as a JSON to the project root directory.
    ///
    /// We also let you query for data. (You might want the dates of those blog posts to appear on your listing page).
    /// This is slower than the other options because we have to call `typst query`.
    ///
    /// Must be one of "disabled", "enabled", "include-data"
    ///
    /// Example in the TOML config file: `file_listing = "enabled"`
    file_listing: Option<String>,
}
```
