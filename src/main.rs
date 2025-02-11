use std::{collections::HashSet, str::FromStr};

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

#[derive(Hash, PartialEq, Eq)]
struct Pos(i32, i32);

enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct IllegalDirectionCharacter(char);

impl TryFrom<char> for Direction {
    type Error = IllegalDirectionCharacter;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '^' => Self::North,
            'v' => Self::South,
            '<' => Self::West,
            '>' => Self::East,
            bad_char => return Err(IllegalDirectionCharacter(bad_char)),
        })
    }
}

struct Moves(Vec<Direction>);

impl FromStr for Moves {
    type Err = IllegalDirectionCharacter;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Moves(
            s.chars()
                .map(|c| c.try_into())
                .collect::<Result<Vec<_>, Self::Err>>()?,
        ))
    }
}

struct VisitedHouses {
    current_pos: Pos,
    houses: HashSet<Pos>,
}

impl VisitedHouses {
    fn new() -> Self {
        Self {
            current_pos: Pos(0, 0),
            houses: HashSet::new(),
        }
    }

    fn num_visited_houses(&self) -> usize {
        self.houses.len()
    }

    fn perform_move(&mut self, direction: Direction) {
        todo!();
    }

    fn perform_moves(&mut self, Moves(moves): Moves) {
        todo!();
    }
}

fn main() {
    let mut set: HashSet<Pos> = HashSet::new();

    set.insert(Pos(5, 8));
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_visited_houses_new() {
    //     let mut visited_houses = VisitedHouses::new();
    //     assert_eq!(visited_houses.num_visited_houses(), 1);
    //     assert_eq!(visited_houses.current_pos, Pos(0, 0));
    // }

    // #[test]
    // fn test_direction_try_from() {
    //     assert_eq!('^'.try_into(), Ok(Direction::North));
    //     assert_eq!('v'.try_into(), Ok(Direction::South));
    //     assert_eq!('<'.try_into(), Ok(Direction::West));
    //     assert_eq!('>'.try_into(), Ok(Direction::East));
    //     assert_eq!('x'.try_into(), Err(IllegalDirectionCharacter('x')));
    // }

    // #[test]
    // fn test_move_east() {
    //     let mut visited_houses = VisitedHouses::new();
    //     visited_houses.perform_move(Direction::East);
    //     assert_eq!(visited_houses.num_visited_houses(), 2);
    //     assert_eq!(visited_houses.current_pos, Pos(1, 0));
    // }

    // #[test]
    // fn test_square_moves() {
    //     let mut visited_houses = VisitedHouses::new();
    //     let moves = Moves::from_str("^>v<").unwrap();
    //     visited_houses.perform_moves(moves);
    //     assert_eq!(visited_houses.num_visited_houses(), 4);
    //     assert_eq!(visited_houses.current_pos, Pos(0, 0));
    // }

    // #[test]
    // fn test_up_down_moves() {
    //     let mut visited_houses = VisitedHouses::new();
    //     let moves = Moves::from_str("^v^v^v^v^v").unwrap();
    //     visited_houses.perform_moves(moves);
    //     assert_eq!(visited_houses.num_visited_houses(), 2);
    //     assert_eq!(visited_houses.current_pos, Pos(0, 0));
    // }

    // #[test]
    // fn test_aoc_input() {
    //     let mut visited_houses = VisitedHouses::new();
    //     let moves = Moves::from_str(include_str!("../input.txt")).unwrap();
    //     visited_houses.perform_moves(moves);
    //     assert_eq!(visited_houses.num_visited_houses(), 2565);
    // }
}
