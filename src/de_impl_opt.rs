use std::{f64, fmt};

use crate::de::{self, Deserializer};

/// De-serialize either a `str`, `i64`, `f64`, or `u64`
/// as a *signed* value wrapped in [`Some`],
/// and a `bool` or `null` value as [`None`].
///
/// # Returns
/// A [`Some`] with the signed (`i64`) value of a string
/// or number.
///
/// A [`None`] in the case of:
///   * a `bool` value.
///   * a `null` value.
///   * any *de-serialization* errors.
///     * ex. a string is non-empty and not a valid numeric value.
///     * ex. the unsigned value `u64` *overflows* when converted to `i64`.
///
pub fn as_opt_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeOptionalI64WithVisitor)
}

/// De-serialize either a `str`, `u64`, `f64`, or `i64`
/// as an *unsigned* value wrapped in [`Some`],
/// and a `bool` or `null` value as [`None`].
///
/// # Returns
/// A [`Some`] with the unsigned (`u64`) value of a string
/// or number.
///
/// A [`None`] in the case of:
///   * a `bool` value.
///   * a `null` value.
///   * any *de-serialization* errors.
///     * ex. a string is non-empty and not a valid numeric value.
///     * ex. the signed value `i64` represents a *negative* number.
///     * ex. float `f64` represents a *negative* number `< -0.5`, or `NaN`.
///
pub fn as_opt_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeOptionalU64WithVisitor)
}

/// De-serialize either a `str`, `f64`, `u64`, or `i64`
/// as a *float* value wrapped in [`Some`],
/// and a `bool` or `null` value as [`None`].
///
/// # Returns
/// A [`Some`] with the floating point (`f64`) value of a string
/// or number.
///
/// A [`None`] in the case of:
///   * a `bool` value.
///   * a `null` value.
///   * any *de-serialization* errors.
///     * ex. a string is non-empty and not a valid numeric value.
///
pub fn as_opt_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeOptionalF64WithVisitor)
}

/// De-serialize either a `bool`, `str`, `u64`, or `f64`
/// as a *boolean* value wrapped in [`Some`],
/// and an `i64` or `null` value as [`None`].
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
/// # Falsy String Values
/// > Note: the pattern matching is *case insensitive*, so `NO` or `no`
/// > works just the same.
///
/// These are the following "falsy" string values that result in a
/// boolean value of `false`:
///
///   - `0`
///   - `NG` ([antonym for `OK`](https://english.stackexchange.com/a/586568/461000))
///   - `OFF`
///   - `F`
///   - `FALSE`
///   - `N`
///   - `NO`
///
/// # Returns
/// A [`Some`] with the boolean (`bool`) value of a string,
/// boolean, or number.
///
/// A [`None`] in the case of:
///   * a `str` value which does not match any of the ["truthy"](#truthy-string-values)
///     or ["falsy"](#falsy-string-values) values as defined above.
///   * an `i64` value.
///   * a `null` value.
///   * any *de-serialization* errors.
///     * ex. an unsigned `u64` or a float `f64` value is not a *zero* or a *one*.
///
pub fn as_opt_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeOptionalBoolWithVisitor)
}

/// De-serialize either a `str`, `bool`, `i64`, `f64`, or `u64`
/// as an (owned) *string* value wrapped in [`Some`],
/// and a `null` value as [`None`].
///
/// # Returns
/// A [`Some`] with the owned `String` value of a string,
/// boolean, or number.
///
/// A [`None`] in the case of:
///   * a `null` value.
///   * any *de-serialization* errors.
///
pub fn as_opt_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeOptionalStringWithVisitor)
}

// TODO maybe update these definitions into a macro ..?

struct DeserializeOptionalU64WithVisitor;

impl de::Visitor<'_> for DeserializeOptionalU64WithVisitor {
    type Value = Option<u64>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an unsigned integer or a string")
    }

    fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(u64::try_from(v).ok())
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(v.round() as u64))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(n) = v.parse::<u64>() {
            Ok(Some(n))
        } else if v.is_empty() {
            Ok(None)
        } else if let Ok(f) = v.parse::<f64>() {
            Ok(Some(f.round() as u64))
        } else {
            Ok(None)
        }
    }

    /// We encounter a `null` value; this default implementation returns an
    /// `Option::None` value.
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

struct DeserializeOptionalI64WithVisitor;

impl de::Visitor<'_> for DeserializeOptionalI64WithVisitor {
    type Value = Option<i64>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a signed integer or a string")
    }

    fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(i64::try_from(v).ok())
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(v.round() as i64))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(n) = v.parse::<i64>() {
            Ok(Some(n))
        } else if v.is_empty() {
            Ok(None)
        } else if let Ok(f) = v.parse::<f64>() {
            Ok(Some(f.round() as i64))
        } else {
            Ok(None)
        }
    }

    /// We encounter a `null` value; this default implementation returns an
    /// `Option::None` value.
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

