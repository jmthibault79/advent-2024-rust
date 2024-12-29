use crate::utils;
use crate::utils::distinct;
use crate::utils::matrix;
use crate::utils::plane;
use crate::utils::plane::Direction;
use crate::utils::plane::MovingObject;

const ROBOT: char = '@';
const WALL: char = '#';
const BOX: char = 'O';
const EMPTY: char = '.';
const L_BOX: char = '[';
const R_BOX: char = ']';

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
    robot_row: usize,
    robot_col: usize,
    dir: Direction,
) -> (Vec<Vec<char>>, usize, usize) {
    let row_count = mat.len();
    let col_count = mat[0].len();

    // println!("Pushing box at ({}, {}) in direction {:?} - {},{}", row, col, dir, row_count, col_count);

    let box_1 = plane::move_one(robot_row, robot_col, row_count, col_count, dir);
    let mut box_spaces = vec![(box_1.row, box_1.col)];

    let mut current_box = box_1.clone();
    while let Some(obstacle) =
        plane::obstacle_ahead(&mat, &vec![WALL, BOX, L_BOX, R_BOX], &current_box)
    {
        match obstacle {
            BOX | L_BOX | R_BOX => {
                current_box =
                    plane::move_one(current_box.row, current_box.col, row_count, col_count, dir);
                box_spaces.push((current_box.row, current_box.col));
            }
            _ => {
                // only other obstacle is WALL; hit a wall before an open space; can't move
                return (mat, robot_row, robot_col);
            }
        }
    }

    // if we reached here, we found open space and can therefore move the box(es)
    let open_space = plane::move_one(current_box.row, current_box.col, row_count, col_count, dir);

    // .@OO. -> ..@OO
    // ..[]@. -> .[]@..

    // from end of box_spaces to start:
    // move box to next space, backfill with EMPTY

    box_spaces.push((open_space.row, open_space.col));

    for i in (0..box_spaces.len() - 1).rev() {
        let dest = box_spaces[i + 1];
        let src = box_spaces[i];
        mat[dest.0][dest.1] = mat[src.0][src.1];
    }

    mat[box_1.row][box_1.col] = ROBOT;
    mat[robot_row][robot_col] = EMPTY;

    (mat, box_1.row, box_1.col)
}

fn complete_box_from_half(
    box_half: &MovingObject,
    which_box: char,
    dir: Direction,
) -> Vec<MovingObject> {
    let (box_l_col, box_r_col) = match which_box {
        L_BOX => (box_half.col, box_half.col + 1),
        R_BOX => (box_half.col - 1, box_half.col),
        _ => panic!("Unexpected box type: {}", which_box),
    };
    vec![
        MovingObject {
            row: box_half.row,
            col: box_l_col,
            dir,
            out_of_bounds: false,
        },
        MovingObject {
            row: box_half.row,
            col: box_r_col,
            dir,
            out_of_bounds: false,
        },
    ]
}

