# Installation

```{important}
To compile your Typst files, we use the Typst CLI. Make sure you have it [installed](https://typst.app/open-source/#download).

If you are on Windows and are unfamiliar with downloading CLIs, the `winget` method may be easiest.
```

After Typst is installed, use one of the `compile-typst-site` installation methods below.

## From a precompiled binary (for beginners)

See the [releases](https://github.com/wade-cheng/compile-typst-site/releases) to install from a precompiled binary.

Check that Typst and `compile-typst-site` were installed properly by printing their versions. If there are no errors, you're done with installation. It should look something like this:

```{literalinclude} versions.txt
:language: text
```

## From cargo-binstall

If you know what this is, yes, we support this.

```
$ cargo binstall compile-typst-site
```

## Compile from source

You probably know what you're doing.

```{note}
The latest stable versions are the releases. They can be found above, or tagged on Github. The `main` branch, if ahead of a stable release, has alpha content. But it passed the test suite, and probably works fine. Other branches like `wade-cheng/dev` are wild territory. Beware.
```

```
# from github
$ cargo install --git https://github.com/wade-cheng/compile-typst-site.git
```

```
# from latest release on crates.io
$ cargo install compile-typst-site
```
