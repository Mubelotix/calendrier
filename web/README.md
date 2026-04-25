# Calendrier-Web

Website showing the current date in the [French Republican calendar](https://en.wikipedia.org/wiki/French_Republican_calendar), along with the current [decimal time](https://en.wikipedia.org/wiki/Decimal_time).

## Building the library

While this project is written entirely in JavaScript, it uses my [Rust library](https://github.com/Mubelotix/calendrier) for date calculations. The wrapper now ships in two variants: the default build and a `solar` build.

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install the WebAssembly target
rustup target add wasm32-unknown-unknown

# Install wasm-pack
cargo install wasm-pack

# Build both wrapper variants
cd wrapper
./build.sh
```

## Running

You just need to run a local web server in `./static`. For example, using Python:

```bash
cd static
python3 -m http.server
```

There are symlinks in the `static` folder that point to the compiled files in `./wrapper/pkg` and `./wrapper/pkg-solar`.
The default site uses `calendrier_web_bg.wasm`, and `calendrier_web_solar_bg.wasm` is exposed alongside it for comparison.
Your web server must support serving symlinks (just use python).
