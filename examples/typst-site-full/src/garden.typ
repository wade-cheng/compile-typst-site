#import "../templates/base.typ": conf

#show: conf.with(
  page-title: "garden",
)

Welcome to my digital garden, where I tend to long-running articles instead
of one-off blog posts.

#html.hr()

#let files = json("../files.json")

#for (path, queried) in files.pairs() [
  #if queried.len() > 0 and path.contains("/garden/"){
    let path = (path
      .split("/garden/")
      .at(-1)
      .replace(regex("\\.typ$"), "/"))
    let page = queried.at(0).at("value")

    html.p[
      #html.a(href: path)[#page.page-title]
    ]
  }
]

#html.script("NekoType = 'white';")
#html.div(id: "nl")[
  #html.script(src: "https://webneko.net/n20171213.js")
  #html.a(href: "https://webneko.net")
]
