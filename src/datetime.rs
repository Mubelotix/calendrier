use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime {
    year0: i64,
    month0: i64,
    day0: i64,
    hour: i64,
    minute: i64,
    second: i64,
}

impl DateTime {
    pub fn from_timestamp(timestamp: Timestamp) -> Self {
        let year0 = ts_to_year0(timestamp.seconds);
        let seconds_in_year = timestamp.seconds - get_year_start0(year0);

        let month0 = seconds_in_year.div_euclid(SECONDS_PER_MONTH);
        let seconds_in_month = seconds_in_year.rem_euclid(SECONDS_PER_MONTH);

        let day0 = seconds_in_month.div_euclid(SECONDS_PER_DAY);
        let seconds_in_day = seconds_in_month.rem_euclid(SECONDS_PER_DAY);

        let hour = seconds_in_day.div_euclid(10000);
        let seconds_in_hour = seconds_in_day.rem_euclid(10000);

        let minute = seconds_in_hour.div_euclid(100);
        let second = seconds_in_hour.rem_euclid(100);

        Self {
            year0,
            month0,
            day0,
            hour,
            minute,
            second,
        }
    }

    /// # Panics
    /// 
    /// Panics if:
    /// - year is 0,
    /// - month is not in [1, 13],
    /// - day is not in [1, 30],
    /// - hour is not in [0, 9],
    /// - minute is not in [0, 99],
    /// - second is not in [0, 99].
    pub fn from_ymd_hms(year: i64, month: i64, day: i64, hour: i64, minute: i64, second: i64) -> Self {
        let year0 = match year.cmp(&0) {
            std::cmp::Ordering::Greater => year - 1,
            std::cmp::Ordering::Less => year,
            std::cmp::Ordering::Equal => panic!("year cannot be 0"),
        };
        assert!((1..=13).contains(&month), "month must be in [1, 13]");
        assert!((1..=30).contains(&day), "day must be in [1, 30]");
        assert!((0..=9).contains(&hour), "hour must be in [0, 9]");
        assert!((0..=99).contains(&minute), "minute must be in [0, 99]");
        assert!((0..=99).contains(&second), "second must be in [0, 99]");
        let month0 = month - 1;
        let day0 = day - 1;
        Self {
            year0,
            month0,
            day0,
            hour,
            minute,
            second,
        }
    }

    /// # Panics
    /// 
    /// Panics if:
    /// - month is not in [0, 12],
    /// - day is not in [0, 29],
    /// - hour is not in [0, 9],
    /// - minute is not in [0, 99],
    /// - second is not in [0, 99].
    pub fn from_ymd_hms0(year0: i64, month0: i64, day0: i64, hour: i64, minute: i64, second: i64) -> Self {
        assert!((0..=12).contains(&month0), "month must be in [0, 12]");
        assert!((0..=29).contains(&day0), "day must be in [0, 29]");
        assert!((0..=9).contains(&hour), "hour must be in [0, 9]");
        assert!((0..=99).contains(&minute), "minute must be in [0, 99]");
        assert!((0..=99).contains(&second), "second must be in [0, 99]");
        Self {
            year0,
            month0,
            day0,
            hour,
            minute,
            second,
        }
    }

    /// # Panics
    /// 
    /// Panics if:
    /// - year is 0,
    /// - month is not in [1, 13],
    /// - day is not in [1, 30].
    pub fn from_ymd(year: i64, month: i64, day: i64) -> Self {
        Self::from_ymd_hms(year, month, day, 0, 0, 0)
    }

    /// # Panics
    /// 
    /// Panics if:
    /// - month is not in [0, 12],
    /// - day is not in [0, 29].
    pub fn from_ymd0(year0: i64, month0: i64, day0: i64) -> Self {
        Self::from_ymd_hms0(year0, month0, day0, 0, 0, 0)
    }

    /// Returns the franciade number starting from 0.
    /// 
    /// A franciade is defined as 4 years, the first franciade ending in year 3.
    /// It is *not* defined as a period of years ending with a sextile year.
    /// Not all franciades are `365*4+1` days long.
    pub fn franciade0(&self) -> i64 {
        if self.year0 >= 0 {
            (self.year0 + 1) / 4
        } else {
            (self.year0 - 2) / 4
        }
    }

    /// Returns the franciade number starting from 1.
    /// 
    /// A franciade is defined as 4 years, the first franciade ending in year 3.
    /// It is *not* defined as a period of years ending with a sextile year.
    /// Not all franciades are `365*4+1` days long.
    pub fn franciade(&self) -> i64 {
        let franciade0 = self.franciade0();
        match franciade0 >= 0 {
            true => franciade0 + 1,
            false => franciade0,
        }
    }

    /// Returns the year but starting from 0.
    pub fn year0(&self) -> i64 {
        self.year0
    }

    /// Returns the year but starting from 1.
    pub fn year(&self) -> i64 {
        match self.year0 >= 0 {
            true => self.year0 + 1,
            false => self.year0,
        }
    }

    /// Returns the month but starting from 0.
    /// A 13th month of 5 or 6 days is added at the end of the year.
    pub fn num_month0(&self) -> i64 {
        self.month0
    }

    /// Returns the month, starting from 1.
    /// A 13th month of 5 or 6 days is added at the end of the year.
    pub fn num_month(&self) -> i64 {
        self.num_month0() + 1
    }

    /// Returns the month.
    pub fn month(&self) -> Month {
        Month::from_num0(self.month0)
    }

    /// Returns the day of the month but starting from 0.
    pub fn day0(&self) -> i64 {
        self.day0
    }

    /// Returns the day of the month, starting from 1.
    pub fn day(&self) -> i64 {
        self.day0() + 1
    }

