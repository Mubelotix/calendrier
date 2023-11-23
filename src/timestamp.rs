use crate::*;

/// Sources:
/// https://www.imcce.fr/newsletter/docs/Equinoxe_automne_1583_2999.pdf
/// https://fr.wikisource.org/wiki/D%C3%A9cret_de_la_Convention_nationale_portant_sur_la_cr%C3%A9ation_du_calendrier_r%C3%A9publicain
/// 09h 00m 30s UT1 = 09h 18m 30s Gregorian Time With Old Paris Time Zone
/// -5594194770 <=> 33510
/// -5594228280 <=> 0
const REPUBLICAN_EPOCH_GREGORIAN_SECONDS: i64 = -5594228280;
const REPUBLICAN_SECONDS_PER_DAY: i64 = 100000;
const GREGORIAN_SECONDS_PER_DAY: i64 = 86400;

/// It is necessary to have a different timestamp than UNIX systems as seconds are different in the Republican Calendar.
/// Indeed, there are 86400 seconds in a day in the Gregorian Calendar, but 100000 seconds in a day in the Republican Calendar.
pub struct Timestamp {
    /// The number of seconds since the beginning of the Republican Calendar.
    pub seconds: i64,
}

impl Timestamp {
    pub fn from_unix(unix_timestamp: i64) -> Self {
        let gregorian_seconds = unix_timestamp - REPUBLICAN_EPOCH_GREGORIAN_SECONDS;
        let republican_seconds = gregorian_seconds * REPUBLICAN_SECONDS_PER_DAY / GREGORIAN_SECONDS_PER_DAY;
        Self {
            seconds: republican_seconds,
        }
    }
}
