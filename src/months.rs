use crate::*;

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
}
