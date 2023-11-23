use crate::*;

/// Sources:
/// https://www.imcce.fr/newsletter/docs/Equinoxe_automne_1583_2999.pdf
/// https://fr.wikisource.org/wiki/D%C3%A9cret_de_la_Convention_nationale_portant_sur_la_cr%C3%A9ation_du_calendrier_r%C3%A9publicain
/// 09h 00m 30s UT1 = 09h 18m 30s Gregorian Time With Old Paris Time Zone
/// There is a 18 minutes offset between the times.
pub(crate) const REPUBLICAN_EPOCH_GREGORIAN_SECONDS: i64 = -5594227200;
pub(crate) const OFFSET_GREGORIAN_SECONDS: i64 = 18*60;
pub(crate) const REPUBLICAN_SECONDS_PER_DAY: i64 = 100000;
pub(crate) const GREGORIAN_SECONDS_PER_DAY: i64 = 86400;

/// It is necessary to have a different timestamp than UNIX systems as seconds are different in the Republican Calendar.
/// Indeed, there are 86400 seconds in a day in the Gregorian Calendar, but 100000 seconds in a day in the Republican Calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp {
    /// The number of seconds since the beginning of the Republican Calendar.
    pub seconds: i64,
}

impl Timestamp {
    pub fn from_unix(unix_timestamp: i64) -> Self {
        let gregorian_seconds = unix_timestamp - REPUBLICAN_EPOCH_GREGORIAN_SECONDS + OFFSET_GREGORIAN_SECONDS;
        let republican_seconds = gregorian_seconds * REPUBLICAN_SECONDS_PER_DAY / GREGORIAN_SECONDS_PER_DAY;
        Self {
            seconds: republican_seconds,
        }
    }

    pub fn to_unix(&self) -> i64 {
        let gregorian_seconds = self.seconds * GREGORIAN_SECONDS_PER_DAY / REPUBLICAN_SECONDS_PER_DAY;
        let unix_timestamp = gregorian_seconds + REPUBLICAN_EPOCH_GREGORIAN_SECONDS - OFFSET_GREGORIAN_SECONDS;
        unix_timestamp
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use super::*;

    #[test]
    fn test_time_offset() {
        let date1 = chrono::Utc.with_ymd_and_hms(1792, 9, 22, 0, 0, 0).single().unwrap();
        let ts1 = Timestamp::from_unix(date1.timestamp());
        assert_eq!(ts1.seconds, 1250);

        let date2 = DateTime::from_ymd(1, 1, 1) + chrono::Duration::minutes(18);
        assert_eq!(date2.timestamp().seconds, 1250);

        // Makes sure that 09:00:30 UT1 = 09:18:30 Gregorian Time With Old Paris Time Zone
        let date3 = chrono::Utc.with_ymd_and_hms(1792, 9, 22, 9, 0, 30).single().unwrap();
        let ts3 = Timestamp::from_unix(date3.timestamp());
        let date4 = DateTime::from_ymd(1, 1, 1) + chrono::Duration::hours(9) + chrono::Duration::minutes(18) + chrono::Duration::seconds(30);
        let ts4 = date4.timestamp();
        assert_eq!(ts3, ts4);
    }
}
