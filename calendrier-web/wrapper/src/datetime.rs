use crate::*;

#[wasm_bindgen]
pub struct DateTime(InnerDateTime);

#[wasm_bindgen]
impl DateTime {
    #[wasm_bindgen(constructor)]
    pub fn from_timestamp(timestamp: &Timestamp) -> DateTime {
        DateTime(InnerDateTime::from_timestamp(timestamp.0))
    }

    #[wasm_bindgen]
    pub fn from_ymd_hms(year: i64, month: i64, day: i64, hour: i64, minute: i64, second: i64) -> DateTime {
        DateTime(InnerDateTime::from_ymd_hms(year, month, day, hour, minute, second))
    }

    #[wasm_bindgen]
    pub fn from_ymd_hms0(year0: i64, month0: i64, day0: i64, hour: i64, minute: i64, second: i64) -> DateTime {
        DateTime(InnerDateTime::from_ymd_hms0(year0, month0, day0, hour, minute, second))
    }

    #[wasm_bindgen]
    pub fn from_ymd(year: i64, month: i64, day: i64) -> DateTime {
        DateTime(InnerDateTime::from_ymd(year, month, day))
    }

    #[wasm_bindgen]
    pub fn from_ymd0(year0: i64, month0: i64, day0: i64) -> DateTime {
        DateTime(InnerDateTime::from_ymd0(year0, month0, day0))
    }

    #[wasm_bindgen(getter)]
    pub fn franciade0(&self) -> i64 {
        self.0.franciade0()
    }

    #[wasm_bindgen(getter)]
    pub fn franciade(&self) -> i64 {
        self.0.franciade()
    }

    #[wasm_bindgen(getter)]
    pub fn year0(&self) -> i64 {
        self.0.year0()
    }

    #[wasm_bindgen(getter)]
    pub fn year(&self) -> i64 {
        self.0.year()
    }

    pub fn num_month0(&self) -> i64 {
        self.0.num_month0()
    }

    #[wasm_bindgen(getter)]
    pub fn num_month(&self) -> i64 {
        self.0.num_month()
    }

    #[wasm_bindgen(getter)]
    pub fn month(&self) -> String {
        self.0.month().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn day0(&self) -> i64 {
        self.0.day0()
    }

    #[wasm_bindgen(getter)]
    pub fn day(&self) -> i64 {
        self.0.day()
    }

    #[wasm_bindgen(getter)]
    pub fn day_name(&self) -> String {
        self.0.day_name().to_owned()
    }

    #[wasm_bindgen(getter)]
    pub fn day_name_with_article(&self) -> String {
        self.0.day_name_with_article().to_owned()
    }

    #[wasm_bindgen(getter)]
    pub fn decade0(&self) -> i64 {
        self.0.decade0()
    }

    #[wasm_bindgen(getter)]
    pub fn decade(&self) -> i64 {
        self.0.decade()
    }

    #[wasm_bindgen(getter)]
    pub fn num_decade_day0(&self) -> i64 {
        self.0.num_decade_day0()
    }

    #[wasm_bindgen(getter)]
    pub fn num_decade_day(&self) -> i64 {
        self.0.num_decade_day()
    }

    #[wasm_bindgen(getter)]
    pub fn decade_day(&self) -> String {
        self.0.decade_day().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn hour(&self) -> i64 {
        self.0.hour()
    }

    #[wasm_bindgen(getter)]
    pub fn minute(&self) -> i64 {
        self.0.minute()
    }

    #[wasm_bindgen(getter)]
    pub fn second(&self) -> i64 {
        self.0.second()
    }

    #[wasm_bindgen(getter)]
    pub fn hms(&self) -> Vec<i64> {
        let mut res = Vec::new();
        let (h, m, s) = self.0.hms();
        res.push(h);
        res.push(m);
        res.push(s);
        res
    }

    #[wasm_bindgen(getter)]
    pub fn timestamp(&self) -> Timestamp {
        Timestamp(self.0.timestamp())
    }
    
    #[wasm_bindgen]
    pub fn to_string_default(&self) -> String {
        self.0.to_string_default()
    }

    #[wasm_bindgen]
    pub fn to_string_traditional(&self) -> String {
        self.0.to_string_traditional()
    }
}
