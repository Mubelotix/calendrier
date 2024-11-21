pub mod date;
pub mod datetime;
pub mod day;
pub mod months;
pub mod timestamp;
pub mod years;
pub use date::*;
pub use datetime::*;
pub use day::*;
pub use months::*;
pub use timestamp::*;
pub mod day_names;
pub mod equinoxes;
pub use day_names::*;
pub use equinoxes::*;
pub use years::*;
#[cfg(feature = "chrono")]
pub mod chrono_compat;
pub mod generated;

pub(crate) const SECONDS_PER_DAY: i64 = 100000;
pub(crate) const DAYS_PER_MONTH: i64 = 30;
pub(crate) const DAYS_PER_DECADE: i64 = 10;
pub(crate) const SECONDS_PER_MONTH: i64 = SECONDS_PER_DAY * DAYS_PER_MONTH;
