use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json::from_str;
use serde_with::{serde_as, DisplayFromStr, PickFirst};

use serde_this_or_that::*;

/// `MsgCustom` - uses a custom `Visitor` pattern with `serde`
#[derive(Clone, Debug, Deserialize)]
pub struct MsgCustom {
    #[serde(deserialize_with = "as_bool")]
    pub is_active: bool,
}

/// `MsgWith` - uses a `PickFirst` approach via `serde_with`
///
/// # Note
/// Use `PickFirst` instead of just `DisplayFromStr`, so we can handle
/// alternate cases of `str` and `bool`; otherwise, `serde_with` appears
/// to lock in the *first type* it sees for a field.
#[serde_as]
#[derive(Deserialize)]
pub struct MsgWith {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub is_active: bool,
}

///  `MsgUntagged` - uses an *untagged enum* approach with `serde`
#[derive(Clone, Debug, Deserialize)]
pub struct MsgUntagged {
    #[serde(deserialize_with = "as_bool_untagged")]
    pub is_active: bool,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum MyCustomType<'a> {
    Str(&'a str),
    Bool(bool),
    U64(u64),
}

pub fn as_bool_untagged<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match MyCustomType::deserialize(deserializer)? {
        MyCustomType::Bool(v) => Ok(v),
        MyCustomType::Str(v) => match v.to_uppercase().as_str() {
            "1" | "OK" | "T" | "TRUE" | "Y" | "YES" => Ok(true),
            _ => Ok(false),
        },
        MyCustomType::U64(v) => match v {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(de::Error::custom("invalid value, need a zero or one")),
        },
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let data = r#"
    {
        "is_active": false
    }"#;

    c.bench_function("de custom (input: bool)", |b| {
        b.iter(|| from_str::<MsgCustom>(black_box(data)).unwrap())
    });
    c.bench_function("de untagged (input: bool)", |b| {
        b.iter(|| from_str::<MsgUntagged>(black_box(data)).unwrap())
    });
    c.bench_function("de serde_with (input: bool)", |b| {
        b.iter(|| from_str::<MsgWith>(black_box(data)).unwrap())
    });

    let data = r#"
    {
        "is_active": "true"
    }"#;

    c.bench_function("de custom (input: str)", |b| {
        b.iter(|| from_str::<MsgCustom>(black_box(data)).unwrap())
    });
    c.bench_function("de untagged (input: str)", |b| {
        b.iter(|| from_str::<MsgUntagged>(black_box(data)).unwrap())
    });
    c.bench_function("de serde_with (input: str)", |b| {
        b.iter(|| from_str::<MsgWith>(black_box(data)).unwrap())
    });

    let data = r#"
    {
        "is_active": 1
    }"#;

    c.bench_function("de custom (input: u64)", |b| {
        b.iter(|| from_str::<MsgCustom>(black_box(data)).unwrap())
    });
    c.bench_function("de untagged (input: u64)", |b| {
        b.iter(|| from_str::<MsgUntagged>(black_box(data)).unwrap())
    });
    // TODO: I think `serde_with` doesn't support converting `u64` (zero
    //   or one) to a `bool` currently.
    // c.bench_function("de serde_with (input: u64)", |b| {
    //     b.iter(|| from_str::<MsgWith>(black_box(data)).unwrap())
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
