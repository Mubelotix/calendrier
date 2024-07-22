#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Month {
    Vendémiaire,
    Brumaire,
    Frimaire,
    Nivôse,
    Pluviôse,
    Ventôse,
    Germinal,
    Floréal,
    Prairial,
    Messidor,
    Thermidor,
    Fructidor,
    Sansculotides
}

impl Month {
    pub fn name(&self) -> &'static str {
        match self {
            Month::Vendémiaire => "Vendémiaire",
            Month::Brumaire => "Brumaire",
            Month::Frimaire => "Frimaire",
            Month::Nivôse => "Nivôse",
            Month::Pluviôse => "Pluviôse",
            Month::Ventôse => "Ventôse",
            Month::Germinal => "Germinal",
            Month::Floréal => "Floréal",
            Month::Prairial => "Prairial",
            Month::Messidor => "Messidor",
            Month::Thermidor => "Thermidor",
            Month::Fructidor => "Fructidor",
            Month::Sansculotides => "Sansculotides",
        }
    }

    pub fn name_lc(&self) -> &'static str {
        match self {
            Month::Vendémiaire => "vendémiaire",
            Month::Brumaire => "brumaire",
            Month::Frimaire => "frimaire",
            Month::Nivôse => "nivôse",
            Month::Pluviôse => "pluviôse",
            Month::Ventôse => "ventôse",
            Month::Germinal => "germinal",
            Month::Floréal => "floréal",
            Month::Prairial => "prairial",
            Month::Messidor => "messidor",
            Month::Thermidor => "thermidor",
            Month::Fructidor => "fructidor",
            Month::Sansculotides => "sansculotides",
        }
    }

    pub fn num0(&self) -> i64 {
        match self {
            Month::Vendémiaire => 0,
            Month::Brumaire => 1,
            Month::Frimaire => 2,
            Month::Nivôse => 3,
            Month::Pluviôse => 4,
            Month::Ventôse => 5,
            Month::Germinal => 6,
            Month::Floréal => 7,
            Month::Prairial => 8,
            Month::Messidor => 9,
            Month::Thermidor => 10,
            Month::Fructidor => 11,
            Month::Sansculotides => 12,
        }
    }

    pub fn num(&self) -> i64 {
        self.num0() + 1
    }

    /// # Panics
    /// 
    /// Panics if `num0` is greater than 12.
    pub fn from_num0(num0: i64) -> Self {
        match num0 {
            0 => Month::Vendémiaire,
            1 => Month::Brumaire,
            2 => Month::Frimaire,
            3 => Month::Nivôse,
            4 => Month::Pluviôse,
            5 => Month::Ventôse,
            6 => Month::Germinal,
            7 => Month::Floréal,
            8 => Month::Prairial,
            9 => Month::Messidor,
            10 => Month::Thermidor,
            11 => Month::Fructidor,
            12 => Month::Sansculotides,
            _ => panic!("Invalid month number: {}", num0)
        }
    }

    /// # Panics
    /// 
    /// Panics if `num` is greater than 11.
    pub fn from_num(num: i64) -> Self {
        Self::from_num0(num - 1)
    }
}

impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
