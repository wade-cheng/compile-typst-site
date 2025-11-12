# Part 2: A teensy site

Let's start building a website!

New folder for our site to be in, `my-site`.

Create a file called `compile-typst-site.toml`. This designates its parent folder, `my-site`, as the project root.

At this point, we can already run `compile-typst-site`. [Example] But nothing happens. So let's create some content.

```typst
// src/index.typ
= home

My name is _saffron_. There are many like it, but this one is mine.
```

```typst
// src/blog.typ
= blog

a blog page!
```

After running `compile-typst-site` again, we get a website!

[example]

We can put this on github pages, or Neocities, blah blah. Mention local serving. `site.com/` and `site.com/blog/`.

Detour to explain what exactly the command did, typst c x->y --root, --features html.