struct DeserializeOptionalF64WithVisitor;

impl de::Visitor<'_> for DeserializeOptionalF64WithVisitor {
    type Value = Option<f64>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a float or a string")
    }

    fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(v as f64))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(v as f64))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.parse::<f64>().ok())
    }

    /// We encounter a `null` value; this default implementation returns an
    /// `Option::None` value.
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

struct DeserializeOptionalBoolWithVisitor;

impl de::Visitor<'_> for DeserializeOptionalBoolWithVisitor {
    type Value = Option<bool>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an integer (0 or 1) or a string")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(v))
    }

    fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        // needs a zero or one, just return `None` here
        Ok(None)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v {
            0 => Ok(Some(false)),
            1 => Ok(Some(true)),
            // needs a zero or one, just return `None` here
            _ => Ok(None),
        }
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v as u8 {
            0 => Ok(Some(false)),
            1 => Ok(Some(true)),
            // needs a zero or one, just return `None` here
            _ => Ok(None),
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        // First, try to match common true/false phrases *without*
        // using `to_uppercase()`. This approach is likely more efficient.
        match v {
            "t" | "T" | "true" | "True" | "1" => Ok(Some(true)),
            "f" | "F" | "false" | "False" | "0" => Ok(Some(false)),
            other => {
                // So from the above, we've already matched the following
                // "truthy" phrases: ["T", "1"]
                // and the following "falsy" phrases: ["F", "0"].
                // To be completely thorough, we also need to do a case-
                // insensitive match on ["OK", "ON", "TRUE", "Y", "YES"]
                // and its counterpart, ["NG", "OFF", "FALSE", "N", "NO"].
                match other.to_uppercase().as_str() {
                    "OK" | "ON" | "TRUE" | "Y" | "YES" => Ok(Some(true)),
                    "NG" | "OFF" | "FALSE" | "N" | "NO" => Ok(Some(false)),
                    _ => Ok(None),
                }
            }
        }
    }

    /// We encounter a `null` value; this default implementation returns an
    /// `Option::None` value.
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

struct DeserializeOptionalStringWithVisitor;