fn push_p2_ud_box_1(
    mut mat: Vec<Vec<char>>,
    robot_row: usize,
    robot_col: usize,
    which_box: char,
    dir: Direction,
) -> (Vec<Vec<char>>, usize, usize) {
    let row_count = mat.len();
    let col_count = mat[0].len();

    let box_1_half = plane::move_one(robot_row, robot_col, row_count, col_count, dir);
    let box_1 = complete_box_from_half(&box_1_half, which_box, dir);

    // println!(
    //     "Pushing box at ({}, {}-{}) in direction {:?} - total matrix size {},{}",
    //     box_1_half.row, box_1[0].col, box_1[1].col, dir, row_count, col_count
    // );

    // in the vertical direction, it's possible for a box to push more than one other box
    // so we need to account for pushes by all segments
    let mut current_box_row = box_1.clone();
    let mut box_rows = vec![current_box_row.clone()];

    let mut obstacles_ahead: Vec<char>;

    loop {
        obstacles_ahead = current_box_row
            .iter()
            .flat_map(|box_half| {
                plane::obstacle_ahead(&mat, &vec![WALL, BOX, L_BOX, R_BOX], box_half)
            })
            .collect();

        if obstacles_ahead.contains(&WALL) {
            // hit a wall before an open space; can't move

            // println!();
            // println!("Hit wall. Current box row: {:?}", current_box_row);

            return (mat, robot_row, robot_col);
        } else if obstacles_ahead.is_empty() {
            // can move into empty space

            // println!();
            // println!(
            //     "Moving into open space. Current box row: {:?}",
            //     current_box_row
            // );

            // from end of box_rows to start:
            // move src row to dest row, then set src row to EMPTY
            // don't need to clear dest row, because it's always EMPTY - either naturally or as the result of the previous iteration

            for i in (0..box_rows.len()).rev() {
                let src_row = &box_rows[i];

                for src_box_half in src_row {
                    let dest_box_half = plane::move_one(
                        src_box_half.row,
                        src_box_half.col,
                        row_count,
                        col_count,
                        dir,
                    );

                    mat[dest_box_half.row][dest_box_half.col] =
                        mat[src_box_half.row][src_box_half.col];
                    mat[src_box_half.row][src_box_half.col] = EMPTY;
                }
            }

            mat[box_1_half.row][box_1_half.col] = ROBOT;
            mat[robot_row][robot_col] = EMPTY;

            return (mat, box_1_half.row, box_1_half.col);
        } else {
            // hit a box; need to push it

            let box_row: Vec<MovingObject> = distinct(
                current_box_row
                    .iter()
                    .map(|box_half| {
                        plane::move_one(box_half.row, box_half.col, row_count, col_count, dir)
                    })
                    // only want boxes here.  expty spaces don't push
                    .flat_map(|next_row_box_half_maybe| {
                        let c = mat[next_row_box_half_maybe.row][next_row_box_half_maybe.col];
                        match c {
                            L_BOX | R_BOX => {
                                complete_box_from_half(&next_row_box_half_maybe, c, dir)
                            }
                            _ => vec![],
                        }
                    })
                    .collect(),
            );

            // println!();
            // println!("Hit box(es).  current row: {:?} ", current_box_row);
            // println!("next row: {:?}", box_row);

            box_rows.push(box_row.clone());
            current_box_row = box_row;
        }
    }
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

    match (
        plane::obstacle_ahead(&mat, &vec![WALL, BOX, L_BOX, R_BOX], &robot),
        dir,
    ) {
        (Some(WALL), _) =>
        // can't move
        {
            return (mat, row, col)
        }
        (Some(BOX), _) => push_box_1(mat, row, col, dir),
        // can use P1 for Left and Right because pushing at broundaries doesn't come into play
        (Some(L_BOX | R_BOX), Direction::Left | Direction::Right) => push_box_1(mat, row, col, dir),
        (Some(which_box @ L_BOX | which_box @ R_BOX), Direction::Up | Direction::Down) => {
            push_p2_ud_box_1(mat, row, col, which_box, dir)
        }
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

fn score(mat: &Vec<Vec<char>>, target: char) -> usize {
    plane::find_all_elements(&mat, target)
        .iter()
        .map(|&(row, col)| gps_score(row, col))
        .sum()
}

pub fn d15p1(file_path: &str) -> usize {
    let (mut mat, moves) = parse(file_path);
    let (mut robot_row, mut robot_col) = plane::find_unique_element(&mat, ROBOT);
    for move_1 in moves {
        (mat, robot_row, robot_col) = move_robot_1(mat, robot_row, robot_col, move_1);
    }
    score(&mat, BOX)
}

fn expand(mat: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = Vec::with_capacity(mat.len());
    for row in mat {
        result.push(
            row.iter()
                .flat_map(|c| match *c {
                    WALL => vec![WALL, WALL],
                    BOX => vec![L_BOX, R_BOX],
                    ROBOT => vec![ROBOT, EMPTY],
                    EMPTY => vec![EMPTY, EMPTY],
                    _ => panic!("Unexpected character in matrix: {}", c),
                })
                .collect(),
        );
    }

    result
}

pub fn d15p2(file_path: &str) -> usize {
    let (mut mat, moves) = parse(file_path);
    mat = expand(&mat);
    let (mut robot_row, mut robot_col) = plane::find_unique_element(&mat, ROBOT);
    for move_1 in moves {
        (mat, robot_row, robot_col) = move_robot_1(mat, robot_row, robot_col, move_1);
    }
    score(&mat, L_BOX)
}

pub fn d15() {
    // let file_path = "inputs/d15sample1.txt";
    // let file_path = "inputs/d15sample2.txt";
    let file_path = "inputs/d15.txt";
    let mut result = d15p1(file_path);
    println!("Result Day 15 Part 1: {}", result);
    result = d15p2(file_path);
    println!("Result Day 15 Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use std::vec;

    use matrix::pretty_print;

    use super::*;

    #[test]
    fn test_push_box_1() {
        // no change because we hit the wall (p1)
        let mut mat = vec![vec![ROBOT, BOX, WALL]];
        let mut expected = vec![vec![ROBOT, BOX, WALL]];
        assert_eq!(push_box_1(mat, 0, 0, Direction::Right), (expected, 0, 0));

        // no change because we hit the wall (p2)
        mat = vec![vec![WALL, L_BOX, R_BOX, ROBOT]];
        expected = vec![vec![WALL, L_BOX, R_BOX, ROBOT]];
        assert_eq!(push_box_1(mat, 0, 3, Direction::Left), (expected, 0, 3));

        // move p1 box left
        mat = vec![vec![EMPTY, BOX, ROBOT]];
        expected = vec![vec![BOX, ROBOT, EMPTY]];
        assert_eq!(push_box_1(mat, 0, 2, Direction::Left), (expected, 0, 1));

        // move p2 box right
        mat = vec![vec![ROBOT, L_BOX, R_BOX, EMPTY, EMPTY]];
        expected = vec![vec![EMPTY, ROBOT, L_BOX, R_BOX, EMPTY]];
        assert_eq!(push_box_1(mat, 0, 0, Direction::Right), (expected, 0, 1));
    }

    #[test]
    fn test_push_p2_ud_box_1() {
        // no change because we hit the wall
        let mut mat = vec![vec![ROBOT, EMPTY], vec![L_BOX, R_BOX], vec![WALL, WALL]];
        let mut expected = mat.clone();
        assert_eq!(
            push_p2_ud_box_1(mat, 0, 0, L_BOX, Direction::Down),
            (expected, 0, 0)
        );

        // no change because we hit the wall
        mat = vec![vec![WALL, WALL], vec![L_BOX, R_BOX], vec![EMPTY, ROBOT]];
        expected = mat.clone();
        assert_eq!(
            push_p2_ud_box_1(mat, 2, 1, R_BOX, Direction::Up),
            (expected, 2, 1)
        );

        // move 1
        mat = vec![vec![ROBOT, EMPTY], vec![L_BOX, R_BOX], vec![EMPTY, EMPTY]];
        let mut expected = vec![vec![EMPTY, EMPTY], vec![ROBOT, EMPTY], vec![L_BOX, R_BOX]];
        assert_eq!(
            push_p2_ud_box_1(mat, 0, 0, L_BOX, Direction::Down),
            (expected, 1, 0)
        );

        // move 1
        mat = vec![vec![EMPTY, EMPTY], vec![L_BOX, R_BOX], vec![EMPTY, ROBOT]];
        expected = vec![vec![L_BOX, R_BOX], vec![EMPTY, ROBOT], vec![EMPTY, EMPTY]];
        assert_eq!(
            push_p2_ud_box_1(mat, 2, 1, R_BOX, Direction::Up),
            (expected, 1, 1)
        );

        // more complex moves

        // wall a few steps down

        mat = vec![
            vec![EMPTY, ROBOT, EMPTY, EMPTY],
            vec![EMPTY, L_BOX, R_BOX, EMPTY],
            vec![L_BOX, R_BOX, L_BOX, R_BOX],
            vec![WALL, EMPTY, EMPTY, EMPTY],
        ];
        expected = mat.clone();
        assert_eq!(
            push_p2_ud_box_1(mat, 0, 1, L_BOX, Direction::Down),
            (expected, 0, 1)
        );

        // push a series of unaligned boxes

        mat = vec![
            vec![EMPTY, ROBOT, EMPTY, EMPTY],
            vec![EMPTY, L_BOX, R_BOX, EMPTY],
            vec![L_BOX, R_BOX, L_BOX, R_BOX],
            vec![EMPTY, L_BOX, R_BOX, EMPTY],
            vec![WALL, EMPTY, EMPTY, EMPTY],
        ];
        expected = vec![
            vec![EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, ROBOT, EMPTY, EMPTY],
            vec![EMPTY, L_BOX, R_BOX, EMPTY],
            vec![L_BOX, R_BOX, L_BOX, R_BOX],
            vec![WALL, L_BOX, R_BOX, EMPTY],
        ];
        let result = push_p2_ud_box_1(mat.clone(), 0, 1, L_BOX, Direction::Down);

        // println!("input");
        // pretty_print(&mat);

        // println!("expected");
        // pretty_print(&expected);

        // println!("result");
        // pretty_print(&result.0);

        assert_eq!(result, (expected, 1, 1));
    }
}
