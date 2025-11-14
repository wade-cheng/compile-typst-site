# Implementing common file structures

In short, anything that you can implement in native Typst is easy, because you just do that and then run `compile-typst-site`. If you can't do something in native Typst, you will have to add third party scripts to your workflow. They can run automatically when added to the config file.

## Hardcoded internal links

The simplest possible layout that sees benefit from using `compile-typst-site` has the following characteristics:

- multiple Typst source files (or just one??) [^1]
- common template(s)
- stylesheets or other assets to pass through
- hardcoded links between pages

[^1]: If you had just one, you could just as well run `typst compile` over and over. Maybe our incantation, `typst-compile-site`, is just easier to remember?

For example:

```text
.
├── compile-typst-site.toml
├── src/
│   ├── about.typ
│   ├── blog.typ
│   ├── index.typ
│   └── style.css
└── templates/
    └── base.typ
```

This layout is super easy to deal with because all the links between pages are hardcoded. For example, notice how our template in [](../tutorials/beginner/03_templates.md) includes a navigation bar with hardcoded links to each page.

To use this layout, all you need to do is specify what you need to copy over:

```toml
# compile-typst-site.toml
passthrough_copy = [
    "style.css",
]
```

Then, running the command `compile-typst-site` under your project root directory will compile all Typst files in `src`[^2] and pass through the stylesheets and other assets you've specified.

[^2]: This hooks in templates automatically by the definition of how `typst compile` works.

## Dynamic blog (or other collection) page

In addition to the simple structure with hardcoded links, you may want a dynamic page that lists all your blog posts.

For example:

```text
.
├── compile-typst-site.toml
├── src/
│   ├── blog/
│   │   ├── post-1.typ
│   │   ├── post-2.typ
│   │   ├── post-3.typ
│   │   └── ...
│   ├── blog.typ  <--- Has titles and descriptions of all blog posts
│   ├── index.typ
│   └── style.css
└── templates/
    └── base.typ
```

Unfortunately, until [typst/issues/2123](https://github.com/typst/typst/issues/2123) is implemented, this can't be done in native Typst.

### Solution 1: `file_listing`

You can set `file_listing` in the configuration file to `"enabled"` or `"include-data"`, and read the JSON file it creates from Typst. See the full example.

Notice the way we set up the template file to let all its templatees be queried:

```typst
#let conf(
  page-title: "",
  date: "",
  doc,
) = {
  [#metadata(
    (
      "page-title": page-title,
      "date": date
    )
  ) <data>]

  // ...
}
```

and how we use that in the blog page:

```typst
#let files = json("../files.json")

#for (path, queried) in files.pairs() [
  #if queried.len() > 0 and path.contains("/blog/"){
    let path = (path
      .split("/blog/")
      .at(-1)
      .replace(regex("\\.typ$"), "/"))
    let page = queried.at(0).at("value")

    html.p[
      #html.a(href: path)[#page.page-title]
      #html.span(class: "date")[
        #utils.format-date(page.date)
      ]
    ]
  }
]
```

### Solution 2: `init` script

Before `file_listing` was implemented, you could create its functionality manually. Depending on your own script may still be desired if you require some feature we don't have.

You can write a script that

- reads all the titles and descriptions in your blog posts, maybe with `typst query`
- writes it to some file
- preferably with some [data format](https://typst.app/docs/reference/data-loading/) Typst can read, like JSON or TOML.

Then, the `blog.typ` would read that data file and use it to list off the blog posts. It might look something like this:

```typst
// somewhere in blog.typ
#let posts = json("../posts.json")

#for post in posts.blog [
  #html.p[
    #html.a(href: post.path)[#post.pagetitle].
    post.description
  ]
]
```

To run your script every time you update a template or compile the full site, add it to your config file:

```toml
# compile-typst-site.toml
init = ['python', 'list-collections.py']
```
