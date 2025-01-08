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

fn shortest_path(maze: &Vec<Vec<char>>) -> usize {
    let (row_count, col_count) = matrix::dimensions(&maze);

    // all non-walls
    let mut spaces = vec![];
    for (row_num, row) in maze.iter().enumerate() {
        for (col_num, space) in row.iter().enumerate() {
            if *space != WALL {
                Direction::all().iter().for_each(|dir| {
                    spaces.push(MovingObject {
                        row: row_num,
                        col: col_num,
                        dir: dir.clone(),
                        out_of_bounds: false,
                    });
                });
            }
        }
    }

    // TODO: do I really need both of these?

    let mut to_visit: PriorityQueue<MovingObject, Reverse<usize>> = PriorityQueue::new();
    let mut score = HashMap::new();
    spaces.iter().for_each(|mo| {
        to_visit.push(mo.clone(), Reverse(usize::MAX));
        score.insert(mo.clone(), usize::MAX);
    });

    let (start_row, start_col) = plane::find_unique_element(&maze, START);
    let (end_row, end_col) = plane::find_unique_element(&maze, END);

    let start_node = MovingObject {
        row: start_row,
        col: start_col,
        dir: START_DIRECTION,
        out_of_bounds: false,
    };
    score.insert(start_node.clone(), 0);
    to_visit.change_priority(&start_node, Reverse(0));

    while let Some(curr) = to_visit.pop().map(|(mo, _)| mo) {
        // all possible neighbor nodes: move forward, turn right, turn left

        let mut neighbors = vec![];
        if plane::obstacle_ahead(&maze, &vec![WALL], &curr).is_none() {
            neighbors.push((
                plane::move_one(curr.row, curr.col, row_count, col_count, curr.dir),
                FORWARD_SCORE,
            ));
        }
        neighbors.push((
            MovingObject {
                row: curr.row,
                col: curr.col,
                dir: plane::turn_right_90_degrees(curr.dir),
                out_of_bounds: false,
            },
            TURN_SCORE,
        ));
        neighbors.push((
            MovingObject {
                row: curr.row,
                col: curr.col,
                dir: plane::turn_left_90_degrees(curr.dir),
                out_of_bounds: false,
            },
            TURN_SCORE,
        ));

        neighbors.iter().for_each(|(neighbor, additional)| {
            let new_score = score.get(&curr).unwrap() + additional;
            let existing_score = score.get(&neighbor).unwrap();
            if new_score < *existing_score {
                score.insert(neighbor.clone(), new_score);
                if to_visit.get(&neighbor).is_some() {
                    to_visit.change_priority(&neighbor, Reverse(new_score));
                }
            }
        });

        neighbors.clear();
    }

    let min_score = Direction::all()
        .iter()
        .flat_map(|dir| {
            score.get(&MovingObject {
                row: end_row,
                col: end_col,
                dir: dir.clone(),
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
