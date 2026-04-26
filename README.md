# calendrier

Crate and npm package for handling dates in the french Revolutionary calendar.
This is the most precise and correct implementation of the Republican calendar in the world. 

There is also a [website](https://calendrier.dera.page) showing the current date and decimal time, which leverages this library via WebAssembly.

## Unrivaled Technical Precision

While most libraries approximate dates using the never-adopted Romme reform or fixed leap-year rules, this project uses high-precision astronomical calculations to adhere strictly to the original law of the Convention.
To achieve historical truth, we leverage the [NOVAS](https://github.com/Mubelotix/novas) library. This allows us to compute precise astronomical times using simulated positions of the Sun to support Paris Apparent Solar Time.

Our implementation has been rigorously validated against historical records, matching precise event timings from 1792 and 1793 with an accuracy better than 90 seconds. It provides correct dates from 1583 to 2999.

Due to the complexity of non-linear time, the solar time implementation is provided as an opt-in feature. By default, it uses Mean Time to ensure consistent second durations.

## Rust Usage

```rust
use calendrier::*;

let date = DateTime::from_ymd(1, 1, 1); // Calendar starts on september 22nd, 1792
assert_eq!(date.to_string().as_str(), "1 Vendémiaire 1");

let ts = date.timestamp();
assert_eq!(ts.to_unix(), -5594228280);
```

## JavaScript / WebAssembly Usage

The Javascript port is a WebAssembly wrapper around the Rust implementation.

```javascript
import init, { DateTime } from '@mubelotix/calendrier'; // Or @mubelotix/calendrier-mt for mean time

await init(); // In Node.js, see the package README for WASM loading instructions

const date = DateTime.from_ymd(1n, 1n, 1n); // Calendar starts on september 22nd, 1792
console.log(date.to_string_default());     // "Primidi 1 Vendémiaire 1"
console.log(date.to_string_traditional()); // "1 Vendémiaire 1"

const ts = date.timestamp();
console.assert(ts.to_unix() === -5594228280n);
```
