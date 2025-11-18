#import "../../templates/garden_planter.typ": conf

#show: conf.with(
  page-title: "on web design",
)

I try to make my site both accessible and pretty. Accessiblity is important
to me because people should, like, be able to read things. As for my
personal stakes, I'm liable to get headaches when things go too "wacko
crazy." Among other things I may have forgotten, I have animations turned
off wherever possible, and am peeved by the "astigmatism white on black"
thing. People should be able to read things. And pretty is in a typography
as well as general design sense. By some definition somewhere, this is also
a subset of accessibility.

The rest of this page is a list of ideas or links with ideas that I've
looked at, if not implemented.

= general accessibility
At the very least, I'd like to make this website accessible to

- dark mode users
- keyboard-only users
- mobile/tablet/desktop users
- screen reader users

If you have problems, #link("https://github.com/wade-cheng/wade-cheng.github.io/issues/new")[file an issue]!

Here's some stuff related to these goals

- semantic HTML

- use the browser's "view as mobile device" mode. Resize the viewport like
  a madman. What happens if someone reads this on, like, a nokia flip
  phone?

- accessible hover tooltips: #link("https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Roles/")[mdn docs link]

- #link("https://dubbot.com/dubblog/2024/a-footnote-on-footnotes-they-need-to-be-accessible.html")

- #link("https://seirdy.one/posts/2020/11/23/website-best-practices/")---so much info

- actually stepping through the site with a screen reader. It looks like
  accessibility software for Linux is lacking these days :/ NVDA seems
  like the industry standard?

- #link("https://webaim.org/")
  - #link("https://wave.webaim.org/") generates a pretty cool visualization
  - don't forget color contrast: #link("https://webaim.org/resources/contrastchecker/")

= web performance
This is yet another subset of accessibility, or something

- profile! both CPU-bound JS and IO-bound network requests. You can throttle network requests in Firefox's devtools network tab to emulate different network speeds.

- the tongue-in-cheek #link("https://motherfuckingwebsite.com/") and its many spinoffs

- the #link("https://www.mcmaster.com/")[McMaster Carr website]
  - #link("https://youtu.be/-Ln-8QM8KhQ")[breakdown on youtube]
  - #link(
      "https://www.reddit.com/r/programming/comments/1g75r84/how_is_this_website_so_fast_breaking_down_the/",
    )[discussion on reddit]

- minimizing IO calls (network requests). Doesn't this sound familiar? \*looks at memory hierarchy\*
  - you can inline small or important bits of JS/CSS code

= typography

- #link("https://practicaltypography.com/")

- I think #link("https://en.wikipedia.org/wiki/Serif#Old-style")[old-style] fonts are really attractive. At writing, this uses EB Garamond, in part because it's free. Someone suggested Sabon.
