# Stream notes, 2025-02-08

These are some design notes from a conversation during Nic's Twitch stream on Saturday,
2025-02-08, where we discussed this problem as an exercise. They're fairly ragged, but
there's stuff here that Nic wanted to keep, so he put them here. Folks should feel free
to use or ignore them as they see fit.

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

Use an `enum`:

- Implement the `TryFrom<char>` trait to convert from input characters to `Direction`s.
- This implies some sort of error type like `struct IllegalCharacterError {}`.

## Optional: Use the `Add` trait to implement moving positions

## Optional: Implement `Error` and `Display` on our error type

To be good Rust citizens we would implement the `std::error::Error` and `Display` traits for our error type. We didn't do it above in order to keep things simple, but we should probably do it now.

Introduce `thiserror`

We could introduce `thiserror` after we get everything working if there's time and interest.

---

## Notes as a big comment

Before I switched to a more TDD approach, I included a list of tasks as a big comment.
I'm saving that here in case it's useful.

```rust
// TASKS
//
// 1. Create a type for positions on a 2D grid. It should have two fields, x and y, both of which are i32.
//    a. This could be a tuple (possibly with a name using `type`)
//    b. This could be a tuple struct
//    c. This could be a field struct with named fields
// 2. We have to be able to put that in a `HashSet`, which is going to require implementing (or _deriving_) a bunch of traits.
// 3. Create a type for directions. It should have four variants: North, South, East, and West.
//    - We don't _have_ to have this, but it's a good idea to have a type that represents the domain of the problem.
//    - It also localizes checking for illegal input characters to one place, and from there on we can assume that the input is valid.
// 4. Stub `TryFrom<char>` for `Direction`. This will allow us to convert a character to a `Direction` if it's one of the four we know about.
//    - This will allow us to use `c.try_into()` to convert a character to a `Direction`.
//    - If the character is not one of the four we know about, return an error.
// 5. Create an error type for when a character can't be converted to a `Direction`.
//    - This could be an empty struct or a simple enum with one variant.
//    - We probably want it to hold the character that caused the error.
//       - This could be a field in the error struct.
//       - This could be a tuple struct with one field.
//       - This could be an enum with one variant that holds the character.
//    - This should be a newtype struct that wraps a `char`.
//    - We'll eventually need various traits for this type; we'll derive them as they come up.
// 6. Implement `TryFrom<char>` for `Direction`.
```
