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
        match self.franciade0() >= 0 {
            true => self.franciade0() + 1,
            false => self.franciade0(),
        }
    }

    /// Returns the year but starting from 0.
    pub fn year0(&self) -> i64 {
        let franciade0 = self.franciade0();
        let seconds_in_franciade = self.timestamp.seconds - franciade0 * SECONDS_PER_FRANCIADE;
        let years_in_franciade = seconds_in_franciade / (SECONDS_PER_DAY * 365);
        if years_in_franciade > 4 {
            4
        } else {
            years_in_franciade
        }
    }

    /// Returns the year but starting from 1.
    pub fn year(&self) -> i64 {
        match self.year0() >= 0 {
            true => self.year0() + 1,
            false => self.year0() - 1,
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
    }
}
