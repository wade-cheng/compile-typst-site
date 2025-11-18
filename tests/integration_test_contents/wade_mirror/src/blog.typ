#import "../templates/base.typ": conf
#import "../templates/utils.typ"

#show: conf.with(
  page-title: "blog",
)

Isn't the internet so weird? You might not know me, but now you can make an
effort without our ever meeting. In another life, maybe we would've met, and
I would've really liked just doing laundry and taxes with you.

// Subscribe to this blog with RSS #html.span[#html.elem("svg", attrs: (
//   xmlns: "http://www.w3.org/2000/svg",
//   style: "transform: translate(0, 3px)",
//   width: "1em",
//   height: "1em",
//   viewBox: "0 0 24 24",
//   fill: "none",
//   stroke: "currentColor",
//   "stroke-width": "2",
//   "stroke-linecap": "round",
//   "stroke-linejoin": "round",
//   class: "feather feather-rss",
// ))[
//   #html.elem("path", attrs: (d: "M4 11a9 9 0 0 1 9 9"))
//   #html.elem("path", attrs: (d: "M4 4a16 16 0 0 1 16 16"))
//   #html.elem("circle", attrs: (cx: "5", cy: "19", r: "1"))
// ]]: #link("./feed.xml", "https://wade-cheng.is-a.dev/blog/feed.xml")

#html.hr()

#let PLACEHOLDER = (value: (date: "0-0-0"))

#let files = (json("../files.json")
  .pairs()
  .sorted(
    by: (l, r) => l >= r,  // decreasing order
    key: entry => {
      let queried = entry.at(1)
      let date = queried
        .at(0, default: PLACEHOLDER)
        .at("value")
        .at("date")
      date
    }))

#for (path, queried) in files [
  #if queried.len() > 0 and path.contains("/blog/"){
    let path = (path
      .split("/blog/")
      .at(-1)
      .replace(regex("/index\\.typ$"), "/")
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
