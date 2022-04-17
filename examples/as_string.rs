#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

use serde::Deserialize;
use serde_this_or_that::as_string;

#[derive(Clone, Debug, Deserialize)]
pub struct Msg {
    #[serde(deserialize_with = "as_string")]
    pub timestamp: String,
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
    assert!(m.timestamp.is_empty());
    trace!("  {m:?}");

    trace!("With Null:  ");
    let data = r#"
    {
        "timestamp": null
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();
    assert!(m.timestamp.is_empty());
    trace!("  {m:?}");

    trace!("With Zero (0):");
    let data = r#"
    {
        "timestamp": 0
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert_eq!(m.timestamp, "0");

    trace!("With String:");

    let data = r#"
    {
        "timestamp": "hello, world!"
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert_eq!(m.timestamp, "hello, world!");

    trace!("With Bool:");

    let data = r#"
    {
        "timestamp": false
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert_eq!(m.timestamp, "false");

    trace!("With U64:");

    let data = r#"
    {
        "timestamp": 123456789076543210
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert_eq!(m.timestamp, "123456789076543210");

    trace!("With I64:");

    let data = r#"
    {
        "timestamp": -123
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert_eq!(m.timestamp, "-123");

    trace!("With F64:");

    let data = r#"
    {
        "timestamp": 1234567890.76543210
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert_eq!(m.timestamp, "1234567890.7654321");

    Ok(())
}
