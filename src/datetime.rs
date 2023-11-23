use crate::*;

const SECONDS_PER_DAY: i64 = 100000;
const SECONDS_PER_YEAR: i64 = 365*SECONDS_PER_DAY;
const SECONDS_PER_FRANCIADE: i64 = SECONDS_PER_YEAR*4 + SECONDS_PER_DAY;

pub struct DateTime {
    timestamp: Timestamp,
}

impl DateTime {
    pub fn from_timestamp(timestamp: Timestamp) -> Self {
        Self {
            timestamp,
        }
    }

    /// Returns the franciade but starting from 0.
    /// A franciade is a period of 4 years.
    pub fn franciade0(&self) -> i64 {
        self.timestamp.seconds.div_euclid(SECONDS_PER_FRANCIADE)
    }

    /// Returns the franciade but starting from 1.
    /// There is no franciade 0.
    pub fn franciade(&self) -> i64 {
        let franciade0 = self.franciade0();
        match franciade0 >= 0 {
            true => franciade0 + 1,
            false => franciade0,
        }
    }

    /// Returns the year but starting from 0.
    pub fn year0(&self) -> i64 {
        let franciade0 = self.franciade0();
        let seconds_in_franciade = self.timestamp.seconds - franciade0 * SECONDS_PER_FRANCIADE;
        let years_in_franciade = seconds_in_franciade.div_euclid(SECONDS_PER_DAY * 365).clamp(-3, 3);
        franciade0 * 4 + years_in_franciade
    }

    /// Returns the year but starting from 1.
    pub fn year(&self) -> i64 {
        let year0 = self.year0();
        match year0 >= 0 {
            true => year0 + 1,
            false => year0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_franciade() {
        let datetime = DateTime::from_timestamp(Timestamp { seconds: 0 });
        assert_eq!(datetime.franciade0(), 0);
        assert_eq!(datetime.franciade(), 1);

        let datetime = DateTime::from_timestamp(Timestamp { seconds: -1 });
        assert_eq!(datetime.franciade0(), -1);
        assert_eq!(datetime.franciade(), -1);

        let datetime = DateTime::from_timestamp(Timestamp { seconds: SECONDS_PER_YEAR*5 });
        assert_eq!(datetime.franciade0(), 1);
        assert_eq!(datetime.franciade(), 2);
    }

    #[test]
    fn test_year() {
        let datetime = DateTime::from_timestamp(Timestamp { seconds: 0 });
        assert_eq!(datetime.year0(), 0);
        assert_eq!(datetime.year(), 1);
        
        let datetime = DateTime::from_timestamp(Timestamp { seconds: -1 });
        assert_eq!(datetime.year0(), -1);
        assert_eq!(datetime.year(), -1);

        let datetime = DateTime::from_timestamp(Timestamp { seconds: -SECONDS_PER_YEAR-SECONDS_PER_DAY-1 });
        assert_eq!(datetime.year0(), -2);
        assert_eq!(datetime.year(), -2);
    }
}
