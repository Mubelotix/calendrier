use std::collections::HashMap;

use chrono::prelude::*;


const REPUBLICAN_EPOCH_GREGORIAN_SECONDS: i64 = -5594228280;
const REPUBLICAN_SECONDS_PER_DAY: i64 = 100000;
const GREGORIAN_SECONDS_PER_DAY: i64 = 86400;
pub fn ts_from_unix(unix_timestamp: i64) -> i64 {
    let gregorian_seconds = unix_timestamp - REPUBLICAN_EPOCH_GREGORIAN_SECONDS;
    let republican_seconds = gregorian_seconds * REPUBLICAN_SECONDS_PER_DAY / GREGORIAN_SECONDS_PER_DAY;
    republican_seconds
}

fn main() {
    println!("cargo:rerun-if-changed=data/equinoxes.txt");
    println!("cargo:rerun-if-changed=src/equinoxes_pattern.rs");

    let mut equinoxes = HashMap::new();

    let data = std::fs::read_to_string("data/equinoxes.txt").expect("Could not read equinoxes.txt");
    for line in data.lines().filter(|l| !l.is_empty()) {
        let numbers = line.split(|c: char| !c.is_numeric()).filter(|s| !s.is_empty()).map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
        assert_eq!(numbers.len(), 11);
        let day = numbers[0];
        let month = numbers[1];
        let gregorian_year = numbers[2] as i32;
        let hour = numbers[3];
        let minute = numbers[4];
        let second = numbers[5];
        let datetime = Utc.with_ymd_and_hms(gregorian_year, month, day, hour, minute, second).single().unwrap();
        let ts = ts_from_unix(datetime.timestamp());

        let republican_year0 = gregorian_year - 1792;
        let republican_year = match republican_year0 >= 0 {
            true => republican_year0 + 1,
            false => republican_year0
        };

        equinoxes.insert(republican_year, ts);
    }

    let mut code = std::fs::read_to_string("src/equinoxes_pattern.rs").expect("Could not read src/equinoxes_pattern.rs");
    for gregorian_year in 1583..=2999 {
        let republican_year0 = gregorian_year - 1792;
        let republican_year = match republican_year0 >= 0 {
            true => republican_year0 + 1,
            false => republican_year0
        };
        let ts = equinoxes.get(&republican_year).expect(&format!("Could not find equinox for year {republican_year} (gregorian {gregorian_year})"));
        code = code.replace("EQUINOX,", format!("{ts}, EQUINOX,").as_str());
    }
    code = code.replace("EQUINOX,", "");
    std::fs::write("src/equinoxes.rs", code).expect("Could not write src/equinoxes.rs");
}