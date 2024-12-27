// handles 2D movement in a plane:
// row 0 is on the top and 0,0 is top left

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_char(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct MovingObject {
    pub row: usize,
    pub col: usize,
    pub dir: Direction,
    pub out_of_bounds: bool,
}

// result is dest_row, dest_col, Some(direction if out of bounds)
pub fn move_one(
    row: usize,
    col: usize,
    row_count: usize,
    col_count: usize,
    dir: Direction,
) -> MovingObject {
    let (out_of_bounds, new_row, new_col) = match (dir, row, col) {
        // out of bounds
        (Direction::Up, r, _) if r == 0 => (true, row, col),
        (Direction::Down, r, _) if r == row_count - 1 => (true, row, col),
        (Direction::Left, _, c) if c == 0 => (true, row, col),
        (Direction::Right, _, c) if c == col_count - 1 => (true, row, col),
        // normal movement
        (Direction::Up, _, _) => (false, row - 1, col),
        (Direction::Down, _, _) => (false, row + 1, col),
        (Direction::Left, _, _) => (false, row, col - 1),
        (Direction::Right, _, _) => (false, row, col + 1),
    };

    MovingObject {
        row: new_row,
        col: new_col,
        dir,
        out_of_bounds,
    }
}

pub fn obstacle_ahead(
    plane: &Vec<Vec<char>>,
    obstacles: &Vec<char>,
    me: &MovingObject,
) -> Option<char> {
    let row_count = plane.len();
    let col_count = plane[0].len();
    let MovingObject {
        row,
        col,
        dir: _,
        out_of_bounds,
    } = move_one(me.row, me.col, row_count, col_count, me.dir);
    let char_ahead = plane[row][col];
    if !out_of_bounds && obstacles.contains(&char_ahead) {
        Some(char_ahead)
    } else {
        None
    }
}

fn move_forward_or_turn_right(
    plane: &Vec<Vec<char>>,
    obstacles: &Vec<char>,
    start: &MovingObject,
) -> MovingObject {
    if obstacle_ahead(plane, &obstacles, &start).is_some() {
        let new_dir = match start.dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
        move_forward_or_turn_right(
            plane,
            obstacles,
            &MovingObject {
                row: start.row,
                col: start.col,
                dir: new_dir,
                out_of_bounds: false,
            },
        )
    } else {
        move_one(start.row, start.col, plane.len(), plane[0].len(), start.dir)
    }
}

// generate the path to the exit, turning right when an obstacle is hit
// return None if a loop is detected
pub fn path_to_exit_turning_right(
    plane: &Vec<Vec<char>>,
    obstacles: &Vec<char>,
    start: &MovingObject,
) -> Option<Vec<MovingObject>> {
    let mut path = vec![start.clone()];
    let mut current = start.clone();

    loop {
        current = move_forward_or_turn_right(plane, obstacles, &current);
        if current.out_of_bounds {
            break;
        } else {
            // check for a loop if the length is above a threshold
            if path.len() > 5000 && path.contains(&current) {
                return None;
            } else {
                path.push(current.clone());
            }
        }
    }
    Some(path)
}

pub fn unique_spaces(path: &Vec<MovingObject>) -> Vec<(usize, usize)> {
    let mut unique = vec![];
    for mo in path {
        if !unique.contains(&(mo.row, mo.col)) {
            unique.push((mo.row, mo.col));
        }
    }
    unique
}

// find some character in the plane
// expectation: only one of this character exists
pub fn find_unique_element(plane: &Vec<Vec<char>>, to_find: char) -> (usize, usize) {
    for (row, row_vec) in plane.iter().enumerate() {
        for (col, char_at_col) in row_vec.iter().enumerate() {
            if *char_at_col == to_find {
                return (row, col);
            }
        }
    }
    panic!("{} not found", to_find);
}

pub fn find_all_elements(plane: &Vec<Vec<char>>, to_find: char) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for (row, row_vec) in plane.iter().enumerate() {
        for (col, char_at_col) in row_vec.iter().enumerate() {
            if *char_at_col == to_find {
                result.push((row, col));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_one_test() {
        // 2x2 map

        // move one, valid
        assert_eq!(
            move_one(0, 0, 2, 2, Direction::Right),
            MovingObject {
                row: 0,
                col: 1,
                dir: Direction::Right,
                out_of_bounds: false
            }
        );
        assert_eq!(
            move_one(0, 0, 2, 2, Direction::Down),
            MovingObject {
                row: 1,
                col: 0,
                dir: Direction::Down,

                out_of_bounds: false
            }
        );
        assert_eq!(
            move_one(1, 1, 2, 2, Direction::Up),
            MovingObject {
                row: 0,
                col: 1,
                dir: Direction::Up,
                out_of_bounds: false
            }
        );
        assert_eq!(
            move_one(1, 1, 2, 2, Direction::Left),
            MovingObject {
                row: 1,
                col: 0,
                dir: Direction::Left,
                out_of_bounds: false
            }
        );

        // move one, invalid
        assert_eq!(
            move_one(0, 0, 2, 2, Direction::Left),
            MovingObject {
                row: 0,
                col: 0,
                dir: Direction::Left,
                out_of_bounds: true
            }
        );
        assert_eq!(
            move_one(0, 0, 2, 2, Direction::Up),
            MovingObject {
                row: 0,
                col: 0,
                dir: Direction::Up,
                out_of_bounds: true
            }
        );
        assert_eq!(
            move_one(1, 1, 2, 2, Direction::Down),
            MovingObject {
                row: 1,
                col: 1,
                dir: Direction::Down,
                out_of_bounds: true
            }
        );
        assert_eq!(
            move_one(1, 1, 2, 2, Direction::Right),
            MovingObject {
                row: 1,
                col: 1,
                dir: Direction::Right,
                out_of_bounds: true
            }
        );
    }

    #[test]
    fn obs_ahead_test() {
        let obstacles = vec!['x'];

        // 2x2 map
        let plane = vec![vec!['.', '.'], vec!['.', '.']];

        assert!(obstacle_ahead(
            &plane,
            &obstacles,
            &MovingObject {
                row: 0,
                col: 0,
                dir: Direction::Right,
                out_of_bounds: false
            }
        )
        .is_none());
        assert!(obstacle_ahead(
            &plane,
            &obstacles,
            &MovingObject {
                row: 0,
                col: 0,
                dir: Direction::Down,
                out_of_bounds: false
            }
        )
        .is_none());

        // exits are not obstacles
        assert!(obstacle_ahead(
            &plane,
            &obstacles,
            &MovingObject {
                row: 0,
                col: 0,
                dir: Direction::Left,
                out_of_bounds: false
            }
        )
        .is_none());
        assert!(obstacle_ahead(
            &plane,
            &obstacles,
            &MovingObject {
                row: 0,
                col: 0,
                dir: Direction::Up,
                out_of_bounds: false
            }
        )
        .is_none());

        let plane = vec![vec!['.', 'x'], vec!['.', '.']];
        assert_eq!(
            obstacle_ahead(
                &plane,
                &obstacles,
                &MovingObject {
                    row: 0,
                    col: 0,
                    dir: Direction::Right,
                    out_of_bounds: false
                }
            ),
            Some('x')
        );
        assert!(obstacle_ahead(
            &plane,
            &obstacles,
            &MovingObject {
                row: 0,
                col: 0,
                dir: Direction::Down,
                out_of_bounds: false
            }
        )
        .is_none());
        assert_eq!(
            obstacle_ahead(
                &plane,
                &obstacles,
                &MovingObject {
                    row: 1,
                    col: 1,
                    dir: Direction::Up,
                    out_of_bounds: false
                }
            ),
            Some('x')
        );
        assert!(obstacle_ahead(
            &plane,
            &obstacles,
            &MovingObject {
                row: 1,
                col: 1,
                dir: Direction::Left,
                out_of_bounds: false
            }
        )
        .is_none());
    }

    #[test]
    fn path_right_test() {
        let obstacles = vec!['x'];

        let mut plane = vec![vec!['.', '.'], vec!['.', '.']];
        let mut start = MovingObject {
            row: 0,
            col: 0,
            dir: Direction::Right,
            out_of_bounds: false,
        };
        let mut expected_path = vec![
            MovingObject {
                row: 0,
                col: 0,
                dir: Direction::Right,
                out_of_bounds: false,
            },
            MovingObject {
                row: 0,
                col: 1,
                dir: Direction::Right,
                out_of_bounds: false,
            },
        ];

        assert_eq!(
            path_to_exit_turning_right(&plane, &obstacles, &start),
            Some(expected_path)
        );

        start = MovingObject {
            row: 0,
            col: 0,
            dir: Direction::Down,
            out_of_bounds: false,
        };
        expected_path = vec![
            MovingObject {
                row: 0,
                col: 0,
                dir: Direction::Down,
                out_of_bounds: false,
            },
            MovingObject {
                row: 1,
                col: 0,
                dir: Direction::Down,
                out_of_bounds: false,
            },
        ];

        assert_eq!(
            path_to_exit_turning_right(&plane, &obstacles, &start),
            Some(expected_path)
        );

        plane = vec![
            vec!['x', '.', '.'],
            vec!['.', '.', 'x'],
            vec!['.', '.', '.'],
        ];
        start = MovingObject {
            row: 2,
            col: 0,
            dir: Direction::Up,
            out_of_bounds: false,
        };
        expected_path = vec![
            MovingObject {
                row: 2,
                col: 0,
                dir: Direction::Up,
                out_of_bounds: false,
            },
            MovingObject {
                row: 1,
                col: 0,
                dir: Direction::Up,
                out_of_bounds: false,
            },
            MovingObject {
                row: 1,
                col: 1,
                dir: Direction::Right,
                out_of_bounds: false,
            },
            MovingObject {
                row: 2,
                col: 1,
                dir: Direction::Down,
                out_of_bounds: false,
            },
        ];

        assert_eq!(
            path_to_exit_turning_right(&plane, &obstacles, &start),
            Some(expected_path)
        );
    }

    #[test]
    fn unique_test() {
        let path = vec![
            MovingObject {
                row: 0,
                col: 0,
                dir: Direction::Right,
                out_of_bounds: false,
            },
            MovingObject {
                row: 0,
                col: 1,
                dir: Direction::Right,
                out_of_bounds: false,
            },
            MovingObject {
                row: 0,
                col: 1,
                dir: Direction::Left,
                out_of_bounds: true,
            },
        ];

        let expected = vec![(0, 0), (0, 1)];
        assert_eq!(unique_spaces(&path), expected);
    }

    #[test]
    fn path_right_loop_test() {
        let obstacles = vec!['x'];

        let plane = vec![
            vec!['.', 'x', '.', '.'],
            vec!['.', '.', '.', 'x'],
            vec!['x', '.', '.', '.'],
            vec!['.', '.', 'x', '.'],
        ];
        let start = MovingObject {
            row: 1,
            col: 0,
            dir: Direction::Right,
            out_of_bounds: false,
        };

        assert_eq!(path_to_exit_turning_right(&plane, &obstacles, &start), None);
    }
}
