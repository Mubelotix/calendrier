#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Day {
    Regular(RegularDay),
    Sansculottide(SansculottideDay),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegularDay {
    Primedi,
    Duodi,
    Tridi,
    Quartidi,
    Quintidi,
    Sextidi,
    Septidi,
    Octidi,
    Nonidi,
    Décadi,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SansculottideDay {
    Vertu,
    Génie,
    Travail,
    Opinion,
    Récompenses,
    Révolution,
}

impl RegularDay {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Primedi => "Primidi",
            Self::Duodi => "Duodi",
            Self::Tridi => "Tridi",
            Self::Quartidi => "Quartidi",
            Self::Quintidi => "Quintidi",
            Self::Sextidi => "Sextidi",
            Self::Septidi => "Septidi",
            Self::Octidi => "Octidi",
            Self::Nonidi => "Nonidi",
            Self::Décadi => "Décadi",
        }
    }

    pub fn name_lc(&self) -> &'static str {
        match self {
            Self::Primedi => "primidi",
            Self::Duodi => "duodi",
            Self::Tridi => "tridi",
            Self::Quartidi => "quartidi",
            Self::Quintidi => "quintidi",
            Self::Sextidi => "sextidi",
            Self::Septidi => "septidi",
            Self::Octidi => "octidi",
            Self::Nonidi => "nonidi",
            Self::Décadi => "décadi",
        }
    }
}

impl std::fmt::Display for RegularDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl SansculottideDay {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Vertu => "Jour de la vertu",
            Self::Génie => "Jour du génie",
            Self::Travail => "Jour du travail",
            Self::Opinion => "Jour de l'opinion",
            Self::Récompenses => "Jour des récompenses",
            Self::Révolution => "Jour de la Révolution",
        }
    }

    pub fn name_lc(&self) -> &'static str {
        match self {
            Self::Vertu => "jour de la vertu",
            Self::Génie => "jour du génie",
            Self::Travail => "jour du travail",
            Self::Opinion => "jour de l'opinion",
            Self::Récompenses => "jour des récompenses",
            Self::Révolution => "jour de la Révolution",
        }
    }
}

impl std::fmt::Display for SansculottideDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Day {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Regular(day) => day.name(),
            Self::Sansculottide(day) => day.name(),
        }
    }

    pub fn name_lc(&self) -> &'static str {
        match self {
            Self::Regular(day) => day.name_lc(),
            Self::Sansculottide(day) => day.name_lc(),
        }
    }
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regular(day) => write!(f, "{}", day),
            Self::Sansculottide(day) => write!(f, "{}", day),
        }
    }
}
