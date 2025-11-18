#!/usr/bin/env python
import sys
import re

input_data = sys.stdin.buffer.read().decode("utf-8")

# Find all head tags
heads = re.findall(r"<head>.*?</head>", input_data, flags=re.DOTALL)

# Replace first head with second head, then remove the original second head
# We need to remove the second one first to avoid issues
replaced = input_data.replace(heads[1], "", 1)  # Remove second head
replaced = replaced.replace(heads[0], heads[1], 1)  # Replace first with second

# TODO: add heading anchors back in"

# Extract the endnotes section content
endnotes_match = re.search(
    r'<section role="doc-endnotes">(.*?)</section>', replaced, re.DOTALL
)

if endnotes_match:
    endnotes_content = endnotes_match.group(1)

    # Remove the original endnotes section
    replaced = re.sub(
        r'\s*<section role="doc-endnotes">.*?</section>', "", replaced, flags=re.DOTALL
    )

    # Replace </main> with the endnotes section inside main
    replaced = re.sub(
        r"</main>",
        f'      <section role="doc-endnotes">\n        <hr><h2>Notes</h2>{endnotes_content}      </section>\n      </main>',
        replaced,
    )

sys.stdout.buffer.write(replaced.encode("utf-8"))
