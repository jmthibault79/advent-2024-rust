use crate::utils;
use crate::utils::matrix;
use crate::utils::plane;
use crate::utils::plane::Direction;
use crate::utils::plane::MovingObject;

const ROBOT: char = '@';
const WALL: char = '#';
const BOX: char = 'O';
const EMPTY: char = '.';

fn parse(file_path: &str) -> (Vec<Vec<char>>, Vec<Direction>) {
    let mut iter = utils::string_iter(file_path);

    let mut mat = vec![vec![]];
    while let Some(row) = iter.next() {
        if row.is_empty() {
            break;
        }
        mat.push(matrix::to_char_vec(row));
    }

    let mut moves = vec![];
    for line in iter {
        moves.extend(line.chars().map(Direction::from_char));
    }

    (mat, moves)
}

fn move_robot_1(
    mut mat: Vec<Vec<char>>,
    row: usize,
    col: usize,
    dir: Direction,
) -> (Vec<Vec<char>>, usize, usize) {
    let row_count = mat.len();
    let col_count = mat[0].len();

    let robot = MovingObject {
        row,
        col,
        dir,
        out_of_bounds: false,
    };

    match plane::obstacle_ahead(&mat, &vec![WALL,BOX], &robot) {
        Some(WALL) => /* can't move */ return (mat, row, col),
        Some(BOX) =>  return (mat, row, col),// TODO
        _ => {
            let new_robot = plane::move_one(row, col, row_count, col_count, dir);
            let (new_row, new_col) = (new_robot.row, new_robot.col);
            mat[row][col] = EMPTY;
            mat[new_row][new_col] = ROBOT;
            (mat, new_row, new_col)
        
        }
    }

}

pub fn d15p1(file_path: &str) -> usize {
    let (mut mat, moves) = parse(file_path);
    let (mut robot_row, mut robot_col) = plane::find_unique_element(&mat, ROBOT);
    matrix::pretty_print(&mat);
    (mat, robot_row, robot_col) = move_robot_1(mat, robot_row, robot_col, moves[0]);
    matrix::pretty_print(&mat);
    (mat, robot_row, robot_col) = move_robot_1(mat, robot_row, robot_col, moves[1]);
    matrix::pretty_print(&mat);
    (mat, robot_row, robot_col) = move_robot_1(mat, robot_row, robot_col, moves[2]);
    matrix::pretty_print(&mat);
    moves.len()
}

pub fn d15p2(file_path: &str) -> usize {
    0
}

pub fn d15() {
    let file_path = "inputs/d15sample1.txt";
    let mut result = d15p1(file_path);
    println!("Result Day 15 Part 1: {}", result);
    result = d15p2(file_path);
    println!("Result Day 15 Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(true);
    }
}
