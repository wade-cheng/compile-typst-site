#import "../templates/base.typ"

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
  
  if target() == "paged" {
    doc
    return
  }
  
  show: base.conf.with(
    page-title: page-title,
    title-override: "wade's garden :: " + page-title,
  )
  html.style[
    main > p:first-of-type::first-line {
        font-variant: small-caps;
        font-feature-settings: \"smcp\";
    }
  ]

  doc

  html.p[#html.small[#link("/garden/")[back to garden]]]
}
