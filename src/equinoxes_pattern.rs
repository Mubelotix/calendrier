
/// Starts at gregorian year 1583, republican year -209
const EQUINOXES: &[i64] = &[
    EQUINOX,
];

pub const fn get_equinox(republican_year: i64) -> i64 {
    let republican_year0 = if republican_year > 0 {
        republican_year - 1
    } else {
        republican_year
    };

    let index = republican_year0 + 209;
    EQUINOXES[index as usize]
}

#[test]
fn test_equinoxes() {
    assert!((0..100_000).contains(&get_equinox(1)));
}
