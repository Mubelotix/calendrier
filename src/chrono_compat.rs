use crate::*;

impl<Tz: chrono::TimeZone> From<chrono::DateTime<Tz>> for DateTime {
    fn from(value: chrono::DateTime<Tz>) -> Self {
        let ts = value.timestamp();
        let ts = Timestamp::from_unix(ts);
        Self::from_timestamp(ts)
    }
}

impl<Tz: chrono::TimeZone> From<chrono::DateTime<Tz>> for Date {
    fn from(value: chrono::DateTime<Tz>) -> Self {
        let ts = value.timestamp();
        let ts = Timestamp::from_unix(ts);
        Self::from_timestamp(ts)
    }
}

impl TryFrom<DateTime> for chrono::DateTime<chrono::Utc> {
    type Error = ();

    fn try_from(value: DateTime) -> Result<Self, Self::Error> {
        match Self::from_timestamp(value.timestamp().to_unix(), 0) {
            Some(dt) => Ok(dt),
            None => Err(()),
        }
    }
}

impl TryFrom<Date> for chrono::DateTime<chrono::Utc> {
    type Error = ();

    fn try_from(value: Date) -> Result<Self, Self::Error> {
        match Self::from_timestamp(value.timestamp().to_unix(), 0) {
            Some(dt) => Ok(dt),
            None => Err(()),
        }
    }
}

impl TryFrom<chrono::NaiveDate> for DateTime {
    type Error = ();

    fn try_from(value: chrono::NaiveDate) -> Result<Self, Self::Error> {
        let ts = match value.and_hms_opt(0, 0, 0) {
            Some(dt) => dt.timestamp(),
            None => return Err(()),
        };
        Ok(Self::from_timestamp(Timestamp::from_unix(ts)))
    }
}

impl TryFrom<chrono::NaiveDate> for Date {
    type Error = ();

    fn try_from(value: chrono::NaiveDate) -> Result<Self, Self::Error> {
        let ts = match value.and_hms_opt(0, 0, 0) {
            Some(dt) => dt.timestamp(),
            None => return Err(()),
        };
        Ok(Self::from_timestamp(Timestamp::from_unix(ts)))
    }
}
