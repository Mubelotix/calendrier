use std::collections::HashMap;

use chrono::prelude::*;


const REPUBLICAN_EPOCH_GREGORIAN_SECONDS: i64 = -5594228280;
const REPUBLICAN_SECONDS_PER_DAY: i64 = 100000;
const GREGORIAN_SECONDS_PER_DAY: i64 = 86400;
pub fn ts_from_unix(unix_timestamp: i64) -> i64 {
    let gregorian_seconds = unix_timestamp - REPUBLICAN_EPOCH_GREGORIAN_SECONDS;
    gregorian_seconds * REPUBLICAN_SECONDS_PER_DAY / GREGORIAN_SECONDS_PER_DAY
}

fn main() {
    println!("cargo:rerun-if-changed=data/equinoxes.txt");
    println!("cargo:rerun-if-changed=src/equinoxes_pattern.rs");
    println!("cargo:rerun-if-changed=src/years_pattern.rs");
    let out_dir = std::env::var("OUT_DIR").expect("Could not find OUT_DIR");

    let mut equinoxes = HashMap::new();
    let mut year_starts = HashMap::new();

    // Read equinoxes.txt
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

    // Generate equinoxes.rs
    let mut code = std::fs::read_to_string("src/equinoxes_pattern.rs").expect("Could not read src/equinoxes_pattern.rs");
    for gregorian_year in 1583..=2999 {
        let republican_year0 = gregorian_year - 1792;
        let republican_year = match republican_year0 >= 0 {
            true => republican_year0 + 1,
            false => republican_year0
        };
        let ts = equinoxes.get(&republican_year).unwrap_or_else(|| panic!("Could not find equinox for year {republican_year} (gregorian {gregorian_year})"));
        code = code.replace("EQUINOX,", format!("{ts}, EQUINOX,").as_str());
    }
    code = code.replace("EQUINOX,", "");
    std::fs::write(format!("{out_dir}/equinoxes.rs"), code).expect("Could not write src/equinoxes.rs");

    // Generate years.rs
    let mut code = std::fs::read_to_string("src/years_pattern.rs").expect("Could not read src/years_pattern.rs");
    for gregorian_year in 1583..=2999 {
        let republican_year0 = gregorian_year - 1792;
        let republican_year = match republican_year0 >= 0 {
            true => republican_year0 + 1,
            false => republican_year0
        };
        let ts = equinoxes.get(&republican_year).unwrap_or_else(|| panic!("Could not find equinox for year {republican_year} (gregorian {gregorian_year})"));
        let day_start = ts - ts.rem_euclid(REPUBLICAN_SECONDS_PER_DAY);
        year_starts.insert(gregorian_year, day_start);
        code = code.replace("YEAR_START,", format!("{day_start}, YEAR_START,").as_str());
    }
    for gregorian_year in 1583..2999 {
        let year_start = year_starts.get(&gregorian_year).unwrap_or_else(|| panic!("Could not find year (greg {gregorian_year})"));
        let next_year_start = year_starts.get(&(gregorian_year + 1)).unwrap_or_else(|| panic!("Could not find year (greg {gregorian_year}+1)"));
        let day_count = (next_year_start - year_start) / REPUBLICAN_SECONDS_PER_DAY;
        code = code.replace("DAY_COUNT,", format!("{day_count}, DAY_COUNT,").as_str());
    }
    code = code.replace("YEAR_START,", "");
    code = code.replace("DAY_COUNT,", "");
    std::fs::write(format!("{out_dir}/years.rs"), code).expect("Could not write src/years.rs");
}