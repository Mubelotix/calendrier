use crate::*;

const SECONDS_PER_DAY: i64 = 100000;
const DAYS_PER_MONTH: i64 = 30;
const DAYS_PER_WEEK: i64 = 10;
const SECONDS_PER_WEEK: i64 = SECONDS_PER_DAY*DAYS_PER_WEEK;
const SECONDS_PER_MONTH: i64 = SECONDS_PER_DAY*DAYS_PER_MONTH;
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

    /// Returns the franciade and the year from 0..=3 in the franciade.
    pub fn franciade0_year0(&self) -> (i64, i64) {
        let franciade0 = self.franciade0();
        let seconds_in_franciade = self.timestamp.seconds - franciade0 * SECONDS_PER_FRANCIADE;
        let year0 = seconds_in_franciade.div_euclid(SECONDS_PER_YEAR).clamp(0, 3);
        (franciade0, year0)
    }

    /// Returns the year but starting from 0.
    pub fn year0(&self) -> i64 {
        let (franciade0, years_in_franciade) = self.franciade0_year0();
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

    fn seconds_in_year(&self) -> i64 {
        let (franciade0, years_in_franciade) = self.franciade0_year0();
        let seconds_in_year = self.timestamp.seconds - franciade0 * SECONDS_PER_FRANCIADE - years_in_franciade * SECONDS_PER_YEAR;
        seconds_in_year
    }

    /// Returns the month but starting from 0.
    /// A 13th month of 5 or 6 days is added at the end of the year.
    pub fn month0(&self) -> i64 {
        let seconds_in_year = self.seconds_in_year();
        let month = seconds_in_year.div_euclid(SECONDS_PER_MONTH);
        month
    }

    /// Returns the month name, starting with a capital letter.
    pub fn month_name(&self) -> &'static str {
        let month0 = self.month0();
        match month0 {
            0 => "Vendémiaire",
            1 => "Brumaire",
            2 => "Frimaire",
            3 => "Nivôse",
            4 => "Pluviôse",
            5 => "Ventôse",
            6 => "Germinal",
            7 => "Floréal",
            8 => "Prairial",
            9 => "Messidor",
            10 => "Thermidor",
            11 => "Fructidor",
            12 => "Sansculotides",
            _ => unreachable!(),
        }
    }

    /// Returns the month name, all lowercase.
    pub fn month_name_lc(&self) -> &'static str {
        let month0 = self.month0();
        match month0 {
            0 => "vendémiaire",
            1 => "brumaire",
            2 => "frimaire",
            3 => "nivôse",
            4 => "pluviôse",
            5 => "ventôse",
            6 => "germinal",
            7 => "floréal",
            8 => "prairial",
            9 => "messidor",
            10 => "thermidor",
            11 => "fructidor",
            12 => "sansculotides",
            _ => unreachable!(),
        }
    }

    /// Returns the month, starting from 1.
    /// A 13th month of 5 or 6 days is added at the end of the year.
    pub fn month(&self) -> i64 {
        self.month0() + 1
    }

    fn seconds_in_month(&self) -> i64 {
        let seconds_in_year = self.seconds_in_year();
        let seconds_in_month = seconds_in_year - self.month0() * SECONDS_PER_MONTH;
        seconds_in_month
    }

    /// Returns the day of the month but starting from 0.
    pub fn day0(&self) -> i64 {
        let seconds_in_month = self.seconds_in_month();
        let day = seconds_in_month.div_euclid(SECONDS_PER_DAY);
        day
    }

    /// Returns the day of the month, starting from 1.
    pub fn day(&self) -> i64 {
        self.day0() + 1
    }

    /// Returns the week but starting from 0.
    pub fn week0(&self) -> i64 {
        let seconds_in_month = self.seconds_in_month();
        let week = seconds_in_month.div_euclid(SECONDS_PER_WEEK);
        week
    }

    /// Returns the week, starting from 1.
    pub fn week(&self) -> i64 {
        self.week0() + 1
    }

    fn seconds_in_week(&self) -> i64 {
        let seconds_in_month = self.seconds_in_month();
        let seconds_in_week = seconds_in_month - self.week0() * SECONDS_PER_WEEK;
        seconds_in_week
    }

    /// Returns the day of the week but starting from 0.
    pub fn weekday0(&self) -> i64 {
        let seconds_in_week = self.seconds_in_week();
        let weekday = seconds_in_week.div_euclid(SECONDS_PER_DAY);
        weekday
    }

    /// Returns the day of the week, starting from 1.
    pub fn weekday(&self) -> i64 {
        self.weekday0() + 1
    }

    /// Returns the day of the week name, starting with a capital letter.
    pub fn weekday_name(&self) -> &'static str {
        if self.month0() == 12 {
            return match self.day0() {
                0 => "Jour de la vertu",
                1 => "Jour du génie",
                2 => "Jour du travail",
                3 => "Jour de l'opinion",
                4 => "Jour des récompenses",
                5 => "Jour de la Révolution",
                _ => unreachable!(),
            }
        }
        let weekday0 = self.weekday0();
        match weekday0 {
            0 => "Primidi",
            1 => "Duodi",
            2 => "Tridi",
            3 => "Quartidi",
            4 => "Quintidi",
            5 => "Sextidi",
            6 => "Septidi",
            7 => "Octidi",
            8 => "Nonidi",
            9 => "Décadi",
            _ => unreachable!(),
        }
    }

    /// Returns the day of the week name, all lowercase (except Révolution)
    pub fn weekday_name_lc(&self) -> &'static str {
        if self.month0() == 12 {
            return match self.day0() {
                0 => "jour de la vertu",
                1 => "jour du génie",
                2 => "jour du travail",
                3 => "jour de l'opinion",
                4 => "jour des récompenses",
                5 => "jour de la Révolution",
                _ => unreachable!(),
            }
        }
        let weekday0 = self.weekday0();
        match weekday0 {
            0 => "primidi",
            1 => "duodi",
            2 => "tridi",
            3 => "quartidi",
            4 => "quintidi",
            5 => "sextidi",
            6 => "septidi",
            7 => "octidi",
            8 => "nonidi",
            9 => "décadi",
            _ => unreachable!(),
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

    #[test]
    fn test_month() {
        let datetime = DateTime::from_timestamp(Timestamp { seconds: 0 });
        assert_eq!(datetime.month0(), 0);
        assert_eq!(datetime.month(), 1);
        
        let datetime = DateTime::from_timestamp(Timestamp { seconds: -1 });
        assert_eq!(datetime.month0(), 12);
        assert_eq!(datetime.month(), 13);
    }

    #[test]
    fn test_day() {
        let datetime = DateTime::from_timestamp(Timestamp { seconds: 0 });
        assert_eq!(datetime.day0(), 0);
        assert_eq!(datetime.day(), 1);
        
        let datetime = DateTime::from_timestamp(Timestamp { seconds: -1 });
        assert_eq!(datetime.day0(), 5); // Jour de la révolution
        assert_eq!(datetime.weekday_name(), "Jour de la Révolution");
        assert_eq!(datetime.day(), 6);
    }
}
