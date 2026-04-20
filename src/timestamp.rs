/// Sources:
/// https://www.imcce.fr/newsletter/docs/Equinoxe_automne_1583_2999.pdf
/// https://fr.wikisource.org/wiki/D%C3%A9cret_de_la_Convention_nationale_portant_sur_la_cr%C3%A9ation_du_calendrier_r%C3%A9publicain
/// https://fr.wikipedia.org/wiki/Heure_en_France#:~:text=9%20minutes%20et%2020%2C921
pub(crate) const REPUBLICAN_EPOCH_GREGORIAN_SECONDS: i64 = -5594227200;
pub(crate) const OFFSET_GREGORIAN_SECONDS: i64 = 9*60+21;
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
    pub fn from_unix(mut unix_timestamp: i64) -> Self {
        #[cfg(feature = "solar")]
        {
            unix_timestamp = crate::get_solar_unix_timestamp(unix_timestamp) as i64;
        }

        let gregorian_seconds =
            unix_timestamp - REPUBLICAN_EPOCH_GREGORIAN_SECONDS + OFFSET_GREGORIAN_SECONDS;
        let republican_seconds =
            gregorian_seconds * REPUBLICAN_SECONDS_PER_DAY / GREGORIAN_SECONDS_PER_DAY;

        Self {
            seconds: republican_seconds,
        }
    }

    pub fn to_unix(&self) -> i64 {
        let mut gregorian_seconds =
            self.seconds * GREGORIAN_SECONDS_PER_DAY / REPUBLICAN_SECONDS_PER_DAY;
        gregorian_seconds += REPUBLICAN_EPOCH_GREGORIAN_SECONDS - OFFSET_GREGORIAN_SECONDS;

        #[cfg(feature = "solar")]
        {
            gregorian_seconds = crate::get_unix_from_solar_timestamp(gregorian_seconds as f64)
        }
        
        gregorian_seconds
    }
}

#[cfg(test)]
mod tests {
    use crate::DateTime;

    use super::*;
    use chrono::TimeZone;

    #[test]
    #[cfg(feature = "solar")]
    fn test_time_offset() {
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
        assert!((ts3.seconds - ts4.seconds).abs() <= 90, "Mismatch should be less than 90s");

        // Makes sure that 14:55:19 UT1 = 15:11:38 Gregorian Time With Old Paris Time Zone
        let date5 = chrono::Utc
            .with_ymd_and_hms(1792, 9, 22, 14, 55, 19)
            .single()
            .unwrap();
        let ts5 = Timestamp::from_unix(date5.timestamp());
        let date6 = DateTime::from_ymd(1, 1, 1)
            + chrono::Duration::hours(15)
            + chrono::Duration::minutes(11)
            + chrono::Duration::seconds(38);
        let ts6 = date6.timestamp();
        assert!((ts5.seconds - ts6.seconds).abs() <= 90, "Mismatch should be less than 90s");

        // Make sure their signs differ (otherwise it would mean we can improve)
        assert!(
            (ts3.seconds - ts4.seconds) * (ts5.seconds - ts6.seconds) < 0,
            "The mismatches should have different signs"
        );
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
