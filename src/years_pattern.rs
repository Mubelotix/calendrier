/// Starts at gregorian year 1583, republican year -209
const YEAR_STARTS: &[i64] = &[
    YEAR_START,
];

/// Starts at gregorian year 1583, republican year -209
const DAY_COUNTS: &[i64] = &[
    DAY_COUNT,
];

pub const fn get_year_start(republican_year: i64) -> i64 {
    let republican_year0 = match republican_year > 0 {
        true => republican_year - 1,
        false => republican_year
    };
    let index = republican_year0 + 209;
    YEAR_STARTS[index as usize] // TODO: check if index is in bounds
}

pub const fn get_day_count(republican_year: i64) -> i64 {
    let republican_year0 = match republican_year > 0 {
        true => republican_year - 1,
        false => republican_year
    };
    let index = republican_year0 + 209;
    DAY_COUNTS[index as usize] // TODO: check if index is in bounds
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
}
