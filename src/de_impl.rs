use std::{f64, fmt};

use crate::de::{self, Deserializer, Unexpected};

/// De-serialize either a `null`, `str`, `i64`, `f64`, or `u64`
/// as a *signed* value.
///
/// # Errors
/// Returns an error if a string is non-empty and not a valid numeric
/// value, or if the unsigned value `u64` *overflows* when converted
/// to `i64`.
///
/// # Returns
/// The signed (`i64`) value of a string or number.
///
pub fn as_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeI64WithVisitor)
}

/// De-serialize either a `null`, `str`, `u64`, `f64`, or `i64`
/// as an *unsigned* value.
///
/// # Errors
/// Returns an error if a string is non-empty and not a valid numeric
/// value, or if the signed value `i64` represents a *negative* number.
///
/// # Returns
/// The unsigned (`u64`) value of a string or number.
///
pub fn as_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeU64WithVisitor)
}

/// De-serialize either a `null`, `str`, `f64`, `u64`, or `i64`
/// as a *float* value.
///
/// # Errors
/// Returns an error if a string is non-empty and not a valid numeric value.
///
/// # Returns
/// The floating point (`f64`) value of a string or number.
///
pub fn as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeF64WithVisitor)
}

/// De-serialize either a `null`, `bool`, `str`, `u64`, or `f64`
/// as a *boolean* value.
///
/// # Truthy String Values
/// > Note: the pattern matching is *case insensitive*, so `YES` or `yes`
/// > works just the same.
///
/// These are the following "truthy" string values that result in a
/// boolean value of `true`:
///
///   - `1`
///   - `OK`
///   - `ON`
///   - `T`
///   - `TRUE`
///   - `Y`
///   - `YES`
///
/// # Errors
/// Returns an error if an unsigned `u64` or a float `f64` value is not
/// a *zero* or a *one*.
///
/// # Returns
/// The boolean (`bool`) value of a string, boolean, or number.
///
pub fn as_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeBoolWithVisitor)
}

/// De-serialize either a `null`, `str`, `bool`, `i64`, `f64`, or `u64`
/// as an (owned) *string* value.
///
/// # Returns
/// The owned `String` value of a string, boolean, or number.
///
pub fn as_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeStringWithVisitor)
}

// TODO maybe update these definitions into a macro ..?

struct DeserializeU64WithVisitor;

impl de::Visitor<'_> for DeserializeU64WithVisitor {
    type Value = u64;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an unsigned integer or a string")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match u64::try_from(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(E::custom(format!(
                "overflow: Unable to convert signed value `{v:?}` to u64"
            ))),
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.round() as u64)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(n) = v.parse::<u64>() {
            Ok(n)
        } else if v.is_empty() {
            Ok(0)
        } else if let Ok(f) = v.parse::<f64>() {
            Ok(f.round() as u64)
        } else {
            Err(E::invalid_value(Unexpected::Str(v), &self))
        }
    }

    /// We encounter a `null` value; this default implementation returns a
    /// "zero" value.
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(0)
    }
}

struct DeserializeI64WithVisitor;

impl de::Visitor<'_> for DeserializeI64WithVisitor {
    type Value = i64;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a signed integer or a string")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match i64::try_from(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(E::custom(format!(
                "overflow: Unable to convert unsigned value `{v:?}` to i64"
            ))),
        }
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.round() as i64)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(n) = v.parse::<i64>() {
            Ok(n)
        } else if v.is_empty() {
            Ok(0)
        } else if let Ok(f) = v.parse::<f64>() {
            Ok(f.round() as i64)
        } else {
            Err(E::invalid_value(Unexpected::Str(v), &self))
        }
    }

    /// We encounter a `null` value; this default implementation returns a
    /// "zero" value.
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(0)
    }
}

struct DeserializeF64WithVisitor;

impl de::Visitor<'_> for DeserializeF64WithVisitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a float or a string")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v as f64)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v as f64)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(f) = v.parse::<f64>() {
            Ok(f)
        } else if v.is_empty() {
            Ok(0.0)
        } else {
            Err(E::invalid_value(Unexpected::Str(v), &self))
        }
    }

    /// We encounter a `null` value; this default implementation returns a
    /// "zero" value.
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(0.0)
    }
}

struct DeserializeBoolWithVisitor;

