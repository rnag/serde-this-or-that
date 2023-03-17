#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

use serde::Deserialize;
use serde_this_or_that::as_opt_bool;

#[derive(Clone, Debug, Deserialize)]
pub struct Msg {
    #[serde(deserialize_with = "as_opt_bool")]
    pub timestamp: Option<bool>,
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
    assert_eq!(m.timestamp, None);
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
    assert_eq!(m.timestamp, Some(false));

    trace!("With One (1):");

    let data = r#"
    {
        "timestamp": 1
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert_eq!(m.timestamp, Some(true));

    trace!("With String (truthy #1):");

    let data = r#"
    {
        "timestamp": "tRuE"
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert_eq!(m.timestamp, Some(true));

    trace!("With String (truthy #2):");

    let data = r#"
    {
        "timestamp": "Y"
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert_eq!(m.timestamp, Some(true));

    trace!("With String (falsy):");

    let data = r#"
    {
        "timestamp": "nope!"
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert_eq!(m.timestamp, Some(false));

    trace!("With String (Invalid Numeric):");

    let data = r#"
    {
        "timestamp": "123456789076543210"
    }"#;

    let m: Msg = serde_json::from_str(data).unwrap();

    trace!("  {m:?}");
    assert_eq!(m.timestamp, Some(false));

    trace!("With U64:");

    let data = r#"
    {
        "timestamp": 123456789076543210
    }"#;

    if serde_json::from_str::<Msg>(data).is_ok() {
        trace!("All Good on the Home-front!");
    } else {
        error!("  ERROR! no error should have occurred.");
        assert_eq!(0, 1, "failure!");
    };

    trace!("With I64:");

    let data = r#"
    {
        "timestamp": -123
    }"#;

    if serde_json::from_str::<Msg>(data).is_ok() {
        trace!("All Good on the Home-front!");
    } else {
        error!("  ERROR! no error should have occurred.");
        assert_eq!(0, 1, "failure!");
    };

    Ok(())
}
