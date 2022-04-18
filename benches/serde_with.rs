use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json::from_str;
use serde_with::{serde_as, DisplayFromStr, PickFirst};

use serde_this_or_that::*;

/// `MsgSerdeThisOrThat` - uses a custom `Visitor` pattern with `serde`
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MsgSerdeThisOrThat {
    #[serde(deserialize_with = "as_bool")]
    pub is_active: bool,
    #[serde(deserialize_with = "as_u64")]
    pub num_attempts: u64,
    #[serde(deserialize_with = "as_f64")]
    pub grade: f64,
}

/// `MsgSerdeWithFromStr` - uses a `DisplayFromStr` approach via `serde_with`
#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MsgSerdeWithFromStr {
    #[serde_as(as = "DisplayFromStr")]
    pub is_active: bool,
    #[serde_as(as = "DisplayFromStr")]
    pub num_attempts: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub grade: f64,
}

/// `MsgSerdeWithPickFirst` - uses a `PickFirst` approach via `serde_with`
///
/// # Note
/// Use `PickFirst` instead of just `DisplayFromStr`, so we can handle
/// alternate cases of `str` and `u64`; otherwise, `serde_with` appears
/// to lock in the *first type* it sees for a field.
#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MsgSerdeWithPickFirst {
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub is_active: bool,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub num_attempts: u64,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub grade: f64,
}

fn criterion_benchmark(c: &mut Criterion) {
    let data = r#"
    {
        "isActive": false,
        "numAttempts": 3,
        "grade": 81
    }"#;

    c.bench_function("de: serde_this_or_that   ", |b| {
        b.iter(|| from_str::<MsgSerdeThisOrThat>(black_box(data)).unwrap())
    });
    c.bench_function("de: serde_with::PickFirst", |b| {
        b.iter(|| from_str::<MsgSerdeWithPickFirst>(black_box(data)).unwrap())
    });

    let data = r#"
    {
        "isActive": "true",
        "numAttempts": "123",
        "grade": "78.0"
    }"#;

    c.bench_function("de: serde_this_or_that         (input: str)", |b| {
        b.iter(|| from_str::<MsgSerdeThisOrThat>(black_box(data)).unwrap())
    });
    c.bench_function("de: serde_with::PickFirst      (input: str)", |b| {
        b.iter(|| from_str::<MsgSerdeWithPickFirst>(black_box(data)).unwrap())
    });
    c.bench_function("de serde_with::DisplayFromStr  (input: str)", |b| {
        b.iter(|| from_str::<MsgSerdeWithFromStr>(black_box(data)).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
