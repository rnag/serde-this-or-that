#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

use serde_this_or_that::{as_i64, Deserialize};

#[derive(Clone, Debug, Deserialize)]
pub struct Msg {
    #[serde(deserialize_with = "as_i64")]
    pub timestamp: i64,
}

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    sensible_env_logger::init!();

    let data = r#"
    {
        "timestamp": 123456789076543210
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("{m:?}");

    let data = r#"
    {
        "timestamp": "123456789076543210"
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("{m:?}");

    let data = r#"
    {
        "timestamp": -123456789076543210
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("{m:?}");

    let data = r#"
    {
        "timestamp": 12345678907654321000
    }"#;

    if let Err(e) = serde_json::from_str::<Msg>(data) {
        trace!("Expected error: {}", e);
    } else {
        error!("ERROR! An overflow error should have occurred.");
        assert_eq!(0, 1, "failure!");
    };

    Ok(())
}
