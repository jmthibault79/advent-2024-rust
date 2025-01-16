use crate::utils::matrix;
use crate::utils::plane::{self, Direction, MovingObject};
use multimap::MultiMap;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

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
    previous_nodes: &mut MultiMap<MovingObject, MovingObject>,
    current: &MovingObject,
    neighbor: &MovingObject,
    additional_score: &usize,
) {
    let new_score = score.get(current).unwrap() + additional_score;
    let existing_score = score.get(&neighbor).unwrap();
    if new_score <= *existing_score {
        score.insert(neighbor.clone(), new_score);
        previous_nodes.insert(neighbor.clone(), current.clone());
        if to_visit.get(neighbor).is_some() {
            to_visit.change_priority(neighbor, Reverse(new_score));
        }
    }
}

fn score_maze(
    maze: &Vec<Vec<char>>,
) -> (
    HashMap<MovingObject, usize>,
    MultiMap<MovingObject, MovingObject>,
) {
    let (mut to_visit, mut score) = init_structs(maze);

    let (start_row, start_col) = plane::find_unique_element(&maze, START);

    let start_node = MovingObject {
        row: start_row,
        col: start_col,
        dir: START_DIRECTION,
        out_of_bounds: false,
    };
    to_visit.change_priority(&start_node, Reverse(0));
    score.insert(start_node, 0);

    let mut previous_nodes: MultiMap<MovingObject, MovingObject> = MultiMap::new();

    while let Some(current) = to_visit.pop().map(|(mo, _)| mo) {
        get_neighbors(&maze, &current)
            .iter()
            .for_each(|(neighbor, additional_score)| {
                visit_neighbor(
                    &mut to_visit,
                    &mut score,
                    &mut previous_nodes,
                    &current,
                    neighbor,
                    additional_score,
                )
            });
    }

    (score, previous_nodes)
}

fn min_score(maze: &Vec<Vec<char>>, score: &HashMap<MovingObject, usize>) -> usize {
    let (end_row, end_col) = plane::find_unique_element(&maze, END);

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

fn shortest_path(maze: &Vec<Vec<char>>) -> usize {
    let (score, _) = score_maze(maze);
    min_score(maze, &score)
}

pub fn d16p1(file_path: &str) -> usize {
    let maze = matrix::as_char_matrix(file_path);
    shortest_path(&maze)
}

fn shortest_path_for_node(
    prev_nodes: &MultiMap<MovingObject, MovingObject>,
    current: &MovingObject,
    nodes_so_far: &HashSet<(usize, usize)>,
    start_row: usize,
    start_col: usize,
) -> HashSet<(usize, usize)> {
    if (current.row, current.col) == (start_row, start_col) {
        return nodes_so_far.clone();
    }

    let Some(next_nodes) = prev_nodes.get_vec(&current) else {
        return HashSet::new();
    };

    next_nodes
        .iter()
        .flat_map(|next_node| {
            let mut new_nodes_so_far = nodes_so_far.clone();
            new_nodes_so_far.insert((next_node.row, next_node.col));

            shortest_path_for_node(
                prev_nodes,
                next_node,
                &new_nodes_so_far,
                start_row,
                start_col,
            )
        })
        .collect()
}

fn tiles_along_shortest_paths(maze: &Vec<Vec<char>>) -> usize {
    let (score, prev_nodes) = score_maze(maze);

    let (start_row, start_col) = plane::find_unique_element(&maze, START);
    let (end_row, end_col) = plane::find_unique_element(&maze, END);

    let shortest_paths: HashSet<(usize, usize)> = Direction::all()
        .iter()
        .map(|dir| MovingObject {
            row: end_row,
            col: end_col,
            dir: *dir,
            out_of_bounds: false,
        })
        .filter(|mo| score.get(mo).unwrap() == &min_score(maze, &score))
        .flat_map(|node| {
            let mut nodes_so_far: HashSet<(usize, usize)> = HashSet::new();
            nodes_so_far.insert((node.row, node.col));
            shortest_path_for_node(&prev_nodes, &node, &nodes_so_far, start_row, start_col)
        })
        .collect();

    shortest_paths.len()
}

pub fn d16p2(file_path: &str) -> usize {
    let maze = matrix::as_char_matrix(file_path);
    tiles_along_shortest_paths(&maze)
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

    #[test]
    fn test_tiles_along_shortest_small() {
        let mut maze = vec![
            vec!['#', '#', '#', '#'],
            vec!['#', '.', 'E', '#'],
            vec!['#', 'S', '.', '#'],
            vec!['#', '#', '#', '#'],
        ];
        // right 1, turn 1000, up 1
        assert_eq!(tiles_along_shortest_paths(&maze), 3);
    }
}
