#import "../templates/base.typ"
#import "../templates/utils.typ"

#let conf(
  page-title: "",
  date: "",
  doc,
) = context {
  [#metadata(
    (
      "page-title": page-title,
      "date": date
    )
  ) <data>]
  
  show: base.conf.with(
    page-title: page-title,
    title-override: "wade's blog :: " + page-title,
    subtitle: [#html.div(class: "date")[#utils.format-date(date)]],
  )
  html.style[
    main > p:first-of-type::first-line {
        font-variant: small-caps;
        font-feature-settings: \"smcp\";
    }
  ]

  doc

  html.p[#html.small[#link("/blog/")[back to blog]]]
}

