# Calendrier-Web

Website showing the current date in the [French Republican calendar](https://en.wikipedia.org/wiki/French_Republican_calendar), along with the current [decimal time](https://en.wikipedia.org/wiki/Decimal_time).

It is built on the [Calendrier Rust library](https://github.com/Mubelotix/calendrier) and compiled to WebAssembly.

## Features

- High precision calculation of the republican calendar dates using astronomical algorithms.
- Supports both Mean Time and Apparent Solar Time (via the `solar` build of the underlying Rust library).
- Displays the current time in decimal format (10 hours a day, 100 minutes an hour, 100 seconds a minute).
- Visually engaging and correctly aligned with the original laws of the French Convention.
