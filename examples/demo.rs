use serde_this_or_that::{as_bool, as_u64, Deserialize};

use log::trace;
use serde_json::from_str;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MyStruct {
    #[serde(deserialize_with = "as_bool")]
    is_active: bool,
    #[serde(deserialize_with = "as_u64")]
    num_attempts: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    sensible_env_logger::init!();

    let string = r#"
    {
        "isActive": "True",
        "numAttempts": "3"
    }
    "#;

    let s: MyStruct = from_str(string)?;

    trace!("{s:?}");

    assert!(s.is_active);
    assert_eq!(s.num_attempts, 3);

    Ok(())
}
