# compile-typst-site

`compile-typst-site` is a binary utility for static site generation using Typst. It takes a directory structure like this:

```
$ tree
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

and generates a `_site` like this:

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
│   └── ...
└── templates/
    └── ...
```

Read more at the docs: <https://wade-cheng.com/compile-typst-site/>.
