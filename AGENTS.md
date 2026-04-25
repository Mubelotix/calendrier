There are three components in this folder:

- ./rust - The core library implementation
- ./js - A wrapper around the Rust implementation to produce NPM packages
- ./web - A website built using the JS wrapper

There must be a README.md in all of these folders, and one at root.

- ./rust/README.md - Documents features and Rust usage. Will be shown on the crates.io page.
- ./js/README.md - Should be based on the Rust readme. Will be shown on npmjs.com for calendrier-solar. Adapts Rust examples and Rust-specific stuff to the JS usage. In the introduction, states that it's a wrapper over the Rust introduction that can run using WASM. It should state that solar time is enabled and link to the non-solar version for those who want to use mean time instead.
- ./js/README-non-solar.md - Should be based on the previous README. It should state solar time is disabled. Thus, the paragraph that talks about the equation of time seconds should be removed, along with other paragraphs about solar time and precision that should be rephrased. It should also link to the other version for those who value historical accuracy.
- ./web/README.md - Focuses on features, not usage. Should state it's built on the Rust library and link to it.
- ./README.md - Sums most of the specific readmes, excluding tips for contributing/building/running. Should include both JS and Rust examples and state that there is a website. 

Note: npm packages are published under the scope `mubelotix`.

READMEs should be kept up-to-date and always follow these guidelines. Not every detail of the implementation belongs to them though, avoid over-complicating them. They should be kept rather simple and engaging.
