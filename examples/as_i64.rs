#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

use serde::Deserialize;
use serde_this_or_that::as_i64;

#[derive(Clone, Debug, Deserialize)]
pub struct Msg {
    #[serde(deserialize_with = "as_i64")]
    pub timestamp: i64,
}

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    sensible_env_logger::init!();

    trace!("With Empty String:");
    let data = r#"
    {
        "timestamp": ""
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();
    assert_eq!(m.timestamp, 0);
    trace!("  {m:?}");

    trace!("With Null:  ");
    let data = r#"
    {
        "timestamp": null
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();
    assert_eq!(m.timestamp, 0);
    trace!("  {m:?}");

    trace!("With U64:");

    let data = r#"
    {
        "timestamp": 123456789076543210
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();
    trace!("  {m:?}");

    trace!("With String:");

    let data = r#"
    {
        "timestamp": "123456789076543210"
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();
    trace!("  {m:?}");

    trace!("With I64:");

    let data = r#"
    {
        "timestamp": -123456789076543210
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();
    trace!("  {m:?}");

    trace!("With F64:");

    let data = r#"
    {
        "timestamp": -0.5
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();
    trace!("  {m:?}");
    assert_eq!(m.timestamp, -1);

    trace!("With U64 (Larger):");

    let data = r#"
    {
        "timestamp": 12345678907654321000
    }"#;

    if let Err(e) = serde_json::from_str::<Msg>(data) {
        trace!("  Expected error: {}", e);
    } else {
        error!("  ERROR! An overflow error should have occurred.");
        assert_eq!(0, 1, "failure!");
    };

    Ok(())
}
