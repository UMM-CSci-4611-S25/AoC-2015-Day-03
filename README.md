# AoC-2015-Day-03

A simple Rust exercise based on an Advent of Code problem from 2015, [Day 3: Perfectly Spherical Houses in a Vacuum](https://adventofcode.com/2015/day/3).

## Decide on a location data structure

There are several options:

- A tuple like `(i32, i32)`
- An array like `[i32; 2]`
- A struct, with a couple of alternatives
  - A tuple struct like `struct Pos(i32, i32)`
  - A field struct (with named fields) like `struct Pos { x: i32, y: i32 }`

Feel free to try this several different ways, but make sure that your "primary" solution uses a `struct` because I want us to talk about the necessary traits that we have `derive` in order to create a `HashSet` of `Pos`s.

Mention `cargo expand` and VSCode macro expansion tools so that we can see what `derive` is actually doing.
- In VSCode, click on the name of the derive (e.g., `Hash` or `PartialEq`) and use the command palette to select "rust-analyzer: Expand macro recursively at caret".
- On the command line, `cargo expand` will show your code after full macro expansion. You can use `cargo expand --tests` if you want to see how test code is handled.
  - This requires that you first install `cargo-expand` with something like `cargo install cargo-expand`.
  - This has some additional detail not shown in VSCode, and you'll probably find the VSCode/rust-analyzer expansion easier to read.

## Create a direction type

Use an `enum`.
- Implement the `TryFrom<char>` trait to convert from input characters to `Direction`s.
- This implies some sort of error type like `struct IllegalCharacterError {}`.

## Optional: Use the `Add` trait to implement moving positions

## Optional: Implement `Error` and `Display` on our error type

To be good Rust citizens we would implement the `std::error::Error` and `Display` traits for our error type. We didn't do it above in order to keep things simple, but we should probably do it now.

Introduce `thiserror`

We could introduce `thiserror` after we get everything working if there's time and interest.

