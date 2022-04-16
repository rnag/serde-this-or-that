use std::{f64, fmt};

use crate::de;
use crate::de::{Deserializer, Unexpected};

/// De-serialize either a `str`, `i64`, `f64`, or `u64`
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
    deserializer.deserialize_any(DeserializeI64OrStringVisitor)
}

/// De-serialize either a `str`, `u64`, `f64`, or `i64`
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
    deserializer.deserialize_any(DeserializeU64OrStringVisitor)
}

/// De-serialize either a `str`, `f64`, `u64`, or `i64` as a float value.
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
    deserializer.deserialize_any(DeserializeF64OrStringVisitor)
}

/// De-serialize either a `bool`, `str`, `u64`, or `f64` as a boolean value.
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
/// The boolean (`bool`) value of a string or number.
///
pub fn as_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeBoolFromStringOrU64Visitor)
}

/// TODO maybe update these definitions into a macro ..?

struct DeserializeU64OrStringVisitor;

impl<'de> de::Visitor<'de> for DeserializeU64OrStringVisitor {
    type Value = u64;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an unsigned integer or a string")
    }

    fn visit_i64<E>(self, v: i64) -> Result<u64, E>
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

    fn visit_u64<E>(self, v: u64) -> Result<u64, E>
    where
        E: de::Error,
    {
        Ok(v)
    }

    fn visit_f64<E>(self, v: f64) -> Result<u64, E>
    where
        E: de::Error,
    {
        Ok(v as u64)
    }

    fn visit_str<E>(self, v: &str) -> Result<u64, E>
    where
        E: de::Error,
    {
        match v.parse::<u64>() {
            Ok(s) => Ok(s),
            Err(_) => {
                if v.is_empty() {
                    Ok(0)
                } else {
                    Err(E::invalid_value(Unexpected::Str(v), &self))
                }
            }
        }
    }
}

struct DeserializeI64OrStringVisitor;

impl<'de> de::Visitor<'de> for DeserializeI64OrStringVisitor {
    type Value = i64;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a signed integer or a string")
    }

    fn visit_i64<E>(self, v: i64) -> Result<i64, E>
    where
        E: de::Error,
    {
        Ok(v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<i64, E>
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

    fn visit_f64<E>(self, v: f64) -> Result<i64, E>
    where
        E: de::Error,
    {
        Ok(v as i64)
    }

    fn visit_str<E>(self, v: &str) -> Result<i64, E>
    where
        E: de::Error,
    {
        match v.parse::<i64>() {
            Ok(s) => Ok(s),
            Err(_) => {
                if v.is_empty() {
                    Ok(0)
                } else {
                    Err(E::invalid_value(Unexpected::Str(v), &self))
                }
            }
        }
    }
}

struct DeserializeF64OrStringVisitor;

impl<'de> de::Visitor<'de> for DeserializeF64OrStringVisitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a float or a string")
    }

    fn visit_i64<E>(self, v: i64) -> Result<f64, E>
    where
        E: de::Error,
    {
        Ok(v as f64)
    }

    fn visit_u64<E>(self, v: u64) -> Result<f64, E>
    where
        E: de::Error,
    {
        Ok(v as f64)
    }

    fn visit_f64<E>(self, v: f64) -> Result<f64, E>
    where
        E: de::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<f64, E>
    where
        E: de::Error,
    {
        match v.parse::<f64>() {
            Ok(s) => Ok(s),
            Err(_) => {
                if v.is_empty() {
                    Ok(0.0)
                } else {
                    Err(E::invalid_value(Unexpected::Str(v), &self))
                }
            }
        }
    }
}

struct DeserializeBoolFromStringOrU64Visitor;

impl<'de> de::Visitor<'de> for DeserializeBoolFromStringOrU64Visitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an integer (0 or 1) or a string")
    }

    fn visit_bool<E>(self, v: bool) -> Result<bool, E>
    where
        E: de::Error,
    {
        Ok(v)
    }

    fn visit_i64<E>(self, v: i64) -> Result<bool, E>
    where
        E: de::Error,
    {
        Err(de::Error::invalid_value(
            Unexpected::Signed(v),
            &"zero or one",
        ))
    }

    fn visit_u64<E>(self, v: u64) -> Result<bool, E>
    where
        E: de::Error,
    {
        match v {
            0 => Ok(false),
            1 => Ok(true),
            other => Err(de::Error::invalid_value(
                Unexpected::Unsigned(other as u64),
                &"zero or one",
            )),
        }
    }

    fn visit_f64<E>(self, v: f64) -> Result<bool, E>
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

    fn visit_str<E>(self, v: &str) -> Result<bool, E>
    where
        E: de::Error,
    {
        match v.to_uppercase().as_str() {
            "1" | "OK" | "T" | "TRUE" | "Y" | "YES" => Ok(true),
            _ => Ok(false),
        }
    }
}
