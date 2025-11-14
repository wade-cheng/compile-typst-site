# Contributing

You don't have to contribute by writing code! Reporting issues and feature requests on [the tracker](https://github.com/wade-cheng/compile-typst-site/issues) is super helpful, as is writing documentation. These docs are under the `docs` folder at <https://github.com/wade-cheng/compile-typst-site>, but you can also edit any page by clicking on the pencil button at the top right of that page!

But of course, pull requests are welcome, though I'd appreciate some discussion in an issue first. Either way, here are some guidelines I'm vaguely following:

- debug logs should happen before the thing they're logging, for consistency and in case the thing they're logging hangs.

  That is, use:

  ```rust
  log::debug!("trying to compile {}", path);
  handle.join().unwrap()?;
  ```

  not:

  ```rust
  handle.join().unwrap()?;
  log::debug!("compiled {}", path);
  ```

  We have a "everything finished" info level log that may break this rule.
