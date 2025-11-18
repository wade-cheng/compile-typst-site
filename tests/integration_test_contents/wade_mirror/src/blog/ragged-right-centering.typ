#import "../../templates/blog_post.typ": conf

#show: conf.with(
  page-title: "centering items within ragged right text",
  date: "2025-11-14",
)

When you set text to align flush against the left margin, you get a ragged right margin. This kind of alignment is also called ragged right alignment after the ragged right side.

This means you have a body with a center mass that isn't the geographic centner of the body. Since there's white space created by having a ragged right margin, the center mass of such a paragraph is a bit to the left of the center of the box you would draw around the paragraph.

Humans perceive center-alignment with respect to center mass, not geographic center. This means an interesting thing happens when you try to center-align something, like a figure or a dinkus,#footnote[Dinkuses are dividers like the one at the top of this footnote section, often center-aligned, often represented as three asterisks.] // I might later add a figure here to show what I mean.
next to a paragraph with too much ragged right whitespace. Even though the item is technically centered, it feels off-balance! Too far to the right!

Once I get the effort, I should include some pictures in this post. Or refer you to the many wonderful articles that visually explain the generic phenomena.

I'm not sure people have a problem with this in practice, but as far as I know, many professionally typeset documents are fully justified and thus don't suffer this issue.#footnote[Academic papers, pretty much any book you pick off a library shelf, so on.]