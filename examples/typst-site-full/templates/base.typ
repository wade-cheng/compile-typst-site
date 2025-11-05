#let conf(
  page-title: "",
  title-override: [],
  subtitle: [],
  doc,
) = {
  html.head[
    // fonts
    #html.link(rel: "stylesheet", href: "/assets/eb-garamond/eb-garamond.css")

    // standard metadata
    // https://css-tricks.com/essential-meta-tags-social-media/
    #html.meta(charset: "utf-8")
    #if title-override != [] {
      html.title[#title-override]
    } else {
      html.title[wade's #page-title]
    }
    #html.meta(name: "viewport", content: "width=device-width, initial-scale=1.0")

    #html.link(href: "/style.css", rel: "stylesheet", type: "text/css")

    // favicon stuff https://realfavicongenerator.net
    #html.elem("link", attrs: (
      rel: "icon",
      type: "image/png",
      href: "/favicon/favicon-96x96.png",
      sizes: "96x96",
    ))
    #html.elem("link", attrs: (
      rel: "icon",
      type: "image/svg+xml",
      href: "/favicon/favicon.svg",
    ))
    #html.elem("link", attrs: (
      rel: "shortcut icon",
      href: "/favicon/favicon.ico",
    ))
    #html.elem("link", attrs: (
      rel: "apple-touch-icon",
      sizes: "180x180",
      href: "/favicon/apple-touch-icon.png",
    ))
    #html.elem("meta", attrs: (
      name: "apple-mobile-web-app-title",
      content: "wade's site",
    ))
    #html.elem("link", attrs: (
      rel: "manifest",
      href: "/favicon/site.webmanifest",
    ))

    #html.link(rel: "prefetch", href: "https://webneko.net/n20171213.js")
  ]

  html.div(id: "site-container")[
    #html.nav[
      #html.a(href: "/")[home]
      #html.a(href: "/blog.html")[blog]
      #html.a(href: "/garden.html")[garden]
    ]

    #html.header[
      #html.h1(page-title)
      #if subtitle != [] {
        subtitle
      }
    ]

    #html.main[
      #doc
    ]
  ]
}
