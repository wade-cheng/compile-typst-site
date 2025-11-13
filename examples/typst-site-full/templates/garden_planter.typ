#import "../templates/base.typ"

#let conf(
  page-title: "",
  date: "",
  doc,
) = {
  [#metadata(
    (
      "page-title": page-title,
      "date": date
    )
  ) <data>]
  
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

  html.p[#html.small[#link("/garden.html")[back to garden]]]
}
