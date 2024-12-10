// handles 2D movement in a plane:
// row 0 is on the top and 0,0 is top left

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Debug)]
pub struct MovingObject {
    pub row: usize,
    pub col: usize,
    pub dir: Direction,
    pub out_of_bounds: bool,
}

impl Clone for MovingObject {
    fn clone(&self) -> MovingObject {
        MovingObject {
            row: self.row,
            col: self.col,
            dir: self.dir.clone(),
            out_of_bounds: self.out_of_bounds,
        }
    }
}

// result is dest_row, dest_col, Some(direction if out of bounds)
fn move_one(
    row: usize,
    col: usize,
    row_count: usize,
    col_count: usize,
    direction: &Direction,
) -> MovingObject {
    match (direction, row, col) {
        // handle out of bounds first
        (Direction::Up, r, c) if r == 0 => MovingObject {
            row: r,
            col: c,
            dir: Direction::Up,
            out_of_bounds: true,
        },
        (Direction::Down, r, c) if r == row_count - 1 => MovingObject {
            row: r,
            col: c,
            dir: Direction::Down,
            out_of_bounds: true,
        },
        (Direction::Left, r, c) if c == 0 => MovingObject {
            row: r,
            col: c,
            dir: Direction::Left,
            out_of_bounds: true,
        },
        (Direction::Right, r, c) if c == col_count - 1 => MovingObject {
            row: r,
            col: c,
            dir: Direction::Right,
            out_of_bounds: true,
        },
        // handle normal movement
        (Direction::Up, r, c) => MovingObject {
            row: r - 1,
            col: c,
            dir: Direction::Up,
            out_of_bounds: false,
        },
        (Direction::Down, r, c) => MovingObject {
            row: r + 1,
            col: c,
            dir: Direction::Down,
            out_of_bounds: false,
        },
        (Direction::Left, r, c) => MovingObject {
            row: r,
            col: c - 1,
            dir: Direction::Left,
            out_of_bounds: false,
        },
        (Direction::Right, r, c) => MovingObject {
            row: r,
            col: c + 1,
            dir: Direction::Right,
            out_of_bounds: false,
        },
    }
}

fn obstacle_ahead(plane: &Vec<Vec<char>>, obstacles: &Vec<char>, me: &MovingObject) -> bool {
    let MovingObject {
        row,
        col,
        dir: _,
        out_of_bounds,
    } = move_one(me.row, me.col, plane.len(), plane[0].len(), &me.dir);
    !out_of_bounds && obstacles.contains(&plane[row][col])
}

fn move_forward_or_turn_right(
    plane: &Vec<Vec<char>>,
    obstacles: &Vec<char>,
    start: &MovingObject,
) -> MovingObject {
    if obstacle_ahead(plane, &obstacles, &start) {
        // turn right
        let right_90_degrees: Direction = match start.dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
        move_one(
            start.row,
            start.col,
            plane.len(),
            plane[0].len(),
            &right_90_degrees,
        )
    } else {
        // go straight
        move_one(
            start.row,
            start.col,
            plane.len(),
            plane[0].len(),
            &start.dir,
        )
    }
}

// generate the path to the exit, turning right when an obstacle is hit
pub fn path_to_exit_turning_right(
    plane: &Vec<Vec<char>>,
    obstacles: &Vec<char>,
    start: &MovingObject,
) -> Vec<MovingObject> {
    let mut path = vec![start.clone()];
    let mut current = start.clone();

    loop {
        current = move_forward_or_turn_right(plane, obstacles, &current);
        if current.out_of_bounds {
            break;
        } else {
            path.push(current.clone());
        }
    }
    path
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_one_test() {
        // 2x2 map

        // move one, valid
        assert_eq!(
            move_one(0, 0, 2, 2, &Direction::Right),
            MovingObject {
                row: 0,
                col: 1,
                dir: Direction::Right,
                out_of_bounds: false
            }
        );
        assert_eq!(
            move_one(0, 0, 2, 2, &Direction::Down),
            MovingObject {
                row: 1,
                col: 0,
                dir: Direction::Down,

                out_of_bounds: false
            }
        );
        assert_eq!(
            move_one(1, 1, 2, 2, &Direction::Up),
            MovingObject {
                row: 0,
                col: 1,
                dir: Direction::Up,
                out_of_bounds: false
            }
        );
        assert_eq!(
            move_one(1, 1, 2, 2, &Direction::Left),
            MovingObject {
                row: 1,
                col: 0,
                dir: Direction::Left,
                out_of_bounds: false
            }
        );

        // move one, invalid
        assert_eq!(
            move_one(0, 0, 2, 2, &Direction::Left),
            MovingObject {
                row: 0,
                col: 0,
                dir: Direction::Left,
                out_of_bounds: true
            }
        );
        assert_eq!(
            move_one(0, 0, 2, 2, &Direction::Up),
            MovingObject {
                row: 0,
                col: 0,
                dir: Direction::Up,
                out_of_bounds: true
            }
        );
        assert_eq!(
            move_one(1, 1, 2, 2, &Direction::Down),
            MovingObject {
                row: 1,
                col: 1,
                dir: Direction::Down,
                out_of_bounds: true
            }
        );
        assert_eq!(
            move_one(1, 1, 2, 2, &Direction::Right),
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
            false
        );
        assert_eq!(
            obstacle_ahead(
                &plane,
                &obstacles,
                &MovingObject {
                    row: 0,
                    col: 0,
                    dir: Direction::Down,
                    out_of_bounds: false
                }
            ),
            false
        );

        // exits are not obstacles
        assert_eq!(
            obstacle_ahead(
                &plane,
                &obstacles,
                &MovingObject {
                    row: 0,
                    col: 0,
                    dir: Direction::Left,
                    out_of_bounds: false
                }
            ),
            false
        );
        assert_eq!(
            obstacle_ahead(
                &plane,
                &obstacles,
                &MovingObject {
                    row: 0,
                    col: 0,
                    dir: Direction::Up,
                    out_of_bounds: false
                }
            ),
            false
        );

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
            true
        );
        assert_eq!(
            obstacle_ahead(
                &plane,
                &obstacles,
                &MovingObject {
                    row: 0,
                    col: 0,
                    dir: Direction::Down,
                    out_of_bounds: false
                }
            ),
            false
        );
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
            true
        );
        assert_eq!(
            obstacle_ahead(
                &plane,
                &obstacles,
                &MovingObject {
                    row: 1,
                    col: 1,
                    dir: Direction::Left,
                    out_of_bounds: false
                }
            ),
            false
        );
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
            expected_path
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
            expected_path
        );

        plane = vec![vec!['.', '.'], vec!['x', '.']];
        start = MovingObject {
            row: 0,
            col: 0,
            dir: Direction::Down,
            out_of_bounds: false,
        };
        expected_path = vec![MovingObject {
            row: 0,
            col: 0,
            dir: Direction::Down,
            out_of_bounds: false,
        }];

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
}
