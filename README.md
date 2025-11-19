# compile-typst-site

`compile-typst-site` is a command-line program for static site generation using Typst. It takes a directory structure like this:

```
.
├── compile-typst-site.toml
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

and after running `compile-typst-site` in the directory, it generates a `_site` like this:

```
.
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
└── ...
```

For more, visit the docs: <https://wade-cheng.com/compile-typst-site/>.
