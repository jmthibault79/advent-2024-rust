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
    let plane = matrix::as_char_matrix(file_path);
    let guard = find_guard(&plane);
    if let Some(path_to_exit) = plane::path_to_exit_turning_right(&plane, &vec![OBSTACLE], &guard) {
        let unique_spaces = plane::unique_spaces(&path_to_exit);
        unique_spaces.len()
    } else {
        0
    }
}

fn d6p2(file_path: &str) -> usize {
    const NEW_OBSTACLE: char = 'O';

    let plane = matrix::as_char_matrix(file_path);
    let guard = find_guard(&plane);
    let alt_planes = matrix::replace_one_cell(&plane, NEW_OBSTACLE);
    println!("Alt planes: {:?}", alt_planes.len());
    alt_planes
        .iter()
        .enumerate()
        .map(|(idx, alt_plane)| {
            if idx % 100 == 0 {
                println!("Alt plane: {}", idx);
            }
            match plane::path_to_exit_turning_right(
                &alt_plane,
                &vec![OBSTACLE, NEW_OBSTACLE],
                &guard,
            ) {
                None => {
                    // println!("Loop found");
                    // matrix::pretty_print(alt_plane);
                    // println!();
                    1
                }
                _ => 0,
            }
        })
        .sum()
    // 1792 is too low!
}

pub fn d6() {
    //let path = "inputs/d6sample.txt";
    let path = "inputs/d6.txt";
    let mut result = d6p1(path);
    println!("Result Day 6 Part 1: {}", result);
    result = d6p2(path);
    println!("Result Day 6 Part 2: {}", result);
}
