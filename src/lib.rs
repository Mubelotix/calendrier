pub mod datetime;
pub mod timestamp;
pub mod date;
pub mod day;
pub mod months;
pub use datetime::*;
pub use timestamp::*;
pub use date::*;
pub use day::*;
pub use months::*;
pub mod equinoxes;
pub use equinoxes::*;
pub mod years;
pub use years::*;
#[cfg(feature = "chrono")]
pub mod chrono_compat;

pub(crate) const SECONDS_PER_DAY: i64 = 100000;
pub(crate) const DAYS_PER_MONTH: i64 = 30;
pub(crate) const DAYS_PER_DECADE: i64 = 10;
pub(crate) const SECONDS_PER_MONTH: i64 = SECONDS_PER_DAY*DAYS_PER_MONTH;
