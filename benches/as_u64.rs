use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json::from_str;
use serde_with::{serde_as, DisplayFromStr, PickFirst};

use serde_this_or_that::*;

/// `MsgCustom` - uses a custom `Visitor` pattern with `serde`
#[derive(Clone, Debug, Deserialize)]
pub struct MsgCustom {
    #[serde(deserialize_with = "as_u64")]
    pub timestamp: u64,
}

/// `MsgWith` - uses a `PickFirst` approach via `serde_with`
///
/// # Note
/// Use `PickFirst` instead of just `DisplayFromStr`, so we can handle
/// alternate cases of `str` and `u64`; otherwise, `serde_with` appears
/// to lock in the *first type* it sees for a field.
#[serde_as]
#[derive(Deserialize)]
pub struct MsgWith {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub timestamp: u64,
}

///  `MsgUntagged` - uses an *untagged enum* approach with `serde`
#[derive(Clone, Debug, Deserialize)]
pub struct MsgUntagged {
    #[serde(deserialize_with = "as_u64_untagged")]
    pub timestamp: u64,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum MyCustomType<'a> {
    Str(&'a str),
    U64(u64),
    I64(i64),
    F64(f64),
}

pub fn as_u64_untagged<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match MyCustomType::deserialize(deserializer)? {
        MyCustomType::U64(v) => v,
        MyCustomType::Str(v) => v.parse().unwrap_or(0), // Ignoring parsing errors
        MyCustomType::I64(v) => v as u64,
        MyCustomType::F64(v) => v as u64,
    })
}

fn criterion_benchmark(c: &mut Criterion) {
    let data = r#"
    {
        "timestamp": 12345
    }"#;

    c.bench_function("de: custom      (input: u64)", |b| {
        b.iter(|| from_str::<MsgCustom>(black_box(data)).unwrap())
    });
    c.bench_function("de: untagged    (input: u64)", |b| {
        b.iter(|| from_str::<MsgUntagged>(black_box(data)).unwrap())
    });
    c.bench_function("de: serde_with  (input: u64)", |b| {
        b.iter(|| from_str::<MsgWith>(black_box(data)).unwrap())
    });

    let data = r#"
    {
        "timestamp": ""
    }"#;

    c.bench_function("de: custom    (input: str <empty>)", |b| {
        b.iter(|| from_str::<MsgCustom>(black_box(data)).unwrap())
    });
    c.bench_function("de: untagged  (input: str <empty>)", |b| {
        b.iter(|| from_str::<MsgUntagged>(black_box(data)).unwrap())
    });
    // TODO It looks like `serde_with` chokes at empty string values
    //   in this scenario currently.
    // c.bench_function("de serde_with (input: str <empty>)", |b| {
    //     b.iter(|| from_str::<MsgWith>(black_box(data)).unwrap())
    // });

    let data = r#"
    {
        "timestamp": "1650057633185497"
    }"#;

    c.bench_function("de: custom      (input: str)", |b| {
        b.iter(|| from_str::<MsgCustom>(black_box(data)).unwrap())
    });
    c.bench_function("de: untagged    (input: str)", |b| {
        b.iter(|| from_str::<MsgUntagged>(black_box(data)).unwrap())
    });
    c.bench_function("de: serde_with  (input: str)", |b| {
        b.iter(|| from_str::<MsgWith>(black_box(data)).unwrap())
    });

    let data = r#"
    {
        "timestamp": 123.45
    }"#;

    c.bench_function("de: custom    (input: f64)", |b| {
        b.iter(|| from_str::<MsgCustom>(black_box(data)).unwrap())
    });
    c.bench_function("de: untagged  (input: f64)", |b| {
        b.iter(|| from_str::<MsgUntagged>(black_box(data)).unwrap())
    });
    // TODO I think `serde_with` doesn't support de-serializing `float`
    //   values to a `u64` currently.
    // c.bench_function("de: serde_with (input: f64)", |b| {
    //     b.iter(|| from_str::<MsgWith>(black_box(data)).unwrap())
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
