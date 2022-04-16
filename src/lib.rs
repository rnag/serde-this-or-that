#![doc(html_root_url = "https://docs.rs/serde-this-or-that/0.1.1")]
#![warn(rust_2018_idioms, missing_docs)]
#![deny(warnings, dead_code, unused_imports, unused_mut)]

//! [![github]](https://github.com/rnag/serde-this-or-that)&ensp;[![crates-io]](https://crates.io/crates/serde-this-or-that)&ensp;[![docs-rs]](https://docs.rs/serde-this-or-that)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! Custom deserialization for fields that can be specified as multiple types.
//!
//! <br>
//!
//! ## Usage
//!
//! ```rust
//! use serde_json::from_str;
//! use serde_this_or_that::{as_bool, as_f64, as_u64, Deserialize};
//!
//! #[derive(Deserialize, Debug)]
//! #[serde(rename_all = "camelCase")]
//! struct MyStruct {
//!     #[serde(deserialize_with = "as_bool")]
//!     is_active: bool,
//!     #[serde(deserialize_with = "as_u64")]
//!     num_attempts: u64,
//!     #[serde(deserialize_with = "as_f64")]
//!     grade: f64,
//! }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let string = r#"
//!     {
//!         "isActive": "True",
//!         "numAttempts": "",
//!         "grade": "81"
//!     }
//!     "#;
//!
//!     let s: MyStruct = from_str(string)?;
//!     println!("{s:#?}");
//!
//!     assert!(s.is_active);
//!     assert_eq!(s.num_attempts, 0);
//!     assert_eq!(s.grade, 81.0);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Examples
//!
//! You can check out sample usage of this crate in the [examples/](https://github.com/rnag/serde-this-or-that/tree/main/examples)
//! folder in the project repo on GitHub.
//!
//! ## Performance
//!
//! The benchmarks suggest that implementing a custom
//! `Visitor` as `serde-this-or-that` does, performs
//! on average **about 10x better** than an approach with an [untagged enum].
//!
//! The benchmarks live in the [benches/](https://github.com/rnag/serde-this-or-that/tree/main/benches)
//! folder, and can be run with `cargo bench`.
//!
//! [untagged enum]: https://stackoverflow.com/a/66961340/10237506
//!
//! ## Readme Docs
//!
//! You can find the crate's readme documentation on the
//! [crates.io] page, or alternatively in the [`README.md`] file on the GitHub project repo.
//!
//! [crates.io]: https://crates.io/crates/serde-this-or-that
//! [`README.md`]: https://github.com/rnag/serde-this-or-that
//!

mod de_impl;

pub use de_impl::{as_bool, as_f64, as_i64, as_string, as_u64};
pub use serde;
#[doc(hidden)]
pub use serde::*;

#[cfg(test)]
mod tests {
    use super::*;

    use serde::Deserialize;
    use serde_json::from_str;

    #[test]
    fn serde_this_or_that_works() {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct MyStruct {
            #[serde(deserialize_with = "as_bool")]
            is_active: bool,
            #[serde(deserialize_with = "as_u64")]
            num_attempts: u64,
        }

        let string = r#"
        {
            "isActive": "True",
            "numAttempts": "3"
        }
        "#;

        let s: MyStruct = from_str(string).unwrap();

        assert!(s.is_active);
        assert_eq!(s.num_attempts, 3)
    }
}
