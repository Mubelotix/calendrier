use crate::*;

impl<Tz: chrono::TimeZone> From<chrono::DateTime<Tz>> for DateTime {
    fn from(value: chrono::DateTime<Tz>) -> Self {
        let ts = value.timestamp();
        let ts = Timestamp::from_unix(ts);
        Self::from_timestamp(ts)
    }
}

impl TryFrom<DateTime> for chrono::DateTime<chrono::Utc> {
    type Error = ();

    fn try_from(value: DateTime) -> Result<Self, Self::Error> {
        let ts = value.timestamp;
        let ts = ts.to_unix();
        match Self::from_timestamp(ts, 0) {
            Some(dt) => Ok(dt),
            None => Err(()),
        }
    }
}

impl From<chrono::NaiveDate> for DateTime {
    fn from(value: chrono::NaiveDate) -> Self {
        
    }
}
