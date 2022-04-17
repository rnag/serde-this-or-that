use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json::from_str;

use serde_this_or_that::*;

#[derive(Clone, Debug, Deserialize)]
pub struct MsgCustom {
    #[serde(deserialize_with = "as_bool")]
    pub is_active: bool,
}

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

    let data = r#"
    {
        "is_active": "TRUE"
    }"#;

    c.bench_function("de custom (input: str)", |b| {
        b.iter(|| from_str::<MsgCustom>(black_box(data)).unwrap())
    });
    c.bench_function("de untagged (input: str)", |b| {
        b.iter(|| from_str::<MsgUntagged>(black_box(data)).unwrap())
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
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
