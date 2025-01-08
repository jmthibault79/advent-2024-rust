use crate::utils::matrix;
use crate::utils::plane;
use crate::utils::plane::Direction;
use crate::utils::plane::MovingObject;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;

const WALL: char = '#';
const START: char = 'S';
const END: char = 'E';
const START_DIRECTION: Direction = Direction::Right;

const FORWARD_SCORE: usize = 1;
const TURN_SCORE: usize = 1000;

fn init_structs(
    maze: &Vec<Vec<char>>,
) -> (
    PriorityQueue<MovingObject, Reverse<usize>>,
    HashMap<MovingObject, usize>,
) {
    let mut to_visit = PriorityQueue::new();
    let mut score = HashMap::new();

    // all non-walls
    for (row_num, row) in maze.iter().enumerate() {
        for (col_num, space) in row.iter().enumerate() {
            if *space != WALL {
                Direction::all().iter().for_each(|dir| {
                    let mo = MovingObject {
                        row: row_num,
                        col: col_num,
                        dir: *dir,
                        out_of_bounds: false,
                    };
                    to_visit.push(mo.clone(), Reverse(usize::MAX));
                    score.insert(mo.clone(), usize::MAX);
                });
            }
        }
    }

    (to_visit, score)
}

fn get_neighbors(maze: &Vec<Vec<char>>, curr: &MovingObject) -> Vec<(MovingObject, usize)> {
    // all possible neighbor nodes: move forward, turn right, turn left

    let mut neighbors = vec![
        (
            MovingObject {
                row: curr.row,
                col: curr.col,
                dir: plane::turn_right_90_degrees(curr.dir),
                out_of_bounds: false,
            },
            TURN_SCORE,
        ),
        (
            MovingObject {
                row: curr.row,
                col: curr.col,
                dir: plane::turn_left_90_degrees(curr.dir),
                out_of_bounds: false,
            },
            TURN_SCORE,
        ),
    ];

    if plane::obstacle_ahead(&maze, &vec![WALL], &curr).is_none() {
        let (row_count, col_count) = matrix::dimensions(&maze);

        neighbors.push((
            plane::move_one(curr.row, curr.col, row_count, col_count, curr.dir),
            FORWARD_SCORE,
        ));
    }

    neighbors
}

fn visit_neighbor(
    to_visit: &mut PriorityQueue<MovingObject, Reverse<usize>>,
    score: &mut HashMap<MovingObject, usize>,
    current: &MovingObject,
    neighbor: &MovingObject,
    additional_score: &usize,
) {
    let new_score = score.get(current).unwrap() + additional_score;
    let existing_score = score.get(&neighbor).unwrap();
    if new_score < *existing_score {
        score.insert(neighbor.clone(), new_score);
        if to_visit.get(neighbor).is_some() {
            to_visit.change_priority(neighbor, Reverse(new_score));
        }
    }
}

fn shortest_path(maze: &Vec<Vec<char>>) -> usize {
    let (mut to_visit, mut score) = init_structs(maze);

    let (start_row, start_col) = plane::find_unique_element(&maze, START);
    let (end_row, end_col) = plane::find_unique_element(&maze, END);

    let start_node = MovingObject {
        row: start_row,
        col: start_col,
        dir: START_DIRECTION,
        out_of_bounds: false,
    };
    to_visit.change_priority(&start_node, Reverse(0));
    score.insert(start_node, 0);

    while let Some(current) = to_visit.pop().map(|(mo, _)| mo) {
        get_neighbors(&maze, &current)
            .iter()
            .for_each(|(neighbor, additional_score)| {
                visit_neighbor(
                    &mut to_visit,
                    &mut score,
                    &current,
                    neighbor,
                    additional_score,
                )
            });
    }

    let min_score = Direction::all()
        .iter()
        .flat_map(|dir| {
            score.get(&MovingObject {
                row: end_row,
                col: end_col,
                dir: *dir,
                out_of_bounds: false,
            })
        })
        .min()
        .unwrap();
    *min_score
}

pub fn d16p1(file_path: &str) -> usize {
    let maze = matrix::as_char_matrix(file_path);
    shortest_path(&maze)
}

pub fn d16p2(file_path: &str) -> usize {
    0
}

pub fn d16() {
    // let file_path = "inputs/d16sample1.txt";
    // let file_path = "inputs/d16sample2.txt";
    let file_path = "inputs/d16.txt";
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
        let mut maze = vec![
            vec!['#', '#', '#', '#'],
            vec!['#', '.', 'E', '#'],
            vec!['#', 'S', '.', '#'],
            vec!['#', '#', '#', '#'],
        ];
        // right 1, turn 1000, up 1
        assert_eq!(shortest_path(&maze), 1002);

        maze = vec![
            vec!['#', '#', '#', '#'],
            vec!['#', '.', 'E', '#'],
            vec!['#', 'S', '#', '#'],
            vec!['#', '#', '#', '#'],
        ];
        // turn 1000, up 1, turn 1000, right 1
        assert_eq!(shortest_path(&maze), 2002);
    }
}
