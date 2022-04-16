use serde_this_or_that::{as_bool, as_f64, as_u64, Deserialize};

use log::trace;
use serde_json::from_str;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MyStruct {
    #[serde(deserialize_with = "as_bool")]
    is_active: bool,
    #[serde(deserialize_with = "as_u64")]
    num_attempts: u64,
    #[serde(deserialize_with = "as_f64")]
    grade: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    sensible_env_logger::init!();

    let string = r#"
    {
        "isActive": "True",
        "numAttempts": "",
        "grade": "81"
    }
    "#;

    let s: MyStruct = from_str(string)?;

    trace!("{s:#?}");

    assert!(s.is_active);
    assert_eq!(s.num_attempts, 0);
    assert_eq!(s.grade, 81.0);

    Ok(())
}
