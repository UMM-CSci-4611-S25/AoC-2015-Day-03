# AoC-2015-Day-03

A simple Rust exercise based on an Advent of Code problem from 2015, [Day 3: Perfectly Spherical Houses in a Vacuum](https://adventofcode.com/2015/day/3).

We'll go through part 1 together in class, and then have small groups sort out part 2 on your own. Below are the problem statements, followed by a sketch of how we solved part 1 together.

The project is organized so that there are two binaries in the `bin` directory:

- `part1.rs`
- `part2.rs`

Both have a set of commented out unit tests that you should uncomment and get to pass.

You should be able to run a given part with something like

```bash
cargo run --bin part1
```

replacing `part1` with `part2` as appropriate.

You should be able to run all the tests with `cargo test`.

## Problem statement

The following problem statements are taken directly from [the Advent of Code problem statements](https://adventofcode.com/2015/day/3).
Typically part 2 of Advent of Code problems is not visible until you've completed part 1,
so I'm cheating slightly by sharing the problem statement of part 2 with everyone.

### Part 1

Santa is delivering presents to an infinite two-dimensional grid of houses.

He begins by delivering a present to the house at his starting location, and then
an elf at the North Pole calls him via radio and tells him where to move next.
Moves are always exactly one house to the north (`^`), south (`v`), east (`>`), or west (`<`).
After each move, he delivers another present to the house at his new location.

However, the elf back at the north pole has had a little too much eggnog, and so
his directions are a little off, and Santa ends up visiting some houses more than
once. How many houses receive at least one present?

For example:

- `>` delivers presents to 2 houses: one at the starting location, and one to the east.
- `^>v<` delivers presents to 4 houses in a square, including twice to the house at
  his starting/ending location.
- `^v^v^v^v^v` delivers a bunch of presents to some very lucky children at only 2 houses.

### Part 2

The next year, to speed up the process, Santa creates a robot version of himself,
Robo-Santa, to deliver presents with him.

Santa and Robo-Santa start at the same location (delivering two presents to the
same starting house), then take turns moving based on instructions from the elf,
who is eggnoggedly reading from the same script as the previous year.

This year, how many houses receive at least one present?

For example:

- `^v` delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
- `^>v<` now delivers presents to 3 houses, and Santa and Robo-Santa end up back where
  they started.
- `^v^v^v^v^v` now delivers presents to 11 houses, with Santa going one direction and
  Robo-Santa going the other.

---

## Solution sketch for part 1

I've provided unit tests that will cover most of the necessary logic, and we'll let those
drive our development. Below we'll mark key decisions that need to be made as we go.

### Create a `VisitedHouses` type

To get the first test to pass, we needed to implement:

- `VisitedHouses::new()`
- `visitedHouses.num_visited_houses()`

For starters we did really simple (and incorrect) implementations; we just returned 1 from
`num_visited_houses()`, for example.

We also implemented `Default` because Clippy suggested
that it would be a good idea.

### Create a `Pos` type

To get the first test to compile and run, we needed some
type for `visited_houses.current_position()` to return.

We discussed several options:

- `(i32, i32)`
- `type Pos = (i32, i32)`
- `struct Pos(i32, i32)`
- `struct Pos { x: i32, y: i32 }`

In the end we decided to use the last (a named
field struct) so we'd have a named type with
named fields.

We found we need to implement both the `PartialEq` and
`Debug` traits in order to get the test to pass. We
just used `#[derive()]` for both of those, and
Nic added `Eq` to the list for good measure.

After class, Nic added a `new` method to our `Pos`
type to make it easier to create new instances of `Pos`.

> **Note:** This is an example of something we couldn't
> do if we had gone with a simple tuple like `(i32, i32)`.
> We arguably wouldn't _need_ to do it if we'd stuck with
> a simple tuple, but it's worth noting that we couldn't
> have even if we'd wanted to.

### Fleshing out `VisitedHouses`

Currently `VisitedHouses` is an empty structure, and
that clearly won't work. So we'll need to add some
fields to `VisitedHouses` that contain the information
we need to solve the problem.

So far our tests require that we be able to answer
two questions:

- How many houses have we visited? (`num_visited_houses()`)
- What is the current position of Santa? (`current_pos()`)

We could answer both of these by adding fields with the
same or similar names, and then just return those fields
as our answers. That probably makes sense for
`current_pos()`, but it's less clear that it's a good
idea for `num_visited_houses()`. Looking ahead, it
seems likely that we would rather have a set (`HashSet`)
of positions (`Pos`s), and return the size of that
for `num_visited_houses`.

So we went with:

```rust
pub struct VisitedHouses {
    visited_houses: HashSet<Pos>,
    current_position: Pos,
}
```

> **That ended class on Tuesday, 11 Feb, and we'll continue
on Thursday, 13 Feb.**

---

### Turn on a ton of Clippy warnings

I realized that we didn't have all the Clippy warnings turned on that I would
have liked, so I fixed that by adding:

```toml
[lints.clippy]
pedantic = { level = "warn", priority = -1 }
nursery = { level = "warn", priority = -1 }
perf = { level = "warn", priority = -1 }
complexity = { level = "warn", priority = -1 }
correctness = { level = "deny", priority = -1 }
style = { level = "warn", priority = -1 }
```

to `Cargo.toml`.

### Create a `Direction` type

We now needed a `Direction` type, namely an `enum` representing
the four cardinal directions that Santa can move in:

```rust
pub enum Direction {
    North,
    South,
    East,
    West,
}
```

We then implemented `VisitedHouses::perform_move(&mut self, Direction)`
to update the game state by processing that move. We also provided "real"
implementations of `num_visited_houses()` and `current_pos()`, which required
deriving `Hash + Clone + Copy` for `Pos`.

> **That ended class on Thursday, 13 Feb, and we'll continue
on Tuesday, 18 Feb.**

---
