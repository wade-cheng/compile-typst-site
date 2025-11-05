#import "../templates/base.typ": conf

#show: conf.with(
  page-title: "garden",
)

Welcome to my digital garden, where I tend to long-running articles instead
of one-off blog posts.

#html.hr()


#let posts = json("../posts.json")

#for post in posts.garden {
  html.p[
    #html.a(href: post.path)[#post.pagetitle]
  ]
}

#html.script("NekoType = 'white';")
#html.div(id: "nl")[
  #html.script(src: "https://webneko.net/n20171213.js")
  #html.a(href: "https://webneko.net")
]