impl de::Visitor<'_> for DeserializeBoolWithVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an integer (0 or 1) or a string")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Err(de::Error::invalid_value(
            Unexpected::Signed(v),
            &"zero or one",
        ))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v {
            0 => Ok(false),
            1 => Ok(true),
            other => Err(de::Error::invalid_value(
                Unexpected::Unsigned(other),
                &"zero or one",
            )),
        }
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v as u8 {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(de::Error::invalid_value(
                Unexpected::Float(v),
                &"zero or one",
            )),
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        // First, try to match common true/false phrases *without*
        // using `to_uppercase()`. This approach is likely more efficient.
        match v {
            "t" | "T" | "true" | "True" | "1" => Ok(true),
            "f" | "F" | "false" | "False" | "0" => Ok(false),
            other => {
                // So from the above, we've already matched the following
                // "truthy" phrases: ["T", "1"].
                // To be completely thorough, we also need to do a case-
                // insensitive match on ["OK", "ON", "TRUE", "Y", "YES"].
                match other.to_uppercase().as_str() {
                    "OK" | "ON" | "TRUE" | "Y" | "YES" => Ok(true),
                    _ => Ok(false),
                }
            }
        }
    }

    /// We encounter a `null` value; this default implementation returns a
    /// "false" value.
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(false)
    }
}

struct DeserializeStringWithVisitor;

