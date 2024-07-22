use std::ops::{Add, AddAssign};
use crate::*;

// DateTime to chrono stuff

impl TryFrom<DateTime> for chrono::DateTime<chrono::Utc> {
    type Error = ();

    fn try_from(value: DateTime) -> Result<Self, Self::Error> {
        match Self::from_timestamp(value.timestamp().to_unix(), 0) {
            Some(dt) => Ok(dt),
            None => Err(()),
        }
    }
}

impl TryFrom<DateTime> for chrono::NaiveDateTime {
    type Error = ();

    fn try_from(value: DateTime) -> Result<Self, Self::Error> {
        let ts = value.timestamp().to_unix();
        match chrono::DateTime::from_timestamp(ts, 0) {
            Some(dt) => Ok(dt.naive_utc()),
            None => Err(()),
        }
    }
}

impl TryFrom<DateTime> for chrono::NaiveDate {
    type Error = ();

    fn try_from(value: DateTime) -> Result<Self, Self::Error> {
        let naive_dt: chrono::NaiveDateTime = value.try_into()?;
        Ok(naive_dt.date())
    }
}

// Chrono stuff to DateTime

impl<Tz: chrono::TimeZone> From<chrono::DateTime<Tz>> for DateTime {
    fn from(value: chrono::DateTime<Tz>) -> Self {
        let ts = value.timestamp();
        let ts = Timestamp::from_unix(ts);
        Self::from_timestamp(ts)
    }
}

impl TryFrom<chrono::NaiveDateTime> for DateTime {
    type Error = ();

    fn try_from(value: chrono::NaiveDateTime) -> Result<Self, Self::Error> {
        let ts = value.and_utc().timestamp();
        let ts = Timestamp::from_unix(ts);
        Ok(Self::from_timestamp(ts))
    }
}

impl TryFrom<chrono::NaiveDate> for DateTime {
    type Error = ();

    fn try_from(value: chrono::NaiveDate) -> Result<Self, Self::Error> {
        let naive_dt = value.and_hms_opt(0, 0, 0).ok_or(())?;
        let ts = naive_dt.and_utc().timestamp();
        let ts = Timestamp::from_unix(ts);
        Ok(Self::from_timestamp(ts))
    }
}

impl Add<chrono::Duration> for DateTime {
    type Output = Self;

    fn add(self, rhs: chrono::Duration) -> Self::Output {
        let seconds = self.timestamp().seconds + rhs.num_seconds() * REPUBLICAN_SECONDS_PER_DAY / GREGORIAN_SECONDS_PER_DAY;
        Self::from_timestamp(Timestamp { seconds })
    }
}

impl AddAssign<chrono::Duration> for DateTime {
    fn add_assign(&mut self, rhs: chrono::Duration) {
        let ts = self.timestamp().seconds + rhs.num_seconds() * REPUBLICAN_SECONDS_PER_DAY / GREGORIAN_SECONDS_PER_DAY;
        *self = Self::from_timestamp(Timestamp { seconds: ts });
    }
}

// Date to chrono stuff

impl TryFrom<Date> for chrono::DateTime<chrono::Utc> {
    type Error = ();

    fn try_from(value: Date) -> Result<Self, Self::Error> {
        match Self::from_timestamp(value.timestamp().to_unix(), 0) {
            Some(dt) => Ok(dt),
            None => Err(()),
        }
    }
}

impl TryFrom<Date> for chrono::NaiveDateTime {
    type Error = ();

    fn try_from(value: Date) -> Result<Self, Self::Error> {
        let ts = value.timestamp().to_unix();
        match chrono::DateTime::from_timestamp(ts, 0) {
            Some(dt) => Ok(dt.naive_utc()),
            None => Err(()),
        }
    }
}

impl TryFrom<Date> for chrono::NaiveDate {
    type Error = ();

    fn try_from(value: Date) -> Result<Self, Self::Error> {
        let naive_dt: chrono::NaiveDateTime = value.try_into()?;
        Ok(naive_dt.date())
    }
}

// Chrono stuff to Date

impl<Tz: chrono::TimeZone> From<chrono::DateTime<Tz>> for Date {
    fn from(value: chrono::DateTime<Tz>) -> Self {
        let ts = value.timestamp();
        let ts = Timestamp::from_unix(ts);
        Self::from_timestamp(ts)
    }
}

impl TryFrom<chrono::NaiveDateTime> for Date {
    type Error = ();

    fn try_from(value: chrono::NaiveDateTime) -> Result<Self, Self::Error> {
        let ts = value.and_utc().timestamp();
        let ts = Timestamp::from_unix(ts);
        Ok(Self::from_timestamp(ts))
    }
}

impl TryFrom<chrono::NaiveDate> for Date {
    type Error = ();

    fn try_from(value: chrono::NaiveDate) -> Result<Self, Self::Error> {
        let naive_dt = value.and_hms_opt(0, 0, 0).ok_or(())?;
        let ts = naive_dt.and_utc().timestamp();
        let ts = Timestamp::from_unix(ts);
        Ok(Self::from_timestamp(ts))
    }
}
