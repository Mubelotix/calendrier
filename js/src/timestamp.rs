use crate::*;

#[wasm_bindgen]
pub struct Timestamp(pub(crate) InnerTimestamp);

#[wasm_bindgen]
impl Timestamp {
    #[wasm_bindgen(constructor)]
    pub fn from_seconds(seconds: i64) -> Timestamp {
        Timestamp(InnerTimestamp { seconds })
    }

    #[wasm_bindgen]
    pub fn from_unix(unix: i64) -> Timestamp {
        Timestamp(InnerTimestamp::from_unix(unix))
    }

    #[wasm_bindgen]
    pub fn unix(&self) -> i64 {
        self.0.to_unix()
    }

    #[wasm_bindgen(getter)]
    pub fn seconds(&self) -> i64 {
        self.0.seconds
    }

    #[wasm_bindgen(setter)]
    pub fn set_seconds(&mut self, seconds: i64) {
        self.0.seconds = seconds;
    }
}
