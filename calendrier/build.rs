use std::collections::HashMap;
use chrono::prelude::*;

const REPUBLICAN_EPOCH_GREGORIAN_SECONDS: i64 = -5594228280;
const REPUBLICAN_SECONDS_PER_DAY: i64 = 100000;
const GREGORIAN_SECONDS_PER_DAY: i64 = 86400;
pub fn ts_from_unix(unix_timestamp: i64) -> i64 {
    let gregorian_seconds = unix_timestamp - REPUBLICAN_EPOCH_GREGORIAN_SECONDS;
    gregorian_seconds * REPUBLICAN_SECONDS_PER_DAY / GREGORIAN_SECONDS_PER_DAY
}

fn parse_equinoxe(line: &str) -> (i32, i64) {
    let numbers = line
        .split(|c: char| !c.is_numeric())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(numbers.len(), 11);
    let day = numbers[0];
    let month = numbers[1];
    let gregorian_year = numbers[2] as i32;
    let hour = numbers[3];
    let minute = numbers[4];
    let second = numbers[5];
    let datetime = Utc
        .with_ymd_and_hms(gregorian_year, month, day, hour, minute, second)
        .single()
        .unwrap();
    (gregorian_year, ts_from_unix(datetime.timestamp()))
}

fn main() {
    println!("cargo:rerun-if-changed=data/equinoxes.txt");

    // Read equinoxes.txt
    let data = std::fs::read_to_string("data/equinoxes.txt").expect("Could not read equinoxes.txt");
    let equinoxes: HashMap<_, _> = data
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse_equinoxe)
        .collect();

    // Generate equinoxes.rs
    let timestamps = (1583..=2999)
        .map(|gregorian_year| {
            equinoxes.get(&gregorian_year).unwrap_or_else(|| {
                panic!("Could not find equinox for year (gregorian {gregorian_year})")
            })
        })
        .map(|ts| format!("    {ts},"))
        .collect::<Vec<_>>();

    let outdir = std::env::var("OUT_DIR").expect("Could not get OUT_DIR");
    let timestamps_as_string = timestamps.join("\n");
    let path = format!("{outdir}/equinoxes.rs");
    std::fs::write(
        path,
        format!("pub const TIMESTAMPS: &[i64] = &[\n{timestamps_as_string}\n];"),
    )
    .expect("Could not write {path}");
}
