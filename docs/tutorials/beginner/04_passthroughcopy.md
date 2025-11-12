# Part 4: Passthroughcopying files

```toml
# compile-typst-site.toml
passthrough_copy = [
    "style.css",
]
```

```css
/** style.css **/
body {
  margin-left: 10vw;
}

nav a {
  margin-left: -1em;
  padding: 1em;
}
```

```typst
// templates/base.typ
#let conf(
  page-title: [],
  subtitle: none,
  doc,
) = [
  #html.link(rel: "stylesheet", href: "/style.css")

  #html.nav[
    #html.a(href: "/")[home]
    #html.a(href: "/blog/")[blog]
  ]

  #html.header[
    #html.h1(page-title)
    #if subtitle != none {
      subtitle
    }
  ]

  #html.main[
    #doc
  ]
]
```
