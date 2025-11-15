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

  doc
}
