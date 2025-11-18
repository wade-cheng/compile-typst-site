#import "../../templates/blog_post.typ": conf

#show: conf.with(
  page-title: "on hyperlink styling",
  date: "2025-10-21",
)

I've changed my opinion on how hyperlinks should be styled throughout my life, and I've recently been thinking about it again.

First, some history and some context. I like small, cozy, lightweight websites! My first links were probably unstyled, along with the rest of the webpage, but this was probably changed quite quickly. I think the blue on white is tacky, and the clicked-purple on white more so. They then became black to match the rest of the body text in color, in part because I don't think it really matters if you've already visited a page or not, or maybe I just don't like things changing out from under me. And then set it to un-underline on-hover to boot. Cute!

The most recent development in my link styling steals from Matthew Butterick's Practical Typography. He #link("https://practicaltypography.com/how-to-use.html")[writes]: "But underlining is nothing more than an odious typewriter habit, held over on websites because of inertia. It was tolerable in 1994 when web browsers could do so little. But not today, when they can do so much," as well as "Of course, the main reason hyperlinks have historically been overstyled is because generating clicks on those links is essential to internet advertising. Here, you will find no advertising---just us readers. The design is tailored accordingly."

Well, that's quite heartwarming. I'm charmed! The current revision of my stylesheet implements his link style---an indeed unobtrusive hollow circle superscripting the link, where the link body fades into an accent color once hovered---but with a purple take on the accent color instead of his maroon.

I really do have to reiterate how heartwarming the rationale is behind this style. And I do indeed feel that the resulting text is considerably less noisy than before. And it feels softer! The lazy fade in of the pastel accent color is quite cozy.

But finally, to the downsides. Visibility. Ehhhh, probably not the greatest now. Butterick #link("https://practicaltypography.com/underlining.html")[alleges] that plenty large sites no longer use underlining, but this seems mostly only strictly true in the display regions of these sites where links may read as buttons instead. The current is a clear example. However, when in the body of actual content (see: Wikipedia as a canonical example), these sites instead turn to underlining on hover while maintaining a colored link.

Overall, not sure how I feel.

I don't quite like setting links in an accent color, because I enjoy the idea of being able to read my site just fine after running it through a grayscale printer. Much of my undergrad has been morally supported by having paper to leaf through and annotate. It's grounding.

I'm not sure about the Butterick Method. How does one know where a link starts, if only the end is delimited? Additionally, I'm not convinced of the attractiveness of adding a circle to delimit on the left side. If it's placed at the same height, it looks a bit like a googly-eyed alien, I'm not sure what to think about if it goes near the baseline (as if like #link("https://en.wikipedia.org/wiki/Quotation_mark")[Dutch quotations]), and   way it's a bit noisy and bubbly and messes up the word spacing and eh.

Maybe I'll even go back to the original all-black method of styling, clutter be damned.

We'll see how my tastes change in these coming weeks.
