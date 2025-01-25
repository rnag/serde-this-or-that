# serde-this-or-that

[<img alt="github" src="https://img.shields.io/badge/github-source-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/rnag/serde-this-or-that)
[<img alt="crates.io" src="https://img.shields.io/crates/v/serde-this-or-that.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">](https://crates.io/crates/serde-this-or-that)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/serde-this-or-that/latest?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="22">](https://docs.rs/serde-this-or-that)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/rnag/serde-this-or-that/build.yml?branch=main&style=for-the-badge" height="22">](https://github.com/rnag/serde-this-or-that/actions?query=branch%3Amain)

Custom deserialization for fields that can be specified as multiple types.

---

This crate works with Cargo with a `Cargo.toml` like:

```toml
[dependencies]
serde-this-or-that = "0.5.0"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## Getting started

Add some usage to your application.

Here's an example of using `serde-this-or-that` in code:

```rust
use serde::Deserialize;
use serde_json::from_str;
use serde_this_or_that::{as_bool, as_f64, as_opt_i64, as_u64};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MyStruct {
    #[serde(deserialize_with = "as_bool")]
    is_active: bool,
    #[serde(deserialize_with = "as_u64")]
    num_attempts: u64,
    #[serde(deserialize_with = "as_f64")]
    grade: f64,
    // uses #[serde(default)] in case the field is missing in JSON
    #[serde(default, deserialize_with = "as_opt_i64")]
    confidence: Option<i64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let string = r#"
    {
        "isActive": "True",
        "numAttempts": "",
        "grade": "81",
        "confidence": "A+"
    }
    "#;

    let s: MyStruct = from_str(string)?;
    println!("{s:#?}");

    assert!(s.is_active);
    assert_eq!(s.num_attempts, 0);
    assert_eq!(s.grade, 81.0);
    assert_eq!(s.confidence, None);

    Ok(())
}
```

## Exported Functions

- [`as_bool`](https://docs.rs/serde-this-or-that/latest/serde_this_or_that/fn.as_bool.html) / [
  `as_opt_bool`](https://docs.rs/serde-this-or-that/latest/serde_this_or_that/fn.as_opt_bool.html)
- [`as_f64`](https://docs.rs/serde-this-or-that/latest/serde_this_or_that/fn.as_f64.html) / [
  `as_opt_f64`](https://docs.rs/serde-this-or-that/latest/serde_this_or_that/fn.as_opt_f64.html)
- [`as_i64`](https://docs.rs/serde-this-or-that/latest/serde_this_or_that/fn.as_i64.html) / [
  `as_opt_i64`](https://docs.rs/serde-this-or-that/latest/serde_this_or_that/fn.as_opt_i64.html)
- [`as_string`](https://docs.rs/serde-this-or-that/latest/serde_this_or_that/fn.as_string.html) / [
  `as_opt_string`](https://docs.rs/serde-this-or-that/latest/serde_this_or_that/fn.as_opt_string.html)
- [`as_u64`](https://docs.rs/serde-this-or-that/latest/serde_this_or_that/fn.as_u64.html) / [
  `as_opt_u64`](https://docs.rs/serde-this-or-that/latest/serde_this_or_that/fn.as_opt_u64.html)

## Examples

You can check out sample usage of this crate in
the [examples/](https://github.com/rnag/serde-this-or-that/tree/main/examples)
folder in the project repo on GitHub.

## Performance

The benchmarks suggest that implementing a custom
[`Visitor`] as `serde-this-or-that` does, performs
on average **about 15x better** than an approach with an [untagged enum].

A separate benchmark compares performance against the [serde_with] crate:
it indicates both crates perform about the same,
assuming only [`DisplayFromStr`] is used.
But when [`PickFirst`] is involved, `serde-this-or-that` appears to
perform **about 12x better** in an average case.

The benchmarks live in the [benches/](https://github.com/rnag/serde-this-or-that/tree/main/benches)
folder, and can be run with `cargo bench`.

[`Visitor`]: https://docs.serde.rs/serde/de/trait.Visitor.html

[untagged enum]: https://stackoverflow.com/a/66961340/10237506

[serde_with]: https://docs.rs/serde_with

[`DisplayFromStr`]: https://docs.rs/serde_with/latest/serde_with/struct.DisplayFromStr.html

[`PickFirst`]: https://docs.rs/serde_with/latest/serde_with/struct.PickFirst.html

## Optionals

The extra helper functions that begin with `as_opt`, return an `Option<T>` of the respective data type `T`,
rather than the type `T` itself (see [#4](https://github.com/rnag/serde-this-or-that/issues/4)).

On success, they return a value of `T` wrapped in [
`Some`](https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some).

On error, or when there is a `null` value, or one of an *invalid* data type, the
`as_opt` helper functions return [`None`](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None) instead.

## Contributing

Contributions are welcome! Open a pull request to fix a bug, or [open an issue][]
to discuss a new feature or change.

Check out the [Contributing][] section in the docs for more info.

[Contributing]: CONTRIBUTING.md

[open an issue]: https://github.com/rnag/serde-this-or-that/issues

## License

This project is proudly licensed under the MIT license ([LICENSE](LICENSE)
or http://opensource.org/licenses/MIT).

`serde-this-or-that` can be distributed according to the MIT license. Contributions
will be accepted under the same license.

## Authors

* [Ritvik Nag](https://github.com/rnag)
