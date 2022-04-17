use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json::from_str;
use serde_with::{serde_as, DisplayFromStr, PickFirst};

use serde_this_or_that::*;

/// `MsgCustom` - uses a custom `Visitor` pattern with `serde`
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MsgCustom {
    #[serde(deserialize_with = "as_bool")]
    pub is_active: bool,
    #[serde(deserialize_with = "as_u64")]
    pub num_attempts: u64,
    #[serde(deserialize_with = "as_f64")]
    pub grade: f64,
}

/// `MsgWith` - uses a `PickFirst` approach via `serde_with`
///
/// # Note
/// Use `PickFirst` instead of just `DisplayFromStr`, so we can handle
/// alternate cases of `str` and `u64`; otherwise, `serde_with` appears
/// to lock in the *first type* it sees for a field.
#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MsgWith {
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

    c.bench_function("de custom", |b| {
        b.iter(|| from_str::<MsgCustom>(black_box(data)).unwrap())
    });
    c.bench_function("de serde_with", |b| {
        b.iter(|| from_str::<MsgWith>(black_box(data)).unwrap())
    });

    let data = r#"
    {
        "isActive": "true",
        "numAttempts": "123",
        "grade": "78.0"
    }"#;

    c.bench_function("de custom (input: str)", |b| {
        b.iter(|| from_str::<MsgCustom>(black_box(data)).unwrap())
    });
    c.bench_function("de serde_with (input: str)", |b| {
        b.iter(|| from_str::<MsgWith>(black_box(data)).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
