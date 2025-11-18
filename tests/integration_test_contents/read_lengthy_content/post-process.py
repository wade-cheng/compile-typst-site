#!/usr/bin/env python
import sys

stdin = sys.stdin.read()

print("stderr message here!", file=sys.stderr)

# exit(1)

print("<!-- meow -->")
print(stdin)
