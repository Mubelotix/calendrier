use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    year0: i64,
    month0: i64,
    day0: i64,
}

impl Date {
    pub fn from_timestamp(timestamp: Timestamp) -> Self {
        let year0 = ts_to_year0(timestamp.seconds);
        let seconds_in_year = timestamp.seconds - get_year_start0(year0);

        let month0 = seconds_in_year.div_euclid(SECONDS_PER_MONTH);
        let seconds_in_month = seconds_in_year.rem_euclid(SECONDS_PER_MONTH);

        let day0 = seconds_in_month.div_euclid(SECONDS_PER_DAY);

        Self {
            year0,
            month0,
            day0,
        }
    }

    /// # Panics
    ///
    /// Panics if:
    /// - year is 0,
    /// - month is not in [1, 13],
    /// - day is not in [1, 30].
    pub fn from_ymd(year: i64, month: i64, day: i64) -> Self {
        let year0 = match year.cmp(&0) {
            std::cmp::Ordering::Greater => year - 1,
            std::cmp::Ordering::Less => year,
            std::cmp::Ordering::Equal => panic!("year cannot be 0"),
        };
        assert!((1..=13).contains(&month), "month must be in [1, 13]");
        assert!((1..=30).contains(&day), "day must be in [1, 30]");
        let month0 = month - 1;
        let day0 = day - 1;
        Self {
            year0,
            month0,
            day0,
        }
    }

    /// # Panics
    ///
    /// Panics if:
    /// - month is not in [0, 12],
    /// - day is not in [0, 29].
    pub fn from_ymd0(year0: i64, month0: i64, day0: i64) -> Self {
        assert!((0..=12).contains(&month0), "month0 must be in [0, 12]");
        assert!((0..=29).contains(&day0), "day0 must be in [0, 29]");
        Self {
            year0,
            month0,
            day0,
        }
    }

    /// Returns the franciade number starting from 0.
    ///
    /// A franciade is defined as 4 years, the first franciade ending in year 3.
    /// It is *not* defined as a period of years ending with a sextile year.
    /// Not all franciades are `365*4+1` days long.
    pub fn franciade0(&self) -> i64 {
        ((self.year0 + 2) / 4) - 1
    }

    /// Returns the franciade number starting from 1.
    ///
    /// A franciade is defined as 4 years, the first franciade ending in year 3.
    /// It is *not* defined as a period of years ending with a sextile year.
    /// Not all franciades are `365*4+1` days long.
    pub fn franciade(&self) -> i64 {
        let franciade0 = self.franciade0();
        if franciade0 >= 0 {
            franciade0 + 1
        } else {
            franciade0
        }
    }

    /// Returns the year but starting from 0.
    pub fn year0(&self) -> i64 {
        self.year0
    }

    /// Returns the year but starting from 1.
    pub fn year(&self) -> i64 {
        if self.year0 >= 0 {
            self.year0 + 1
        } else {
            self.year0
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

    /// Returns the timestamp
    pub fn timestamp(&self) -> Timestamp {
        Timestamp {
            seconds: get_year_start0(self.year0)
                + self.month0 * SECONDS_PER_MONTH
                + self.day0 * SECONDS_PER_DAY,
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
            thousand_years,
            five_hundred_years,
            hundred_years,
            fifty_years,
            ten_years,
            five_years,
            one_year
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
