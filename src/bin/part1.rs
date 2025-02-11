use std::{
    collections::HashSet,
    ops::{Add, AddAssign},
    str::FromStr,
};

fn main() {
    let input_file_name = "input.txt";
    let contents = std::fs::read_to_string(input_file_name)
        .expect("Failed to read the input file")
        .trim()
        .to_string();

    let moves = Moves::from_str(&contents).expect("Failed to parse moves");

    let mut visited_houses = Santa::new();
    visited_houses.perform_moves(moves);

    let num_visited_houses = visited_houses.num_visited_houses();

    println!("The number of visited houses was {num_visited_houses}");
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IllegalDirectionCharacter(char);

impl TryFrom<char> for Direction {
    type Error = IllegalDirectionCharacter;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::North),
            'v' => Ok(Self::South),
            '<' => Ok(Self::West),
            '>' => Ok(Self::East),
            illegal_char => Err(IllegalDirectionCharacter(illegal_char)),
        }
    }
}

pub struct Moves(Vec<Direction>);

impl FromStr for Moves {
    type Err = IllegalDirectionCharacter;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(<char as TryInto<Direction>>::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add<Direction> for Pos {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::North => Self::new(self.x, self.y + 1),
            Direction::South => Self::new(self.x, self.y - 1),
            Direction::East => Self::new(self.x + 1, self.y),
            Direction::West => Self::new(self.x - 1, self.y),
        }
    }
}

impl AddAssign<Direction> for Pos {
    fn add_assign(&mut self, direction: Direction) {
        *self = *self + direction;
    }
}

pub struct Santa {
    visited_houses: HashSet<Pos>,
    current_position: Pos,
}

impl Santa {
    #[must_use]
    pub fn new() -> Self {
        let current_position = Pos::new(0, 0);
        let mut visited_houses = HashSet::new();
        visited_houses.insert(current_position);

        Self {
            visited_houses,
            current_position,
        }
    }

    #[must_use]
    pub fn num_visited_houses(&self) -> usize {
        self.visited_houses.len()
    }

    #[must_use]
    pub const fn current_pos(&self) -> Pos {
        self.current_position
    }

    pub fn perform_move(&mut self, direction: Direction) {
        self.current_position += direction;
        self.visited_houses.insert(self.current_position);
    }

    pub fn perform_moves(&mut self, moves: Moves) {
        for direction in moves.0 {
            self.perform_move(direction);
        }
    }
}

impl Default for Santa {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visited_houses_new() {
        let santa = Santa::new();
        assert_eq!(santa.num_visited_houses(), 1);
        assert_eq!(santa.current_pos(), Pos::new(0, 0));
    }

    #[test]
    fn test_direction_try_from() {
        assert_eq!('^'.try_into(), Ok(Direction::North));
        assert_eq!('v'.try_into(), Ok(Direction::South));
        assert_eq!('<'.try_into(), Ok(Direction::West));
        assert_eq!('>'.try_into(), Ok(Direction::East));

        let result: Result<Direction, IllegalDirectionCharacter> = 'x'.try_into();
        assert_eq!(result, Err(IllegalDirectionCharacter('x')));

        assert_eq!(
            <char as std::convert::TryInto<Direction>>::try_into('x'),
            Err(IllegalDirectionCharacter('x'))
        );
    }

    #[test]
    fn test_move_east() {
        let mut santa = Santa::new();
        santa.perform_move(Direction::East);
        assert_eq!(santa.num_visited_houses(), 2);
        assert_eq!(santa.current_pos(), Pos::new(1, 0));
    }

    #[test]
    fn test_square_moves() {
        let mut santa = Santa::new();
        let moves = Moves::from_str("^>v<").unwrap();
        santa.perform_moves(moves);
        assert_eq!(santa.num_visited_houses(), 4);
        assert_eq!(santa.current_pos(), Pos::new(0, 0));
    }

    #[test]
    fn test_up_down_moves() {
        let mut santa = Santa::new();
        let moves = Moves::from_str("^v^v^v^v^v").unwrap();
        santa.perform_moves(moves);
        assert_eq!(santa.num_visited_houses(), 2);
        assert_eq!(santa.current_pos(), Pos::new(0, 0));
    }

    #[test]
    fn test_aoc_input() {
        let mut santa = Santa::new();
        let moves = Moves::from_str(include_str!("../../input.txt").trim()).unwrap();
        santa.perform_moves(moves);
        assert_eq!(santa.num_visited_houses(), 2565);
    }
}
