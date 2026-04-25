There are three components in this folder:

- ./rust - The core library implementation
- ./js - A wrapper around the Rust implementation to produce NPM packages
- ./website - A website built using the JS wrapper

There must be a README.md in all of these folders, and one at root.

- ./rust/README.md - Documents features and Rust usage. Will be shown on the crates.io page.
- ./js/README.md - Should be based on the Rust readme. Adapts Rust examples and Rust-specific stuff to the JS usage. In the introduction, states that it's a wrapper over the Rust introduction that can run using WASM.
- ./website/README.md - Focuses on features, not usage. Should state it's built on the Rust library and link to it.
- ./README.md - Sums most of the specific readmes, excluding tips for contributing/building/running. Should include both JS and Rust examples and state that there is a website. 

READMEs should be kept up-to-date and always follow these guidelines. Not every detail of the implementation belongs to them though, avoid over-complicating them. They should be kept rather simple and engaging.
