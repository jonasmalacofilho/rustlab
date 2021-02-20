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

fn criterion_benchmark(c: &mut Criterion) {
    {
        let raw_status_data = r#"{"Device": "NZXT Smart Device (V1)", "Bus": "hid", "Address": "/dev/hidraw3", "Status": [["Fan 1", "PWM", ""], ["Fan 1 current", 0.02, "A"], ["Fan 1 speed", 1505, "rpm"], ["Fan 1 voltage", 11.91, "V"], ["Fan 2", "PWM", ""], ["Fan 2 current", 0.02, "A"], ["Fan 2 speed", 1368, "rpm"], ["Fan 2 voltage", 11.91, "V"], ["Fan 3", "PWM", ""], ["Fan 3 current", 0.03, "A"], ["Fan 3 speed", 1649, "rpm"], ["Fan 3 voltage", 11.91, "V"], ["Firmware version", "1.0.7", ""], ["LED accessories", 2, ""], ["LED accessory type", "HUE+ Strip", ""], ["LED count (total)", 20, ""], ["Noise level", 63, "dB"]]}"#;

        c.bench_function("raw_status_baseline", |b| {
            b.iter(|| raw_status_baseline(black_box(raw_status_data)))
        });
        c.bench_function("raw_status_filter_rpms", |b| {
            b.iter(|| raw_status_filter_rpms(black_box(raw_status_data)))
        });
        c.bench_function("raw_status_find_fan_rpms", |b| {
            b.iter(|| raw_status_find_fan_rpms(black_box(raw_status_data)))
        });

        dbg!(std::mem::size_of::<Response<RawStatus>>());
    }

    {
        let key_tuple_data = r#"{"Device": "NZXT Smart Device (V1)", "Bus": "hid", "Address": "/dev/hidraw3", "Status": {"Fan 1": ["PWM", ""], "Fan 1 current": [0.03, "A"], "Fan 1 speed": [1532, "rpm"], "Fan 1 voltage": [11.91, "V"], "Fan 2": ["PWM", ""], "Fan 2 current": [0.02, "A"], "Fan 2 speed": [1326, "rpm"], "Fan 2 voltage": [11.91, "V"], "Fan 3": ["PWM", ""], "Fan 3 current": [0.04, "A"], "Fan 3 speed": [1634, "rpm"], "Fan 3 voltage": [11.91, "V"], "Firmware version": ["1.0.7", ""], "LED accessories": [2, ""], "LED accessory type": ["HUE+ Strip", ""], "LED count (total)": [20, ""], "Noise level": [63, "dB"]}}"#;

        c.bench_function("key_tuple_baseline", |b| {
            b.iter(|| key_tuple_baseline(black_box(key_tuple_data)))
        });
        c.bench_function("key_tuple_filter_rpms", |b| {
            b.iter(|| key_tuple_filter_rpms(black_box(key_tuple_data)))
        });
        c.bench_function("key_tuple_find_fan_rpms", |b| {
            b.iter(|| key_tuple_find_fan_rpms(black_box(key_tuple_data)))
        });

        dbg!(std::mem::size_of::<Response<KeyTupleStatus>>());
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
