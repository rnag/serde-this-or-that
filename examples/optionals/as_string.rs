#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

use serde::Deserialize;
use serde_this_or_that::as_opt_string;

#[derive(Clone, Debug, Deserialize)]
pub struct Msg {
    #[serde(deserialize_with = "as_opt_string")]
    pub timestamp: Option<String>,
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
    assert_eq!(m.timestamp, Some("".into()));
    trace!("  {m:?}");

    trace!("With Null:  ");
    let data = r#"
    {
        "timestamp": null
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();
    assert_eq!(m.timestamp, None);
    trace!("  {m:?}");

    trace!("With Zero (0):");
    let data = r#"
    {
        "timestamp": 0
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert_eq!(m.timestamp, Some("0".into()));

    trace!("With String:");

    let data = r#"
    {
        "timestamp": "hello, world!"
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert_eq!(m.timestamp, Some("hello, world!".into()));

    trace!("With Bool:");

    let data = r#"
    {
        "timestamp": false
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert_eq!(m.timestamp, Some("false".into()));

    trace!("With U64:");

    let data = r#"
    {
        "timestamp": 123456789076543210
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert_eq!(m.timestamp, Some("123456789076543210".into()));

    trace!("With I64:");

    let data = r#"
    {
        "timestamp": -123
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert_eq!(m.timestamp, Some("-123".into()));

    trace!("With F64:");

    let data = r#"
    {
        "timestamp": 1234567890.76543210
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert_eq!(m.timestamp, Some("1234567890.7654321".into()));

    Ok(())
}