    /// Returns the decade but starting from 0.
    pub fn decade0(&self) -> i64 {
        self.day0().div_euclid(DAYS_PER_DECADE)
    }

    /// Returns the decade, starting from 1.
    pub fn decade(&self) -> i64 {
        self.decade0() + 1
    }

    /// Returns the day of the decade, starting from 0.
    pub fn num_decade_day0(&self) -> i64 {
        self.day0().rem_euclid(DAYS_PER_DECADE)
    }

    /// Returns the day of the decade, starting from 1.
    pub fn num_decade_day(&self) -> i64 {
        self.num_decade_day0() + 1
    }

    pub fn decade_day(&self) -> Day {
        if self.month0 == 12 {
            Day::Sansculottide(SansculottideDay::from_num0(self.num_decade_day0()))
        } else {
            Day::Regular(RegularDay::from_num0(self.num_decade_day0()))
        }
    }

    pub fn hour(&self) -> i64 {
        self.hour
    }

    pub fn minute(&self) -> i64 {
        self.minute
    }

    pub fn second(&self) -> i64 {
        self.second
    }

    pub fn hms(&self) -> (i64, i64, i64) {
        (self.hour, self.minute, self.second)
    }

    /// Returns the timestamp
    pub fn timestamp(&self) -> Timestamp {
        Timestamp {
            seconds: get_year_start0(self.year0) + self.month0 * SECONDS_PER_MONTH + self.day0 * SECONDS_PER_DAY + self.hour * 10000 + self.minute * 100 + self.second,
        }
    }

    fn fmt_default(&self, f: &mut impl std::io::Write) -> std::io::Result<()> {
        write!(
            f,
            "{} {} {} {}",
            self.decade_day(),
            self.day(),
            self.month(),
            self.year(),
        )
    }

    fn fmt_traditional(&self, f: &mut impl std::io::Write) -> std::io::Result<()> {
        let mut remaining_years = self.year();
        let thousand_years = remaining_years.div_euclid(1000);
        remaining_years -= thousand_years * 1000;
        let thousand_years = "M".repeat(thousand_years as usize);
        let five_hundred_years = remaining_years.div_euclid(500);
        remaining_years -= five_hundred_years * 500;
        let five_hundred_years = match five_hundred_years {
            0 => "",
            1 => "D",
            _ => unreachable!(),
        };
        let hundred_years = remaining_years.div_euclid(100);
        remaining_years -= hundred_years * 100;
        let hundred_years = match hundred_years {
            0 => "",
            1 => "C",
            2 => "CC",
            3 => "CCC",
            4 => "CD",
            _ => unreachable!(),
        };
        let fifty_years = remaining_years.div_euclid(50);
        remaining_years -= fifty_years * 50;
        let fifty_years = match fifty_years {
            0 => "",
            1 => "L",
            _ => unreachable!(),
        };
        let ten_years = remaining_years.div_euclid(10);
        remaining_years -= ten_years * 10;
        let ten_years = match ten_years {
            0 => "",
            1 => "X",
            2 => "XX",
            3 => "XXX",
            4 => "XL",
            _ => unreachable!(),
        };
        let five_years = remaining_years.div_euclid(5);
        remaining_years -= five_years * 5;
        let five_years = match five_years {
            0 => "",
            1 => "V",
            _ => unreachable!(),
        };
        let one_year = remaining_years;
        let one_year = match one_year {
            0 => "",
            1 => "I",
            2 => "II",
            3 => "III",
            4 => "IV",
            _ => unreachable!(),
        };

        write!(
            f,
            "{} {} {} an {}{}{}{}{}{}{}",
            self.decade_day(),
            self.day(),
            self.month(),
            thousand_years, five_hundred_years, hundred_years, fifty_years, ten_years, five_years, one_year
        )
    }

    pub fn to_string_default(&self) -> String {
        let mut s = Vec::new();
        self.fmt_default(&mut s).unwrap();
        String::from_utf8(s).unwrap()
    }

    pub fn to_string_traditional(&self) -> String {
        let mut s = Vec::new();
        self.fmt_traditional(&mut s).unwrap();
        String::from_utf8(s).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SECONDS_PER_YEAR: i64 = 365*SECONDS_PER_DAY;

    #[test]
    fn test_franciade() {
        let datetime = DateTime::from_timestamp(Timestamp { seconds: 0 });
        assert_eq!(datetime.franciade0(), 0);
        assert_eq!(datetime.franciade(), 1);
        
        let datetime = DateTime::from_timestamp(Timestamp { seconds: -400*SECONDS_PER_DAY });
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
        assert_eq!(datetime.num_month0(), 0);
        assert_eq!(datetime.month(), Month::Vendémiaire);
        
        let datetime = DateTime::from_timestamp(Timestamp { seconds: -1 });
        assert_eq!(datetime.num_month0(), 12);
        assert_eq!(datetime.month(), Month::Sansculotides);
    }

    #[test]
    fn test_day() {
        let datetime = DateTime::from_timestamp(Timestamp { seconds: 0 });
        assert_eq!(datetime.day0(), 0);
        assert_eq!(datetime.day(), 1);
        
        let datetime = DateTime::from_timestamp(Timestamp { seconds: get_year_start(4)-1 });
        assert_eq!(datetime.day0(), 5); // Jour de la révolution
        assert_eq!(datetime.decade_day().name(), "Jour de la Révolution");
        assert_eq!(datetime.day(), 6);
    }

    #[test]
    fn test_fmt() {
        let datetime = DateTime::from_timestamp(Timestamp { seconds: 0 });
        assert_eq!(datetime.to_string_default(), "Primidi 1 Vendémiaire 1");
        assert_eq!(datetime.to_string_traditional(), "Primidi 1 Vendémiaire an I");
    }
}
