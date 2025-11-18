#import "../../templates/blog_post.typ": conf

#show: conf.with(
    page-title: "typst blog generation",
    date: "2025-11-05",
)

Alright, I think I've figured out how to make this site render with Typst in a reasonably painless way.

Before, I was basically hand-writing HTML (with Nunjucks as a templating language), and it wasn't really too painful? The only really annoying points were 

- having to type links twice
- having to type `<p>` manually
- I never really worked out a solution to footnotes#footnote[But we do have footnotes now!]

But anyways, eleventy didn't have a way to incrementally compile properly, so I just wrote my own binary that watches for file changes.

All it does is call the Typst CLI on every typst file and also pass through files that I specify (like CSS). All templating is done in native typst!