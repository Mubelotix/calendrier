/// Sources:
/// https://www.imcce.fr/newsletter/docs/Equinoxe_automne_1583_2999.pdf
/// https://fr.wikisource.org/wiki/D%C3%A9cret_de_la_Convention_nationale_portant_sur_la_cr%C3%A9ation_du_calendrier_r%C3%A9publicain
///
/// From article I, we know that:
/// 09:00:30 today = 09:18:30 then
/// Meaning there is a 1080 gregorian seconds offset between the times.
///
/// From article V, we know that:
/// 14:55:19 today = 15:11:38 then
/// Meaning there is a 979 gregorian seconds offset between the times.
///
/// The first offset was chosen for this library.
pub(crate) const REPUBLICAN_EPOCH_GREGORIAN_SECONDS: i64 = -5594227200;
#[cfg(not(any(feature = "no-time-offset", feature = "average-time-offset")))]
pub(crate) const OFFSET_GREGORIAN_SECONDS: i64 = 1080;
#[cfg(feature = "no-time-offset")]
pub(crate) const OFFSET_GREGORIAN_SECONDS: i64 = 0;
#[cfg(feature = "average-time-offset")]
pub(crate) const OFFSET_GREGORIAN_SECONDS: i64 = 1029;
pub(crate) const REPUBLICAN_SECONDS_PER_DAY: i64 = 100000;
pub(crate) const GREGORIAN_SECONDS_PER_DAY: i64 = 86400;

#[cfg(all(feature = "no-time-offset", feature = "average-time-offset"))]
compile_error!(
    "You cannot enable both no-time-offset and average-time-offset features at the same time"
);

/// It is necessary to have a different timestamp than UNIX systems as seconds are different in the Republican Calendar.
/// Indeed, there are 86400 seconds in a day in the Gregorian Calendar, but 100000 seconds in a day in the Republican Calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp {
    /// The number of seconds since the beginning of the Republican Calendar.
    pub seconds: i64,
}

impl Timestamp {
    pub fn from_unix(unix_timestamp: i64) -> Self {
        let gregorian_seconds =
            unix_timestamp - REPUBLICAN_EPOCH_GREGORIAN_SECONDS + OFFSET_GREGORIAN_SECONDS;
        let republican_seconds =
            gregorian_seconds * REPUBLICAN_SECONDS_PER_DAY / GREGORIAN_SECONDS_PER_DAY;
        Self {
            seconds: republican_seconds,
        }
    }

    pub fn to_unix(&self) -> i64 {
        let gregorian_seconds =
            self.seconds * GREGORIAN_SECONDS_PER_DAY / REPUBLICAN_SECONDS_PER_DAY;
        gregorian_seconds + REPUBLICAN_EPOCH_GREGORIAN_SECONDS - OFFSET_GREGORIAN_SECONDS
    }
}

#[cfg(test)]
mod tests {
    use crate::DateTime;

    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_time_offset() {
        let date1 = chrono::Utc
            .with_ymd_and_hms(1792, 9, 22, 0, 0, 0)
            .single()
            .unwrap();
        let ts1 = Timestamp::from_unix(date1.timestamp());
        assert_eq!(ts1.seconds, 1250);

        let date2 = DateTime::from_ymd(1, 1, 1) + chrono::Duration::minutes(18);
        assert_eq!(date2.timestamp().seconds, 1250);

        // Makes sure that 09:00:30 UT1 = 09:18:30 Gregorian Time With Old Paris Time Zone
        let date3 = chrono::Utc
            .with_ymd_and_hms(1792, 9, 22, 9, 0, 30)
            .single()
            .unwrap();
        let ts3 = Timestamp::from_unix(date3.timestamp());
        let date4 = DateTime::from_ymd(1, 1, 1)
            + chrono::Duration::hours(9)
            + chrono::Duration::minutes(18)
            + chrono::Duration::seconds(30);
        let ts4 = date4.timestamp();
        assert_eq!(ts3, ts4);
    }

    #[test]
    fn hour_converter() {
        let hours = vec![
            (8, 0, 0),
            (9, 45, 0),
            (11, 30, 0),
            (13, 15, 0),
            (15, 0, 0),
            (16, 45, 0),
            (18, 30, 0),
        ];
        for hour in hours {
            let date: chrono::NaiveDateTime = chrono::NaiveDate::from_ymd_opt(1, 1, 1)
                .expect("Incorrect date")
                .and_hms_opt(hour.0, hour.1, hour.2)
                .expect("Incorrect time");
            let republican: DateTime = date.try_into().unwrap();
            println!(
                "{}h{} {}s -> {}h{} {}s",
                hour.0,
                hour.1,
                hour.2,
                republican.hour(),
                republican.minute(),
                republican.second()
            );
        }
    }
}
