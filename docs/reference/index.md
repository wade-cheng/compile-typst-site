# Reference

Nitty-gritty technical descriptions of how `compile-typst-site` works. Most useful when you
need detailed information about usage, requirements, and inner workings.

## Concepts

`compile-typst-site` expects to be called somewhere under the following directory structure:

```
.
├── src/
├── templates/
└── compile-typst-site.toml
```

That is, just like `cargo`, `just`, `uv`, and so on, you can use `compile-typst-site` while in a subdirectory of the project root.

The directory `.` (one that contains a `compile-typst-site.toml` file) is known as the "project root."

When you do so, it looks at every file in `src`. For each such file, one of the following happens, checked in the following order:

- Files matching those in the `passthrough_copy` array in `compile-typst-site.toml` are copied over. Matching can use globs. Files are rooted in the content `src` directory, not the project root.
- Typst files are compiled by calling your local Typst CLI; we expect one to be installed.
- Other files are ignored.

If file watching is turned on, changes in `src` will only recompile that file. Changes in `templates` will recompile the entire project (all of `src`). We aren't smart enough to detect exactly which dependents to recompile.

## Config File API

The configuration file at `compile-typst-site.toml` is specified as such:

```{literalinclude} config_struct.rs
:language: rust
```

## CLI API

```{literalinclude} cts_help.txt
:language: text
```

When serve or watch mode is on, errors are demoted to warnings to prevent, for example, temporary compilation errors from crashing the mode.
