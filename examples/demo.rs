#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

use serde::Deserialize;
use serde_json::from_str;
use serde_this_or_that::{as_bool, as_f64, as_opt_i64, as_opt_string, as_u64};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MyStruct {
    #[serde(deserialize_with = "as_bool")]
    is_active: bool,
    #[serde(deserialize_with = "as_u64")]
    num_attempts: u64,
    #[serde(deserialize_with = "as_f64")]
    grade: f64,
    #[serde(deserialize_with = "as_opt_string")]
    notes: Option<String>,
    #[serde(default, deserialize_with = "as_opt_i64")]
    confidence: Option<i64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    sensible_env_logger::init!();

    let string = r#"
    {
        "isActive": "True",
        "numAttempts": "",
        "grade": "81",
        "notes": ""
    }
    "#;

    let s: MyStruct = from_str(string)?;

    trace!("{s:#?}");

    assert!(s.is_active);
    assert_eq!(s.num_attempts, 0);
    assert_eq!(s.grade, 81.0);
    assert_eq!(s.notes, None);
    assert_eq!(s.confidence, None);

    let string = r#"
    {
        "isActive": false,
        "numAttempts": 1.7,
        "grade": null,
        "notes": true,
        "confidence": "test!"
    }
    "#;

    let s: MyStruct = from_str(string)?;

    trace!("{s:#?}");

    assert!(!s.is_active);
    assert_eq!(s.num_attempts, 2);
    assert_eq!(s.grade, 0.0);
    assert_eq!(s.notes, Some("true".to_owned()));
    assert_eq!(s.confidence, None);

    Ok(())
}
