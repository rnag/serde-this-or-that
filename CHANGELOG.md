# Changelog

This project follows semantic versioning.

Possible header types:

- `Features` for any new features added, or for backwards-compatible
  changes to existing functionality.
- `Bug Fixes` for any bug fixes.
- `Breaking Changes` for any backwards-incompatible changes.

## [Unreleased]

<!--
### Features
- Added a new struct `MyStruct` with the following methods:
  - `my_method()`
  - `other_method()`
-->

## v0.5.0 (2025-01-25)

### Features

- **Added support for optional deserialization**:
    - New functions were introduced to handle `Option<T>` types during deserialization:
        - `as_opt_bool` – Returns `Option<bool>`.
        - `as_opt_f64` – Returns `Option<f64>`.
        - `as_opt_i64` – Returns `Option<i64>`.
        - `as_opt_string` – Returns `Option<String>`.
        - `as_opt_u64` – Returns `Option<u64>`.

  These functions ensure that `null` values in the JSON input or deserialization errors are correctly deserialized into
  `None`,
  while valid values will
  return the appropriate `Some(T)`.

### Bug Fixes

- Resolved an issue where the existing deserialization functions did not handle `Option<T>` types, only direct types.
  Now,
  `null` values and errors are deserialized to `None`, making the handling of nulls more consistent with Serde's
  standard
  behavior.

### Breaking Changes

- No breaking changes introduced in this version.

## v0.4.2 (2023-02-05)

### Bug Fixes

* Fix Clippy warnings in the build workflow.

## v0.4.1 (2023-02-05)

### Bug Fixes

* Fix readme badge, as per [badges/shields#8671](https://github.com/badges/shields/issues/8671).

## v0.4.0 (2022-04-18)

### Features

- Add benchmarks to compare performance against `serde_with`.
- Flatten some nested `match` arms into simpler `if` statements.
- Update `as_bool`
    - Update to check for a new "truthy" string value of  `ON`.
    - Add pattern matching to check common *true/false* values **before** converting the string
      to uppercase, which should make it overall more efficient.
- `serde_this_or_that` is now on par - in terms of performance - with `serde_with`! This is
  truly great news.

## v0.3.0 (2022-04-17)

### Breaking Changes

- Remove dependency on the `derive` feature of `serde`
    - Add it as an optional feature named `derive` instead.

### Features

- Replace `utilities` keyword with `this-or-that`, as I want crate to be
  searchable when someone types "this or that".
- Update docs.

## v0.2.0 (2022-04-17)

### Features

- Add `as_string` helper function, to coerce values to an owned `String` type.
- Update to handle *empty strings* and `null` values in JSON (should be deserialized as "zero" values).
- Round `floats` when converting to `u64` or `i64`.
- Similarly, handle floating-point values in strings when converting to `u64` or `i64`.
- Refactor to use `Result<Self::Value, E>` everywhere, instead of `Result<T, E>`.
- Rename `de.rs` -> `de_impl.rs` to avoid name conflicts.
- Add example `as_string.rs`
- Update *examples/*
- Update docs

## v0.1.1 (2022-04-16)

- Fix docs

## v0.1.0 (2022-04-16)

- Initial Release on [crates.io] :tada:

[crates.io]: https://crates.io/crates/serde-this-or-that
