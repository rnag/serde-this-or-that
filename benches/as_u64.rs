use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde::Deserializer;
use serde_json::from_str;

use serde_this_or_that::*;

#[derive(Clone, Debug, Deserialize)]
pub struct MsgCustom {
    #[serde(deserialize_with = "as_u64")]
    pub timestamp: u64,
}

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

    c.bench_function("de custom (input: u64)", |b| {
        b.iter(|| from_str::<MsgCustom>(black_box(data)).unwrap())
    });
    c.bench_function("de untagged (input: u64)", |b| {
        b.iter(|| from_str::<MsgUntagged>(black_box(data)).unwrap())
    });

    let data = r#"
    {
        "timestamp": "1650057633185497"
    }"#;

    c.bench_function("de custom (input: str)", |b| {
        b.iter(|| from_str::<MsgCustom>(black_box(data)).unwrap())
    });
    c.bench_function("de untagged (input: str)", |b| {
        b.iter(|| from_str::<MsgUntagged>(black_box(data)).unwrap())
    });

    let data = r#"
    {
        "timestamp": 123.45
    }"#;

    c.bench_function("de custom (input: f64)", |b| {
        b.iter(|| from_str::<MsgCustom>(black_box(data)).unwrap())
    });
    c.bench_function("de untagged (input: f64)", |b| {
        b.iter(|| from_str::<MsgUntagged>(black_box(data)).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
