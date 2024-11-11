use crate::{generated::equinoxes::TIMESTAMPS, REPUBLICAN_SECONDS_PER_DAY};

fn try_year_starts0(republican_year0: i64) -> Option<i64> {
    let index = republican_year0 + 209;
    if index < 0 || index as usize >= TIMESTAMPS.len() {
        None
    } else {
        let ts = TIMESTAMPS[index as usize];
        Some(ts - ts.rem_euclid(REPUBLICAN_SECONDS_PER_DAY))
    }
}

pub fn get_year_start(republican_year: i64) -> i64 {
    get_year_start0(republican_year0(republican_year))
}

fn republican_year0(republican_year: i64) -> i64 {
    if republican_year > 0 {
        republican_year - 1
    } else {
        republican_year
    }
}

pub fn get_year_start0(republican_year0: i64) -> i64 {
    try_year_starts0(republican_year0).unwrap_or_else(|| {
        if republican_year0 >= 0 {
            let sextile_years_since_1208 = (republican_year0 + 1 - 1208) / 4;
            let standard_years_since_1208 = republican_year0 + 1 - 1208 - sextile_years_since_1208;
            let days_since_1208 = sextile_years_since_1208 * 366 + standard_years_since_1208 * 365;
            get_year_start(1208) + days_since_1208 * REPUBLICAN_SECONDS_PER_DAY
        } else {
            let sextile_years_since_m210 = -(republican_year0 + 210) / 4;
            let standard_years_since_m210 = -(republican_year0 + 210) - sextile_years_since_m210;
            let days_since_m210 = sextile_years_since_m210 * 366 + standard_years_since_m210 * 365;
            get_year_start(-209) - (days_since_m210 + 366) * REPUBLICAN_SECONDS_PER_DAY
        }
    })
}

pub fn get_day_count(republican_year: i64) -> i64 {
    get_day_count0(republican_year0(republican_year))
}

fn get_day_count0(republican_year0: i64) -> i64 {
    let year_start = get_year_start0(republican_year0);
    let next_year_start = get_year_start0(republican_year0 + 1);
    (next_year_start - year_start) / REPUBLICAN_SECONDS_PER_DAY
}

const AVERAGE_SECONDS_PER_YEAR: i64 =
    (4 * 365 * REPUBLICAN_SECONDS_PER_DAY + REPUBLICAN_SECONDS_PER_DAY) / 4;

pub fn ts_to_year0(ts: i64) -> i64 {
    let estimated = ts / AVERAGE_SECONDS_PER_YEAR;
    if estimated.abs() > 100_000 {
        return estimated;
    }

    let mut year0 = estimated;
    loop {
        let year_start = get_year_start0(year0);
        if year_start > ts {
            // If the predicted year starts after the timestamp
            year0 -= 1;
            continue;
        }
        if year_start + get_day_count0(year0) * REPUBLICAN_SECONDS_PER_DAY <= ts {
            // If the predicted year ends before the timestamp
            year0 += 1;
            continue;
        }
        return year0;
    }
}

pub fn ts_to_year(ts: i64) -> i64 {
    let year0 = ts_to_year0(ts);
    if year0 >= 0 {
        year0 + 1
    } else {
        year0
    }
}

#[test]
fn test_year_start() {
    assert_eq!(0, get_year_start(1));
    assert_eq!(365, get_day_count(1));
    assert_eq!(365, get_day_count(2));
    assert_eq!(366, get_day_count(3)); // sextile
    assert_eq!(365, get_day_count(4));
    assert_eq!(365, get_day_count(5));
    assert_eq!(365, get_day_count(6));
    assert_eq!(366, get_day_count(7)); // sextile
    assert_eq!(365, get_day_count(8));
    assert_eq!(365, get_day_count(9));
    assert_eq!(365, get_day_count(10));
    assert_eq!(366, get_day_count(11)); // sextile

    assert_eq!(365, get_day_count(1208));
    assert_eq!(365, get_day_count(1209));
    assert_eq!(365, get_day_count(1210));
    assert_eq!(366, get_day_count(1211)); // sextile

    assert_eq!(365, get_day_count(-211));
    assert_eq!(365, get_day_count(-212));
    assert_eq!(365, get_day_count(-213));
    assert_eq!(366, get_day_count(-214)); // sextile

    // Verify coherence
    let mut previous_year_start = get_year_start(1);
    for year in 2..=10000 {
        let year_start = get_year_start(year);
        assert_eq!(
            year_start,
            previous_year_start + get_day_count(year - 1) * REPUBLICAN_SECONDS_PER_DAY,
            "year {}",
            year
        );
        previous_year_start = year_start;
    }
    let mut next_year_start = get_year_start(-1);
    for year in (-10000..=-2).rev() {
        let year_start = get_year_start(year);
        assert_eq!(
            year_start,
            next_year_start - get_day_count(year) * REPUBLICAN_SECONDS_PER_DAY,
            "year {}",
            year
        );
        next_year_start = year_start;
    }

    // ts to year
    assert_eq!(1, ts_to_year(get_year_start(1)));
    assert_eq!(-1, ts_to_year(-1));
    assert_eq!(1, ts_to_year(1));
    for year0 in -10000..=10000 {
        assert_eq!(year0, ts_to_year0(get_year_start0(year0)));
    }
}
