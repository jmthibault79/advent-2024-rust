use crate::utils::matrix;
use crate::utils::plane;

const GUARD: char = '^';
const OBSTACLE: char = '#';

fn find_guard(plane: &Vec<Vec<char>>) -> plane::MovingObject {
    for (row, row_vec) in plane.iter().enumerate() {
        for (col, char_at_col) in row_vec.iter().enumerate() {
            if *char_at_col == GUARD {
                // we know that the guard will always be facing up at the start
                return plane::MovingObject {
                    row,
                    col,
                    dir: plane::Direction::Up,
                    out_of_bounds: false,
                };
            }
        }
    }
    panic!("Guard not found");
}

fn d6p1(file_path: &str) -> usize {
    let plane = matrix::as_matrix(file_path);
    let guard = find_guard(&plane);
    let path_to_exit = plane::path_to_exit_turning_right(&plane, &vec![OBSTACLE], &guard);
    let unique_spaces = plane::unique_spaces(&path_to_exit);
    unique_spaces.len()
}

fn d6p2(path: &str) -> usize {
    0
}

pub fn d6() {
    //let path = "inputs/d6sample.txt";
    let path = "inputs/d6.txt";
    let mut result = d6p1(path);
    println!("Result Day 6 Part 1: {}", result);
    result = d6p2(path);
    println!("Result Day 6 Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_one_test() {
        // 2x2 map

        // // move one, valid
        // assert_eq!(move_one(0, 0, 2, 2, Direction::Right), (0, 1, None));
        // assert_eq!(move_one(0, 0, 2, 2, Direction::Down), (1, 0, None));
        // assert_eq!(move_one(1, 1, 2, 2, Direction::Up), (0, 1, None));
        // assert_eq!(move_one(1, 1, 2, 2, Direction::Left), (1, 0, None));

        // // move one, invalid
        // assert_eq!(move_one(0, 0, 2, 2, Direction::Left), (0, 0, Some(Direction::Left)));
        // assert_eq!(move_one(0, 0, 2, 2, Direction::Up), (0, 0, Some(Direction::Up)));
        // assert_eq!(move_one(1, 1, 2, 2, Direction::Down), (1, 1, Some(Direction::Down)));
        // assert_eq!(move_one(1, 1, 2, 2, Direction::Right), (1, 1, Some(Direction::Right)));
    }
}
