use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use serde::Deserialize;

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct Response<'a, T> {
    pub Device: &'a str,
    pub Bus: &'a str,
    pub Address: &'a str,
    pub Status: T,
}

type RawStatus<'a> = Vec<(&'a str, StatusValue<'a>, &'a str)>;

type KeyTupleStatus<'a> = HashMap<&'a str, (StatusValue<'a>, &'a str)>;

#[derive(Deserialize)]
struct FieldFullStatusItem<'a> {
    pub key: &'a str,
    pub value: StatusValue<'a>,
    pub unit: &'a str,
}

type FieldFullStatus<'a> = Vec<FieldFullStatusItem<'a>>;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum StatusValue<'a> {
    Text(&'a str),
    Number(f64),
}

fn raw_status_baseline<'a>(data: &str) {
    let response = serde_json::from_str::<Response<RawStatus>>(data).unwrap();

    assert_eq!(response.Status.len(), 17);
}

fn raw_status_filter_rpms<'a>(data: &str) {
    let response = serde_json::from_str::<Response<RawStatus>>(data).unwrap();

    assert_eq!(response.Status.iter().filter(|x| x.2 == "rpm").count(), 3);
}

fn raw_status_find_fan_rpms<'a>(data: &str) {
    let response = serde_json::from_str::<Response<RawStatus>>(data).unwrap();

    let fan_speed = |key| match response.Status.iter().find(|x| x.0 == key).unwrap().1 {
        StatusValue::Number(speed) => speed,
        _ => unreachable!(),
    };

    assert_eq!(fan_speed("Fan 1 speed"), 1505.);
    assert_eq!(fan_speed("Fan 2 speed"), 1368.);
    assert_eq!(fan_speed("Fan 3 speed"), 1649.);
}

fn key_tuple_baseline<'a>(data: &str) {
    let response = serde_json::from_str::<Response<KeyTupleStatus>>(data).unwrap();

    assert_eq!(response.Status.len(), 17);
}

fn key_tuple_filter_rpms<'a>(data: &str) {
    let response = serde_json::from_str::<Response<KeyTupleStatus>>(data).unwrap();

    assert_eq!(
        response
            .Status
            .iter()
            .filter(|(_, (_, u))| *u == "rpm")
            .count(),
        3
    );
}

fn key_tuple_find_fan_rpms<'a>(data: &str) {
    let response = serde_json::from_str::<Response<KeyTupleStatus>>(data).unwrap();

    let fan_speed = |key| match response.Status[key].0 {
        StatusValue::Number(speed) => speed,
        _ => unreachable!(),
    };

    assert_eq!(fan_speed("Fan 1 speed"), 1532.);
    assert_eq!(fan_speed("Fan 2 speed"), 1326.);
    assert_eq!(fan_speed("Fan 3 speed"), 1634.);
}

fn field_full_baseline<'a>(data: &str) {
    let response = serde_json::from_str::<Response<FieldFullStatus>>(data).unwrap();

    assert_eq!(response.Status.len(), 17);
}

fn field_full_filter_rpms<'a>(data: &str) {
    let response = serde_json::from_str::<Response<FieldFullStatus>>(data).unwrap();

    assert_eq!(
        response.Status.iter().filter(|x| x.unit == "rpm").count(),
        3
    );
}

fn field_full_find_fan_rpms<'a>(data: &str) {
    let response = serde_json::from_str::<Response<FieldFullStatus>>(data).unwrap();

    let fan_speed = |key| match response.Status.iter().find(|x| x.key == key).unwrap().value {
        StatusValue::Number(speed) => speed,
        _ => unreachable!(),
    };

    assert_eq!(fan_speed("Fan 1 speed"), 1505.);
    assert_eq!(fan_speed("Fan 2 speed"), 1368.);
    assert_eq!(fan_speed("Fan 3 speed"), 1649.);
}

fn criterion_benchmark(c: &mut Criterion) {
    use std::mem::size_of;

    dbg!(size_of::<Response<RawStatus>>(), size_of::<RawStatus>());
    dbg!(
        size_of::<Response<KeyTupleStatus>>(),
        size_of::<KeyTupleStatus>()
    );
    dbg!(
        size_of::<Response<FieldFullStatus>>(),
        size_of::<FieldFullStatus>()
    );

    {
        let raw_status_data = include_str!("../data/mock_liquidctl_raw.json");

        c.bench_function("raw_status_baseline", |b| {
            b.iter(|| raw_status_baseline(black_box(raw_status_data)))
        });
        c.bench_function("raw_status_filter_rpms", |b| {
            b.iter(|| raw_status_filter_rpms(black_box(raw_status_data)))
        });
        c.bench_function("raw_status_find_fan_rpms", |b| {
            b.iter(|| raw_status_find_fan_rpms(black_box(raw_status_data)))
        });
    }

    {
        let key_tuple_data = include_str!("../data/mock_liquidctl_key_tuple.json");

        c.bench_function("key_tuple_baseline", |b| {
            b.iter(|| key_tuple_baseline(black_box(key_tuple_data)))
        });
        c.bench_function("key_tuple_filter_rpms", |b| {
            b.iter(|| key_tuple_filter_rpms(black_box(key_tuple_data)))
        });
        c.bench_function("key_tuple_find_fan_rpms", |b| {
            b.iter(|| key_tuple_find_fan_rpms(black_box(key_tuple_data)))
        });
    }

    {
        let field_full = include_str!("../data/mock_liquidctl_field_full.json");

        c.bench_function("field_full_baseline", |b| {
            b.iter(|| field_full_baseline(black_box(field_full)))
        });
        c.bench_function("field_full_filter_rpms", |b| {
            b.iter(|| field_full_filter_rpms(black_box(field_full)))
        });
        c.bench_function("field_full_find_fan_rpms", |b| {
            b.iter(|| field_full_find_fan_rpms(black_box(field_full)))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
