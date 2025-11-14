# Contributing

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
