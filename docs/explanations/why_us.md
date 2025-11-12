# Why use this program?

This program is for people who want to use Typst in their websites. But it's not the only solution.

For example, maybe you're considering just manually compiling all your files. Or maybe you've seen different solutions like integrating Typst into a framework ([Leptos](https://www.christopherbiscardi.com/i-use-typst-now), [Astro](https://forum.typst.app/t/my-static-site-blog-based-on-astro-and-typst/4103)), or using it in a custom filter for a templating engine a la Eleventy.[^1]

[^1]: This is what I tried out before writing this program. It was not fun. Call it a skill issue.

It's good to be informed, so the rest of this page will be a comparison of the methods I'm aware of based on the features that are available.

## Actually turning the Typst to HTML

All `compile-typst-site` does is call `typst compile src/file.typ src/file.html --features html --root .` over and over. This means compiling by hand is perfectly feasible, and yields the same exact results. But I wanted to automate doing that for every file.

This is simple because I like my website generation to be simple. Before this, I used Eleventy, Nunjucks, and handwritten HTML---another web stack that followed the philosophy of "just copy from some places to other places, running wrapper scripts as needed." When I decided HTML was a bit too tedious, I tried to add Typst as a language via a [filter](https://www.11ty.dev/docs/filters/), but found out Eleventy was only able to generate the entire site on every save. Since filters aren't run on multiple threads, this means things chugged. This sucks.

Since I like simple website stacks for my personal site, I haven't bothered learning what might be possible with a full framework. See the links in the lede or do your own research.

## Viewing your site

Compiling by hand and `compile-typst-site` don't have methods of local auto-reloading web servers. When your browser opens a plain HTML file, it might not load your CSS if it's linked in a separate file. This is a security feature in browsers, iiuc. This means you have to, e.g., `python -m http.server`, or only view your site after uploading it to Github Pages or Neocities.

Every other serious solution spins up a web server locally (to, for example, <http://localhost:8000/>) and updates it automatically when save your files. With our `http.server`, we have to manually hit refresh.[^2]

[^2]: This feature is on the issue tracker.

## Publishing your site

Pretty easy all-around. Any solution just lets you take a compiled output folder and shove it to some service verbatim (Github Pages, Neocites).