impl de::Visitor<'_> for DeserializeStringWithVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string, bool, or a number")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_owned())
    }

    /// We encounter a `null` value; this default implementation returns an
    /// "empty" string.
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    mod as_string_tests {
        use super::*;

        #[derive(Debug, PartialEq, Deserialize)]
        struct TestStrStruct {
            #[serde(deserialize_with = "as_string")]
            field: String,
        }

        #[test]
        fn test_as_string_with_string() {
            let json = r#"{"field": "Hello"}"#;
            let deserialized: TestStrStruct = serde_json::from_str(json).unwrap();
            assert_eq!(
                deserialized,
                TestStrStruct {
                    field: "Hello".to_owned()
                }
            );
        }

        #[test]
        fn test_as_string_with_null() {
            let json = r#"{"field": null}"#;
            let deserialized: TestStrStruct = serde_json::from_str(json).unwrap();
            assert_eq!(
                deserialized,
                TestStrStruct {
                    field: "".to_owned()
                }
            );
        }

        #[test]
        fn test_as_string_with_number() {
            let json = r#"{"field": 123}"#;
            let deserialized: TestStrStruct = serde_json::from_str(json).unwrap();
            assert_eq!(
                deserialized,
                TestStrStruct {
                    field: "123".to_owned()
                }
            );
        }

        #[test]
        fn test_as_string_with_boolean() {
            let json = r#"{"field": true}"#;
            let deserialized: TestStrStruct = serde_json::from_str(json).unwrap();
            assert_eq!(
                deserialized,
                TestStrStruct {
                    field: "true".to_owned()
                }
            );
        }

        #[test]
        fn test_as_string_with_empty_string() {
            let json = r#"{"field": ""}"#;
            let deserialized: TestStrStruct = serde_json::from_str(json).unwrap();
            assert_eq!(
                deserialized,
                TestStrStruct {
                    field: "".to_owned()
                }
            );
        }
    }

    mod as_bool_tests {
        use super::*;

        #[derive(Debug, PartialEq, Deserialize)]
        struct TestBoolStruct {
            #[serde(deserialize_with = "as_bool")]
            field: bool,
        }

        #[test]
        fn test_truthy_values() {
            let truthy_values = ["1", "OK", "ON", "T", "TRUE", "Y", "YES"];
            for value in truthy_values {
                let json = format!(r#"{{"field": "{}"}}"#, value);
                let deserialized: TestBoolStruct = serde_json::from_str(&json).unwrap();
                assert_eq!(deserialized, TestBoolStruct { field: true });
            }
        }

        #[test]
        fn test_falsy_values() {
            let falsy_values = ["0", "OFF", "F", "FALSE", "N", "NO"];
            for value in falsy_values {
                let json = format!(r#"{{"field": "{}"}}"#, value);
                let deserialized: TestBoolStruct = serde_json::from_str(&json).unwrap();
                assert_eq!(deserialized, TestBoolStruct { field: false });
            }
        }

        #[test]
        fn test_as_bool_with_null() {
            let json = r#"{"field": null}"#;
            let deserialized: TestBoolStruct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestBoolStruct { field: false });
        }

        #[test]
        fn test_invalid_boolean() {
            let json = r#"{"field": "INVALID"}"#;
            let deserialized: TestBoolStruct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestBoolStruct { field: false });
        }
    }

    mod as_f64_tests {
        use super::*;

        #[derive(Debug, PartialEq, Deserialize)]
        struct TestF64Struct {
            #[serde(deserialize_with = "as_f64")]
            field: f64,
        }

        #[test]
        fn test_as_f64_with_large_number() {
            let json = r#"{"field": 1e308}"#;
            let deserialized: TestF64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestF64Struct { field: 1e308 });
        }

        #[test]
        fn test_as_f64_with_negative_number() {
            let json = r#"{"field": -123.45}"#;
            let deserialized: TestF64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestF64Struct { field: -123.45 });
        }

        #[test]
        fn test_as_f64_with_number() {
            let json = r#"{"field": 123.45}"#;
            let deserialized: TestF64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestF64Struct { field: 123.45 });
        }

        #[test]
        fn test_as_f64_with_integer() {
            let json = r#"{"field": 123}"#;
            let deserialized: TestF64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestF64Struct { field: 123.0 });
        }

        #[test]
        fn test_as_f64_with_string() {
            let json = r#"{"field": "123.45"}"#;
            let deserialized: TestF64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestF64Struct { field: 123.45 });
        }

        #[test]
        fn test_as_f64_with_empty_string() {
            let json = r#"{"field": ""}"#;
            let deserialized = serde_json::from_str::<TestF64Struct>(json).unwrap();
            assert_eq!(deserialized, TestF64Struct { field: 0.0 });
        }

        #[test]
        fn test_as_f64_with_null() {
            let json = r#"{"field": null}"#;
            let deserialized: TestF64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestF64Struct { field: 0.0 });
        }
    }

    mod as_i64_tests {
        use super::*;

        #[derive(Debug, PartialEq, Deserialize)]
        struct TestI64Struct {
            #[serde(deserialize_with = "as_i64")]
            field: i64,
        }

        #[test]
        fn test_as_i64_with_integer() {
            let json = r#"{"field": 123}"#;
            let deserialized: TestI64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestI64Struct { field: 123 });
        }

        #[test]
        fn test_as_i64_with_string() {
            let json = r#"{"field": "123"}"#;
            let deserialized: TestI64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestI64Struct { field: 123 });
        }

        #[test]
        fn test_as_i64_with_float() {
            let json = r#"{"field": 123.45}"#;
            let deserialized: TestI64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestI64Struct { field: 123 });
        }

        #[test]
        fn test_as_i64_with_null() {
            let json = r#"{"field": null}"#;
            let deserialized: TestI64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestI64Struct { field: 0 });
        }

        #[test]
        fn test_as_i64_with_max_value() {
            let json = format!(r#"{{"field": {}}}"#, i64::MAX);
            let deserialized: TestI64Struct = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, TestI64Struct { field: i64::MAX });
        }

        #[test]
        fn test_as_i64_with_overflow() {
            let json = format!(r#"{{"field": {}}}"#, u64::MAX);
            let deserialized = serde_json::from_str::<TestI64Struct>(&json);
            assert!(deserialized.is_err());
        }
    }

    mod as_u64_tests {
        use super::*;

        #[derive(Debug, PartialEq, Deserialize)]
        struct TestU64Struct {
            #[serde(deserialize_with = "as_u64")]
            field: u64,
        }

        #[test]
        fn test_as_u64_with_integer() {
            let json = r#"{"field": 123}"#;
            let deserialized: TestU64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestU64Struct { field: 123 });
        }

        #[test]
        fn test_as_u64_with_string() {
            let json = r#"{"field": "123"}"#;
            let deserialized: TestU64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestU64Struct { field: 123 });
        }

        #[test]
        fn test_as_u64_with_float() {
            let json = r#"{"field": 123.45}"#;
            let deserialized: TestU64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestU64Struct { field: 123 });
        }

        #[test]
        fn test_as_u64_with_null() {
            let json = r#"{"field": null}"#;
            let deserialized: TestU64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestU64Struct { field: 0 });
        }

        #[test]
        fn test_as_u64_with_negative_number() {
            let json = r#"{"field": -1}"#;
            let deserialized = serde_json::from_str::<TestU64Struct>(json);
            assert!(deserialized.is_err());
        }

        #[test]
        fn test_as_u64_with_large_value() {
            let json = format!(r#"{{"field": {}}}"#, u64::MAX);
            let deserialized: TestU64Struct = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, TestU64Struct { field: u64::MAX });
        }
    }
}
