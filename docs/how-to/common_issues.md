# Solving common issues

In short, anything that you can implement in native Typst is easy, because you just do that and then run `compile-typst-site`. If you can't do something in native Typst, you will have to add third party scripts to your workflow. They can run automatically when added to the config file.

## Injecting content into `<head>`

You might want to add your stylesheets to the `<head>` because this makes them load faster and makes your HTML more semantically correct. Typst doesn't let you yet.[^1]

[^1]: This is on their radar.

We can solve this by using post-processing and regex, lmao. Add a command to the config file:

```toml
# compile-typst-site.toml
post_processing_typ = ['python', 'post-process.py']
```

For every Typst source file, the compiled HTML output is passed into the command as stdin. The command's stdout becomes the output written to `_site`.[^2]

[^2]: Since these are the rules, we don't have to use Python as our scripting language. You can use bash. awk. Rust. An LLM call. A utility that lets you email Dave from work with your HTML output, returning his reply to stdout. `['cat', '<!-- cat. -->', '-']`.

The [full example](https://github.com/wade-cheng/compile-typst-site/tree/main/examples/typst-site-full) does this. The script it runs is:

```python
#!/usr/bin/env python
import sys
import re

input_data = sys.stdin.buffer.read()

# Find all head tags
heads = re.findall(rb'<head>.*?</head>', input_data, flags=re.DOTALL)

# Replace first head with second head, then remove the original second head
# We need to remove the second one first to avoid issues
replaced = input_data.replace(heads[1], b'', 1)  # Remove second head
replaced = replaced.replace(heads[0], heads[1], 1)  # Replace first with second

sys.stdout.buffer.write(replaced)
```