impl de::Visitor<'_> for DeserializeOptionalStringWithVisitor {
    type Value = Option<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string, bool, or a number")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(v.to_string()))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(v.to_string()))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(v.to_string()))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(v.to_string()))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(v.to_owned()))
    }

    /// We encounter a `null` value; this default implementation returns an
    /// `Option::None` value.
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    // Tests for as_opt_bool
    mod as_opt_bool_tests {
        use super::*;

        #[derive(Debug, PartialEq, Deserialize)]
        struct TestOptBoolStruct {
            #[serde(deserialize_with = "as_opt_bool")]
            field: Option<bool>,
        }

        #[test]
        fn test_as_opt_bool_with_truthy_values() {
            let truthy_values = ["1", "OK", "ON", "T", "TRUE", "Y", "YES"];
            for value in truthy_values {
                let json = format!(r#"{{"field": "{}"}}"#, value);
                let deserialized: TestOptBoolStruct = serde_json::from_str(&json).unwrap();
                assert_eq!(deserialized, TestOptBoolStruct { field: Some(true) });
            }
        }

        #[test]
        fn test_as_opt_bool_with_falsy_values() {
            let falsy_values = ["0", "OFF", "F", "FALSE", "N", "NO"];
            for value in falsy_values {
                let json = format!(r#"{{"field": "{}"}}"#, value);
                let deserialized: TestOptBoolStruct = serde_json::from_str(&json).unwrap();
                assert_eq!(deserialized, TestOptBoolStruct { field: Some(false) });
            }
        }

        #[test]
        fn test_as_opt_bool_with_null() {
            let json = r#"{"field": null}"#;
            let deserialized: TestOptBoolStruct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestOptBoolStruct { field: None });
        }

        #[test]
        fn test_as_opt_bool_with_invalid() {
            let json = r#"{"field": "INVALID"}"#;
            let deserialized: TestOptBoolStruct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestOptBoolStruct { field: None });
        }
    }

    // Tests for as_opt_f64
    mod as_opt_f64_tests {
        use super::*;

        #[derive(Debug, PartialEq, Deserialize)]
        struct TestOptF64Struct {
            #[serde(deserialize_with = "as_opt_f64")]
            field: Option<f64>,
        }

        #[test]
        fn test_as_opt_f64_with_large_number() {
            let json = r#"{"field": 1e308}"#;
            let deserialized: TestOptF64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestOptF64Struct { field: Some(1e308) });
        }

        #[test]
        fn test_as_opt_f64_with_negative_number() {
            let json = r#"{"field": -123.45}"#;
            let deserialized: TestOptF64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(
                deserialized,
                TestOptF64Struct {
                    field: Some(-123.45)
                }
            );
        }

        #[test]
        fn test_as_opt_f64_with_number() {
            let json = r#"{"field": 123.45}"#;
            let deserialized: TestOptF64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(
                deserialized,
                TestOptF64Struct {
                    field: Some(123.45)
                }
            );
        }

        #[test]
        fn test_as_opt_f64_with_integer() {
            let json = r#"{"field": 123}"#;
            let deserialized: TestOptF64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestOptF64Struct { field: Some(123.0) });
        }

        #[test]
        fn test_as_opt_f64_with_null() {
            let json = r#"{"field": null}"#;
            let deserialized: TestOptF64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestOptF64Struct { field: None });
        }

        #[test]
        fn test_as_opt_f64_with_invalid_string() {
            let json = r#"{"field": "INVALID"}"#;
            let deserialized: TestOptF64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestOptF64Struct { field: None });
        }
    }

    // Tests for as_opt_i64
    mod as_opt_i64_tests {
        use super::*;

        #[derive(Debug, PartialEq, Deserialize)]
        struct TestOptI64Struct {
            #[serde(deserialize_with = "as_opt_i64")]
            field: Option<i64>,
        }

        #[test]
        fn test_as_opt_i64_with_integer() {
            let json = r#"{"field": 123}"#;
            let deserialized: TestOptI64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestOptI64Struct { field: Some(123) });
        }

        #[test]
        fn test_as_opt_i64_with_string() {
            let json = r#"{"field": "123"}"#;
            let deserialized: TestOptI64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestOptI64Struct { field: Some(123) });
        }

        #[test]
        fn test_as_opt_i64_with_float() {
            let json = r#"{"field": 123.45}"#;
            let deserialized: TestOptI64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestOptI64Struct { field: Some(123) });
        }

        #[test]
        fn test_as_opt_i64_with_null() {
            let json = r#"{"field": null}"#;
            let deserialized: TestOptI64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestOptI64Struct { field: None });
        }

        #[test]
        fn test_as_opt_i64_with_invalid() {
            let json = r#"{"field": "INVALID"}"#;
            let deserialized: TestOptI64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestOptI64Struct { field: None });
        }
    }

    // Tests for as_opt_string
    mod as_opt_string_tests {
        use super::*;

        #[derive(Debug, PartialEq, Deserialize)]
        struct TestOptStringStruct {
            #[serde(deserialize_with = "as_opt_string")]
            field: Option<String>,
        }

        #[test]
        fn test_as_opt_string_with_string() {
            let json = r#"{"field": "Hello"}"#;
            let deserialized: TestOptStringStruct = serde_json::from_str(json).unwrap();
            assert_eq!(
                deserialized,
                TestOptStringStruct {
                    field: Some("Hello".to_owned())
                }
            );
        }

        #[test]
        fn test_as_opt_string_with_number() {
            let json = r#"{"field": 123}"#;
            let deserialized: TestOptStringStruct = serde_json::from_str(json).unwrap();
            assert_eq!(
                deserialized,
                TestOptStringStruct {
                    field: Some("123".to_owned())
                }
            );
        }

        #[test]
        fn test_as_opt_string_with_boolean() {
            let json = r#"{"field": true}"#;
            let deserialized: TestOptStringStruct = serde_json::from_str(json).unwrap();
            assert_eq!(
                deserialized,
                TestOptStringStruct {
                    field: Some("true".to_owned())
                }
            );
        }

        #[test]
        fn test_as_opt_string_with_null() {
            let json = r#"{"field": null}"#;
            let deserialized: TestOptStringStruct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestOptStringStruct { field: None });
        }
    }

    // Tests for as_opt_u64
    mod as_opt_u64_tests {
        use super::*;

        #[derive(Debug, PartialEq, Deserialize)]
        struct TestOptU64Struct {
            #[serde(deserialize_with = "as_opt_u64")]
            field: Option<u64>,
        }

        #[test]
        fn test_as_opt_u64_with_integer() {
            let json = r#"{"field": 123}"#;
            let deserialized: TestOptU64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestOptU64Struct { field: Some(123) });
        }

        #[test]
        fn test_as_opt_u64_with_string() {
            let json = r#"{"field": "123"}"#;
            let deserialized: TestOptU64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestOptU64Struct { field: Some(123) });
        }

        #[test]
        fn test_as_opt_u64_with_float() {
            let json = r#"{"field": 123.45}"#;
            let deserialized: TestOptU64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestOptU64Struct { field: Some(123) });
        }

        #[test]
        fn test_as_opt_u64_with_null() {
            let json = r#"{"field": null}"#;
            let deserialized: TestOptU64Struct = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, TestOptU64Struct { field: None });
        }

        #[test]
        fn test_as_opt_u64_with_negative() {
            let json = r#"{"field": -1}"#;
            let deserialized = serde_json::from_str::<TestOptU64Struct>(json).unwrap();
            assert_eq!(deserialized, TestOptU64Struct { field: None });
        }
    }
}
