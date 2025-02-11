fn main() {
    let input_file_name = "input.txt";
    let contents = std::fs::read_to_string(input_file_name).expect("Failed to read the input file");
    println!("{}", contents.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_visited_houses_new() {
    //     let mut visited_houses = VisitedHouses::new();
    //     assert_eq!(visited_houses.num_visited_houses(), 1);
    //     // What do you want to do about the current position?
    //     // assert_eq!(visited_houses.current_pos, Pos(0, 0));
    // }

    // #[test]
    // fn test_direction_try_from() {
    //     assert_eq!('^'.try_into(), Ok(Direction::North));
    //     assert_eq!('v'.try_into(), Ok(Direction::South));
    //     assert_eq!('<'.try_into(), Ok(Direction::We st));
    //     assert_eq!('>'.try_into(), Ok(Direction::East));
    //     assert_eq!('x'.try_into(), Err(IllegalDirectionCharacter('x')));
    // }

    // #[test]
    // fn test_move_north_south() {
    //     let mut visited_houses = VisitedHouses::new();
    //     visited_houses.perform_move(Direction::North);
    //     visited_houses.perform_move(Direction::South);
    //     assert_eq!(visited_houses.num_visited_houses(), 3);
    //     // assert_eq!(visited_houses.current_pos, Pos(1, 0));
    // }

    // #[test]
    // fn test_square_moves() {
    //     let mut visited_houses = VisitedHouses::new();
    //     let moves = Moves::from_str("^>v<").unwrap();
    //     visited_houses.perform_moves(moves);
    //     assert_eq!(visited_houses.num_visited_houses(), 3);
    //     // assert_eq!(visited_houses.current_pos, Pos(0, 0));
    // }

    // #[test]
    // fn test_up_down_moves() {
    //     let mut visited_houses = VisitedHouses::new();
    //     let moves = Moves::from_str("^v^v^v^v^v").unwrap();
    //     visited_houses.perform_moves(moves);
    //     assert_eq!(visited_houses.num_visited_houses(), 11);
    //     // assert_eq!(visited_houses.current_pos, Pos(0, 0));
    // }

    // #[test]
    // fn test_aoc_input() {
    //     let mut visited_houses = VisitedHouses::new();
    //     let moves = Moves::from_str(include_str!("../input.txt")).unwrap();
    //     visited_houses.perform_moves(moves);
    //     assert_eq!(visited_houses.num_visited_houses(), 2639);
    // }
}
