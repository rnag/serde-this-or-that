#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

use serde_this_or_that::{as_bool, Deserialize};

#[derive(Clone, Debug, Deserialize)]
pub struct Msg {
    #[serde(deserialize_with = "as_bool")]
    pub timestamp: bool,
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
    assert!(!m.timestamp);
    trace!("  {m:?}");

    trace!("With Null:  ");
    let data = r#"
    {
        "timestamp": null
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();
    assert!(!m.timestamp);
    trace!("  {m:?}");

    trace!("With Zero (0):");
    let data = r#"
    {
        "timestamp": 0
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert!(!m.timestamp);

    trace!("With One (1):");

    let data = r#"
    {
        "timestamp": 1
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert!(m.timestamp);

    trace!("With String (truthy #1):");

    let data = r#"
    {
        "timestamp": "tRuE"
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert!(m.timestamp);

    trace!("With String (truthy #2):");

    let data = r#"
    {
        "timestamp": "Y"
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert!(m.timestamp);

    trace!("With String (falsy):");

    let data = r#"
    {
        "timestamp": "nope!"
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert!(!m.timestamp);

    trace!("With String (Invalid Numeric):");

    let data = r#"
    {
        "timestamp": "123456789076543210"
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert!(!m.timestamp);

    trace!("With U64:");

    let data = r#"
    {
        "timestamp": 123456789076543210
    }"#;

    if let Err(e) = serde_json::from_str::<Msg>(data) {
        trace!("  Expected error: {}", e);
    } else {
        error!("  ERROR! An invalid value error should have occurred.");
        assert_eq!(0, 1, "failure!");
    };

    trace!("With I64:");

    let data = r#"
    {
        "timestamp": -123
    }"#;

    if let Err(e) = serde_json::from_str::<Msg>(data) {
        trace!("  Expected error: {}", e);
    } else {
        error!("  ERROR! An invalid value error should have occurred.");
        assert_eq!(0, 1, "failure!");
    };

    Ok(())
}
