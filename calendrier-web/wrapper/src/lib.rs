use wasm_bindgen::prelude::*;
use calendrier::{timestamp::Timestamp as InnerTimestamp, datetime::DateTime as InnerDateTime};

mod timestamp;
mod datetime;

pub use timestamp::*;
pub use datetime::*;

#[cfg(feature = "solar")]
#[wasm_bindgen]
pub fn get_solar_time_speed_ratio(unix_timestamp: i64, window: usize) -> f64 {
	calendrier::solar::get_solar_time_speed_ratio(unix_timestamp, window)
}
