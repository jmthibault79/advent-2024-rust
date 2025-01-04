use crate::utils::matrix;
use crate::utils::plane;
use crate::utils::plane::Direction;

const WALL: char = '#';
const START: char = 'S';
const END: char = 'E';
const START_DIRECTION: Direction = Direction::Right;

fn shortest_path(maze: &Vec<Vec<char>>) -> usize {
    let (start_row, start_col) = plane::find_unique_element(&maze, START);
    let (end_row, end_col) = plane::find_unique_element(&maze, END);
    let (row_count, col_count) = matrix::dimensions(&maze);

    let mut lowest_score = vec![vec![None::<usize>; col_count]; row_count];
    let mut visited = vec![vec![false; col_count]; row_count];

    lowest_score[end_row][end_col] = Some(0);
    visited[end_row][end_col] = true;

    // all example inputs have the end point in the upper right walled corner, so let's confirm that

    assert!(end_row == 1 && end_col == col_count - 2);
    assert!(maze[0][col_count - 1] == WALL);
    assert!(maze[0][col_count - 2] == WALL);
    assert!(maze[1][col_count - 1] == WALL);

    visited[0][col_count - 1] = true;
    visited[0][col_count - 2] = true;
    visited[1][col_count - 1] = true;

    // ok now all I have to do is figure out a maze solving algorithm

    0
}

pub fn d16p1(file_path: &str) -> usize {
    let maze = matrix::as_char_matrix(file_path);
    shortest_path(&maze)
}

pub fn d16p2(file_path: &str) -> usize {
    0
}

pub fn d16() {
    let file_path = "inputs/d16sample1.txt";
    let mut result = d16p1(file_path);
    println!("Result Day 16 Part 1: {}", result);
    result = d16p2(file_path);
    println!("Result Day 16 Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_shortest_small() {
        let maze = vec![
            vec!['#', '#', '#', '#'],
            vec!['#', '.', 'E', '#'],
            vec!['#', 'S', '.', '#'],
            vec!['#', '#', '#', '#'],
        ];
        // right 1, turn 1000, up 1
        assert_eq!(shortest_path(&maze), 1002);
    }
}
