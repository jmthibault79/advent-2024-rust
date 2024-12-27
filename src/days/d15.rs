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

    let mut mat = vec![];
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

fn push_box_1(
    mut mat: Vec<Vec<char>>,
    row: usize,
    col: usize,
    dir: Direction,
) -> (Vec<Vec<char>>, usize, usize) {
    let row_count = mat.len();
    let col_count = mat[0].len();

    // println!("Pushing box at ({}, {}) in direction {:?} - {},{}", row, col, dir, row_count, col_count);

    let box_1 = plane::move_one(row, col, row_count, col_count, dir);

    let mut current_box = box_1.clone();
    while let Some(obstacle) = plane::obstacle_ahead(&mat, &vec![WALL, BOX], &current_box) {
        match obstacle {
            BOX => {
                current_box =
                    plane::move_one(current_box.row, current_box.col, row_count, col_count, dir)
            }
            _ => {
                // only other obstacle is WALL; hit a wall before an open space; can't move
                return (mat, row, col);
            }
        }
    }

    // if we reached here, we found open space and can therefore move the box(es)
    let open_space = plane::move_one(current_box.row, current_box.col, row_count, col_count, dir);

    // .@OO. -> ..@OO
    // set open space to BOX
    // set box_1 to ROBOT
    // set (old) robot to EMPTY
    // return box_1 as robot location

    mat[open_space.row][open_space.col] = BOX;
    mat[box_1.row][box_1.col] = ROBOT;
    mat[row][col] = EMPTY;
    (mat, box_1.row, box_1.col)
}

fn move_robot_1(
    mut mat: Vec<Vec<char>>,
    row: usize,
    col: usize,
    dir: Direction,
) -> (Vec<Vec<char>>, usize, usize) {
    let row_count = mat.len();
    let col_count = mat[0].len();

    // println!("Moving robot at ({}, {}) in direction {:?} - {},{}", row, col, dir, row_count, col_count);

    let robot = MovingObject {
        row,
        col,
        dir,
        out_of_bounds: false,
    };

    match plane::obstacle_ahead(&mat, &vec![WALL, BOX], &robot) {
        Some(WALL) =>
        // can't move
        {
            return (mat, row, col)
        }
        Some(BOX) => push_box_1(mat, row, col, dir),
        _ => {
            let new_robot = plane::move_one(row, col, row_count, col_count, dir);
            let (new_row, new_col) = (new_robot.row, new_robot.col);
            mat[row][col] = EMPTY;
            mat[new_row][new_col] = ROBOT;
            (mat, new_row, new_col)
        }
    }
}

fn gps_score(row: usize, col: usize) -> usize {
    100 * row + col
}

pub fn d15p1(file_path: &str) -> usize {
    let (mut mat, moves) = parse(file_path);
    let (mut robot_row, mut robot_col) = plane::find_unique_element(&mat, ROBOT);
    for move_1 in moves {
        (mat, robot_row, robot_col) = move_robot_1(mat, robot_row, robot_col, move_1);
    }
    plane::find_all_elements(&mat, BOX)
        .iter()
        .map(|&(row, col)| gps_score(row, col))
        .sum()
}

pub fn d15p2(file_path: &str) -> usize {
    0
}

pub fn d15() {
    //let file_path = "inputs/d15sample1.txt";
    // let file_path = "inputs/d15sample2.txt";
    let file_path = "inputs/d15.txt";
    let mut result = d15p1(file_path);
    println!("Result Day 15 Part 1: {}", result);
    result = d15p2(file_path);
    println!("Result Day 15 Part 2: {}", result);
}
