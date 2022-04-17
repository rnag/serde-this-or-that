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

## v0.3.0 (2022-04-17)

### Breaking Changes
- Remove dependency on the `derive` feature of `serde`
  - Add it as an optional feature named `derive` instead.

### Features
- Update docs

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
