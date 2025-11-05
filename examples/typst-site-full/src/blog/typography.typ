#import "../../templates/blog_post.typ": conf

#show: conf.with(
    page-title: "typography",
    date: "2025-10-19",
)

I love typography!!! Massive fan of making things look nice. I've just added lead-in small caps to the start of these content pages and enabled oldstyle figures for when I might want them later. (For now, these are the ones that look like this: 1234567890)

Also, I've through this just discovered that EB Garamond actually supports a lot of OpenType features after all. The #link("https://googlefonts.github.io/ebgaramond-specimen/")[typography] linked from the Google Fonts page is quite pretty. Unfortunately and ironically, the Google Fonts binary for the font doesn't actually bundle these features. This site statically hosts the full (#link("/assets/eb-garamond/OFL.txt")[OFL-license-licensed]) font from #link("https://github.com/octaviopardo/EBGaramond12").

A really nice resource on many things typography is Matthew Butterick's #link("https://practicaltypography.com/").
