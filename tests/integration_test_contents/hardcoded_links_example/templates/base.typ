#let conf(
  page-title: [],
  subtitle: none,
  doc,
) = {
  html.link(rel: "stylesheet", href: "/style.css")

  html.nav[
    #html.a(href: "/")[home]
    #html.a(href: "/blog/")[blog]
  ]

  html.header[
    #html.h1(page-title)
    #if subtitle != none {
      subtitle
    }
  ]

  html.main[
    #doc
  ]
}

