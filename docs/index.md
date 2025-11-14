# compile-typst-site

`compile-typst-site` is a binary utility for static site generation using Typst. It takes a directory structure like this:

````{tab} Linux/Mac
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
````

````{tab} Windows
```
> tree
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
````

and generates a `_site` like this:

````{tab} Linux/Mac
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
````

````{tab} Windows
```
> compile-typst-site
INFO  [compile_typst_site] compiled project from scratch

> tree
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
````

## Continue Reading

For more information, visit:

- [](./tutorials/beginner/index.md): start learning
- [](./how-to/quick_start.md): jump right in
- [](./explanations/why_us.md): a comparison of options in the Typst site generation space

```{toctree}
:hidden:

tutorials/index
how-to/index
explanations/index
reference/index
```

```{toctree}
:caption: About
:hidden:

contributing
compile-typst-site on crates.io <https://crates.io/crates/compile-typst-site/>
compile-typst-site on GitHub <https://github.com/wade-cheng/compile-typst-site/>
```
