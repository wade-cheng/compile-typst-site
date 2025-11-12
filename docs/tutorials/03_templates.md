# Part 3: Templates

I defer to the Typst documentation for how the language lets you set up templates. Here's one in action:

```typst
// templates/base.typ
#let conf(
  page-title: [],
  subtitle: none,
  doc,
) = [
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

And let's modify our pages to use it:

```typst
// src/index.typ
#import "../templates/base.typ": conf

#show: conf.with(
  page-title: "home",
)

My name is _saffron_. There are many like it, but this one is mine.
```

```typst
// src/blog.typ
#import "../templates/base.typ": conf

#show: conf.with(
  page-title: "blog",
  subtitle: "where all my memories lie",
)

a blog page!
```
