# Quick start/performing common actions

This how-to guide is for people who want to hit the ground running. If you want a slower-paced pedagogic introduction, try [](../tutorials/beginner/index.md).

## Install the software

[](./installation.md)

## Download the sample site

Get the sample site from <https://github.com/wade-cheng/compile-typst-site-hardcoded-links-example>. Either download the zip by clicking [here](https://github.com/wade-cheng/compile-typst-site-hardcoded-links-example/archive/refs/heads/main.zip) or running

```

$ git clone https://github.com/wade-cheng/compile-typst-site-hardcoded-links-example

```

## Compile

Compile the project into a site by `cd`ing into the project root directory and running

```

$ compile-typst-site

```

## Update files

You can either simply make changes and recompile on every change, or use

```

$ compile-typst-site --watch --ignore-initial

```

and then save your changes.

Try doing so after changing `src/blog.typ` to:

```typst
// src/blog.typ
#import "../templates/base.typ": conf

#show: conf.with(
  page-title: "blog",
  subtitle: "where all my memories lie",
)

I love blogging! // <---- NEW
```

## Add new pages

Observe `src/blog.typ`:

```typst
// src/blog.typ
#import "../templates/base.typ": conf

#show: conf.with(
  page-title: "blog",
  subtitle: "where all my memories lie",
)

a blog page
```

To add another page titled `garden`, we will follow the template. You can create the file:

```typst
// src/garden.typ  <---- NEW
#import "../templates/base.typ": conf

#show: conf.with(
  page-title: "garden",
)

🌾 🍂 🌿 🪹 🪴 🍃 🪺
```

To add it to the navbar, you will also want to update the base template:

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
    #html.a(href: "/garden/")[garden] //  <---- NEW
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

Notice that if you updated it while `compile-typst-site --watch --ignore-initial` was running, all three of your files got the new navigation bar link.
