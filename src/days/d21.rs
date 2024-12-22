use crate::utils;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone, Hash, Copy)]
pub enum DirectionAndPush {
    Up,    // ^
    Down,  // v
    Left,  // <
    Right, // >
    Push,  // A
}

const START_CHAR: char = 'A';
const START_DIR: DirectionAndPush = DirectionAndPush::Push;

// outside vector is one per button.
// each button has 1 or more path options.
// create the overall set of path options by taking one from each button option
fn combine_button_options<T: Clone>(v: Vec<Vec<Vec<T>>>) -> Vec<Vec<T>> {
    if v.len() == 1 {
        v[0].clone()
    } else {
        let mut result = vec![];
        for first_button_option in &v[0] {
            for rest_button_options in combine_button_options(v[1..].to_vec()) {
                let mut current_path = first_button_option.clone();
                current_path.extend(rest_button_options);
                result.push(current_path);
            }
        }
        result
    }
}

// level 1 keypad
// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
const L1_CHARS: [char; 11] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'A'];
fn get_l1_adjacency_map() -> HashMap<(char, char), DirectionAndPush> {
    let mut adjacency_map = HashMap::new();

    // right

    adjacency_map.insert(('7', '8'), DirectionAndPush::Right);
    adjacency_map.insert(('8', '9'), DirectionAndPush::Right);

    adjacency_map.insert(('4', '5'), DirectionAndPush::Right);
    adjacency_map.insert(('5', '6'), DirectionAndPush::Right);

    adjacency_map.insert(('1', '2'), DirectionAndPush::Right);
    adjacency_map.insert(('2', '3'), DirectionAndPush::Right);

    adjacency_map.insert(('0', 'A'), DirectionAndPush::Right);

    // left

    adjacency_map.insert(('9', '8'), DirectionAndPush::Left);
    adjacency_map.insert(('8', '7'), DirectionAndPush::Left);

    adjacency_map.insert(('6', '5'), DirectionAndPush::Left);
    adjacency_map.insert(('5', '4'), DirectionAndPush::Left);

    adjacency_map.insert(('3', '2'), DirectionAndPush::Left);
    adjacency_map.insert(('2', '1'), DirectionAndPush::Left);

    adjacency_map.insert(('A', '0'), DirectionAndPush::Left);

    // down

    adjacency_map.insert(('7', '4'), DirectionAndPush::Down);
    adjacency_map.insert(('4', '1'), DirectionAndPush::Down);

    adjacency_map.insert(('8', '5'), DirectionAndPush::Down);
    adjacency_map.insert(('5', '2'), DirectionAndPush::Down);
    adjacency_map.insert(('2', '0'), DirectionAndPush::Down);

    adjacency_map.insert(('9', '6'), DirectionAndPush::Down);
    adjacency_map.insert(('6', '3'), DirectionAndPush::Down);
    adjacency_map.insert(('3', 'A'), DirectionAndPush::Down);

    // up

    adjacency_map.insert(('1', '4'), DirectionAndPush::Up);
    adjacency_map.insert(('4', '7'), DirectionAndPush::Up);

    adjacency_map.insert(('0', '2'), DirectionAndPush::Up);
    adjacency_map.insert(('2', '5'), DirectionAndPush::Up);
    adjacency_map.insert(('5', '8'), DirectionAndPush::Up);

    adjacency_map.insert(('A', '3'), DirectionAndPush::Up);
    adjacency_map.insert(('3', '6'), DirectionAndPush::Up);
    adjacency_map.insert(('6', '9'), DirectionAndPush::Up);

    adjacency_map
}

fn l1_shortest_between_2(
    adj: &HashMap<(char, char), DirectionAndPush>,
    seen: &Vec<char>,
    src: char,
    dest: char,
    memoizer: &mut HashMap<(char, char), Vec<Vec<DirectionAndPush>>>,
) -> Vec<Vec<DirectionAndPush>> {
    if let Some(memoized) = memoizer.get(&(src, dest)) {
        return memoized.clone();
    }

    if !L1_CHARS.contains(&src) || !L1_CHARS.contains(&dest) {
        panic!("Invalid source or destination")
    }

    if src == dest {
        return vec![vec![]];
    }

    let mut my_seen = seen.clone();
    let result = match adj.get(&(src, dest)) {
        Some(dir) => vec![vec![dir.clone()]],
        _ => {
            my_seen.push(src);

            // (next step, next step's destination)
            let next_step_options: Vec<(DirectionAndPush, char)> = L1_CHARS
                .iter()
                .flat_map(|next_dest| {
                    // I'd prefer to filter this earlier, but that causes an immutable borrow which interferes with the (mutable borrow) push
                    if my_seen.contains(next_dest) {
                        vec![]
                    } else {
                        match adj.get(&(src, *next_dest)) {
                            Some(dir) => {
                                my_seen.push(*next_dest);
                                vec![(dir.clone(), *next_dest)]
                            }
                            _ => vec![],
                        }
                    }
                })
                .collect();

            // .0 is the next step, .1 is the next step's destination
            // if next_step_options.len() == 1 && next_step_options[0].1 == dest {
            //     return vec![vec![next_step_options[0].0.clone()]];
            // } else {
            let mut paths_from_here = Vec::new();

            for (next_direction, next_dest) in next_step_options {
                let next_paths = l1_shortest_between_2(adj, &my_seen, next_dest, dest, memoizer);
                for mut next_path in next_paths {
                    next_path.insert(0, next_direction.clone());
                    paths_from_here.push(next_path.clone());
                }
            }

            let min_len = paths_from_here.iter().map(|p| p.len()).min().unwrap_or(0);
            paths_from_here
                .iter()
                .filter(|p| p.len() == min_len)
                .cloned()
                .collect()
        }
    };

    memoizer.insert((src, dest), result.clone());
    result
}

fn l1_shortest_paths(
    adj: &HashMap<(char, char), DirectionAndPush>,
    l1_desired_buttons: Vec<char>,
) -> Vec<Vec<DirectionAndPush>> {
    let mut options_per_step = vec![];
    let mut current_pos = START_CHAR;
    let memoizer = &mut HashMap::new();
    for l1_button in l1_desired_buttons {
        let mut current_button_options = Vec::new();
        let step_options_to_button =
            l1_shortest_between_2(adj, &vec![], current_pos, l1_button, memoizer);
        for mut steps_option_to_button in step_options_to_button {
            steps_option_to_button.push(DirectionAndPush::Push);
            current_button_options.push(steps_option_to_button);
        }
        current_pos = l1_button.clone();
        options_per_step.push(current_button_options.clone());
        current_button_options.clear();
    }

    combine_button_options(options_per_step)
}

// level 2 keypad
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

fn l2_shortest_between_2(
    src: DirectionAndPush,
    dest: DirectionAndPush,
    memoizer: &mut HashMap<(DirectionAndPush, DirectionAndPush), Vec<Vec<DirectionAndPush>>>,
) -> Vec<Vec<DirectionAndPush>> {
    if let Some(memoized) = memoizer.get(&(src, dest)) {
        return memoized.clone();
    }

    let next_step_options = match (src, dest) {
        (s, d) if s == d => {
            return vec![vec![]];
        }

        // (next step, next step's destination)
        (DirectionAndPush::Up, DirectionAndPush::Push) => {
            vec![(DirectionAndPush::Right, DirectionAndPush::Push)]
        }
        (DirectionAndPush::Up, DirectionAndPush::Down | DirectionAndPush::Left) => {
            vec![(DirectionAndPush::Down, DirectionAndPush::Down)]
        }
        (DirectionAndPush::Up, _) => vec![
            (DirectionAndPush::Down, DirectionAndPush::Down),
            (DirectionAndPush::Right, DirectionAndPush::Push),
        ],

        (DirectionAndPush::Push, DirectionAndPush::Up) => {
            vec![(DirectionAndPush::Left, DirectionAndPush::Up)]
        }
        (DirectionAndPush::Push, DirectionAndPush::Right) => {
            vec![(DirectionAndPush::Down, DirectionAndPush::Right)]
        }
        (DirectionAndPush::Push, _) => vec![
            (DirectionAndPush::Left, DirectionAndPush::Up),
            (DirectionAndPush::Down, DirectionAndPush::Right),
        ],

        (DirectionAndPush::Left, _) => vec![(DirectionAndPush::Right, DirectionAndPush::Down)],

        (DirectionAndPush::Down, DirectionAndPush::Left) => {
            vec![(DirectionAndPush::Left, DirectionAndPush::Left)]
        }
        (DirectionAndPush::Down, DirectionAndPush::Right) => {
            vec![(DirectionAndPush::Right, DirectionAndPush::Right)]
        }
        (DirectionAndPush::Down, DirectionAndPush::Up) => {
            vec![(DirectionAndPush::Up, DirectionAndPush::Up)]
        }
        (DirectionAndPush::Down, _) => vec![
            (DirectionAndPush::Right, DirectionAndPush::Right),
            (DirectionAndPush::Up, DirectionAndPush::Up),
        ],

        (DirectionAndPush::Right, DirectionAndPush::Push) => {
            vec![(DirectionAndPush::Up, DirectionAndPush::Push)]
        }
        (DirectionAndPush::Right, DirectionAndPush::Down | DirectionAndPush::Left) => {
            vec![(DirectionAndPush::Left, DirectionAndPush::Down)]
        }
        (DirectionAndPush::Right, _) => vec![
            (DirectionAndPush::Left, DirectionAndPush::Down),
            (DirectionAndPush::Up, DirectionAndPush::Push),
        ],
    };

    // .0 is the next step, .1 is the next step's destination
    let result = if next_step_options.len() == 1 && next_step_options[0].1 == dest {
        return vec![vec![next_step_options[0].0.clone()]];
    } else {
        let mut paths_from_here = Vec::new();

        for (next_direction, next_dest) in next_step_options {
            let next_paths = l2_shortest_between_2(next_dest, dest, memoizer);
            for mut next_path in next_paths {
                next_path.insert(0, next_direction.clone());
                paths_from_here.push(next_path.clone());
            }
        }

        paths_from_here
    };

    memoizer.insert((src, dest), result.clone());
    result
}

fn l2_shortest_paths(
    prev_level_path: &Vec<DirectionAndPush>,
    memoizer: &mut HashMap<(DirectionAndPush, DirectionAndPush), Vec<Vec<DirectionAndPush>>>,
) -> Vec<Vec<DirectionAndPush>> {
    let mut options_per_step = vec![];
    let mut current_pos = START_DIR;
    for prev_level_button in prev_level_path {
        let mut current_button_options = Vec::new();
        let step_options_to_button =
            l2_shortest_between_2(current_pos, *prev_level_button, memoizer);
        for mut steps_option_to_button in step_options_to_button {
            steps_option_to_button.push(DirectionAndPush::Push);
            current_button_options.push(steps_option_to_button);
        }
        current_pos = *prev_level_button;
        options_per_step.push(current_button_options.clone());
        current_button_options.clear();
    }

    combine_button_options(options_per_step)
}

pub fn get_complexity(s: String) -> usize {
    println!("s: {}", s);
    let l1_desired_buttons: Vec<char> = s.chars().collect();
    let l1_paths = l1_shortest_paths(&get_l1_adjacency_map(), l1_desired_buttons);
    let l1_min = l1_paths.iter().map(|p| p.len()).min().unwrap();
    let l1_max = l1_paths.iter().map(|p| p.len()).max().unwrap();
    println!(
        "L1 path count {} / min {} / max {}",
        l1_paths.len(),
        l1_min,
        l1_max
    );

    let l2_memoizer: &mut HashMap<
        (DirectionAndPush, DirectionAndPush),
        Vec<Vec<DirectionAndPush>>,
    > = &mut HashMap::new();
    let l2_paths: Vec<Vec<_>> = l1_paths
        .iter()
        .filter(|p| p.len() == l1_min)
        .flat_map(|p| l2_shortest_paths(p, l2_memoizer))
        .collect();
    let l2_min = l2_paths.iter().map(|p| p.len()).min().unwrap();
    let l2_max = l2_paths.iter().map(|p| p.len()).max().unwrap();
    println!(
        "L2 path count {} / min {} / max {}",
        l2_paths.len(),
        l2_min,
        l2_max
    );

    // L3 is just L2 again
    let l3_paths: Vec<Vec<_>> = l2_paths
        .iter()
        .filter(|p| p.len() == l2_min)
        .flat_map(|p| l2_shortest_paths(p, l2_memoizer))
        .collect();
    let l3_min = l3_paths.iter().map(|p| p.len()).min().unwrap();
    let l3_max = l3_paths.iter().map(|p| p.len()).max().unwrap();
    println!(
        "L3 path count {} / min {} / max {}",
        l3_paths.len(),
        l3_min,
        l3_max
    );

    // all inputs are integer plus A
    let parsed_from_input = s[0..s.len() - 1].parse::<usize>().unwrap();
    let complexity = parsed_from_input * l3_min;
    println!(
        "min L3 * parsed: {} * {} = {}",
        l3_min, parsed_from_input, complexity
    );
    complexity
}

pub fn d21p1(file_path: &str) -> usize {
    utils::string_iter(file_path).map(get_complexity).sum()
}

pub fn d21p2(file_path: &str) -> usize {
    0
}

pub fn d21() {
    //let file_path = "inputs/d21sample.txt";
    let file_path = "inputs/d21.txt";
    let mut result = d21p1(file_path);
    println!("Result Day 21 Part 1: {}", result);
    result = d21p2(file_path);
    println!("Result Day 21 Part 2: {}", result);
}

mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_l1_shortest_between_2() {
        let adjacency = get_l1_adjacency_map();
        let seen = vec![];
        let memoizer = &mut HashMap::new();

        assert_eq!(
            l1_shortest_between_2(&adjacency, &seen, '0', 'A', memoizer),
            vec![vec![DirectionAndPush::Right]]
        );
        assert_eq!(
            l1_shortest_between_2(&adjacency, &seen, 'A', '0', memoizer),
            vec![vec![DirectionAndPush::Left]]
        );
        assert_eq!(
            l1_shortest_between_2(&adjacency, &seen, '1', '4', memoizer),
            vec![vec![DirectionAndPush::Up]]
        );
        assert_eq!(
            l1_shortest_between_2(&adjacency, &seen, '5', '8', memoizer),
            vec![vec![DirectionAndPush::Up]]
        );
        assert_eq!(
            l1_shortest_between_2(&adjacency, &seen, '5', '4', memoizer),
            vec![vec![DirectionAndPush::Left]]
        );
        assert_eq!(
            l1_shortest_between_2(&adjacency, &seen, '8', '9', memoizer),
            vec![vec![DirectionAndPush::Right]]
        );

        assert_eq!(
            l1_shortest_between_2(&adjacency, &seen, 'A', 'A', memoizer),
            vec![vec![]]
        );

        assert_eq!(
            l1_shortest_between_2(&adjacency, &seen, '4', '2', memoizer),
            vec![
                vec![DirectionAndPush::Down, DirectionAndPush::Right],
                vec![DirectionAndPush::Right, DirectionAndPush::Down]
            ]
        );

        assert_eq!(
            l1_shortest_between_2(&adjacency, &seen, 'A', '1', memoizer),
            vec![
                vec![
                    DirectionAndPush::Up,
                    DirectionAndPush::Left,
                    DirectionAndPush::Left
                ],
                vec![
                    DirectionAndPush::Left,
                    DirectionAndPush::Up,
                    DirectionAndPush::Left
                ]
            ]
        );
    }

    #[test]
    fn l2_shortest_test() {
        let memoizer = &mut HashMap::new();

        // single option cases

        assert_eq!(
            l2_shortest_between_2(DirectionAndPush::Push, DirectionAndPush::Push, memoizer),
            vec![vec![]]
        );
        assert_eq!(
            l2_shortest_between_2(DirectionAndPush::Left, DirectionAndPush::Down, memoizer),
            vec![vec![DirectionAndPush::Right]]
        );
        assert_eq!(
            l2_shortest_between_2(DirectionAndPush::Left, DirectionAndPush::Right, memoizer),
            vec![vec![DirectionAndPush::Right, DirectionAndPush::Right]]
        );

        // multiple option cases

        assert_eq!(
            l2_shortest_between_2(DirectionAndPush::Down, DirectionAndPush::Push, memoizer),
            vec![
                vec![DirectionAndPush::Right, DirectionAndPush::Up],
                vec![DirectionAndPush::Up, DirectionAndPush::Right]
            ]
        );

        assert_eq!(
            l2_shortest_between_2(DirectionAndPush::Push, DirectionAndPush::Down, memoizer),
            vec![
                vec![DirectionAndPush::Left, DirectionAndPush::Down],
                vec![DirectionAndPush::Down, DirectionAndPush::Left],
            ]
        );

        assert_eq!(
            l2_shortest_between_2(DirectionAndPush::Left, DirectionAndPush::Push, memoizer),
            vec![
                vec![
                    DirectionAndPush::Right,
                    DirectionAndPush::Right,
                    DirectionAndPush::Up
                ],
                vec![
                    DirectionAndPush::Right,
                    DirectionAndPush::Up,
                    DirectionAndPush::Right
                ]
            ]
        );

        assert_eq!(
            l2_shortest_between_2(DirectionAndPush::Push, DirectionAndPush::Left, memoizer),
            vec![
                vec![
                    DirectionAndPush::Left,
                    DirectionAndPush::Down,
                    DirectionAndPush::Left,
                ],
                vec![
                    DirectionAndPush::Down,
                    DirectionAndPush::Left,
                    DirectionAndPush::Left
                ],
            ]
        );
    }

    #[test]
    fn l2_shortest_paths_test() {
        let memoizer = &mut HashMap::new();

        // single button

        assert_eq!(
            l2_shortest_paths(&vec![DirectionAndPush::Push], memoizer),
            vec![vec![DirectionAndPush::Push]]
        );
        assert_eq!(
            l2_shortest_paths(&vec![DirectionAndPush::Up], memoizer),
            vec![vec![DirectionAndPush::Left, DirectionAndPush::Push]]
        );
        assert_eq!(
            l2_shortest_paths(&vec![DirectionAndPush::Right], memoizer),
            vec![vec![DirectionAndPush::Down, DirectionAndPush::Push]]
        );
        assert_eq!(
            l2_shortest_paths(&vec![DirectionAndPush::Down], memoizer),
            vec![
                vec![
                    DirectionAndPush::Left,
                    DirectionAndPush::Down,
                    DirectionAndPush::Push
                ],
                vec![
                    DirectionAndPush::Down,
                    DirectionAndPush::Left,
                    DirectionAndPush::Push
                ],
            ]
        );
        assert_eq!(
            l2_shortest_paths(&vec![DirectionAndPush::Left], memoizer),
            vec![
                vec![
                    DirectionAndPush::Left,
                    DirectionAndPush::Down,
                    DirectionAndPush::Left,
                    DirectionAndPush::Push
                ],
                vec![
                    DirectionAndPush::Down,
                    DirectionAndPush::Left,
                    DirectionAndPush::Left,
                    DirectionAndPush::Push
                ],
            ]
        );

        // simple two step

        assert_eq!(
            l2_shortest_paths(
                &vec![DirectionAndPush::Push, DirectionAndPush::Push],
                memoizer
            ),
            vec![vec![DirectionAndPush::Push, DirectionAndPush::Push]]
        );

        assert_eq!(
            l2_shortest_paths(
                &vec![DirectionAndPush::Push, DirectionAndPush::Up],
                memoizer
            ),
            vec![vec![
                DirectionAndPush::Push,
                DirectionAndPush::Left,
                DirectionAndPush::Push
            ]]
        );

        assert_eq!(
            l2_shortest_paths(
                &vec![DirectionAndPush::Up, DirectionAndPush::Down],
                memoizer
            ),
            vec![vec![
                DirectionAndPush::Left,
                DirectionAndPush::Push,
                DirectionAndPush::Down,
                DirectionAndPush::Push
            ]]
        );

        assert_eq!(
            l2_shortest_paths(
                &vec![DirectionAndPush::Down, DirectionAndPush::Push],
                memoizer
            ),
            vec![
                vec![
                    DirectionAndPush::Left,
                    DirectionAndPush::Down,
                    DirectionAndPush::Push,
                    DirectionAndPush::Right,
                    DirectionAndPush::Up,
                    DirectionAndPush::Push
                ],
                vec![
                    DirectionAndPush::Left,
                    DirectionAndPush::Down,
                    DirectionAndPush::Push,
                    DirectionAndPush::Up,
                    DirectionAndPush::Right,
                    DirectionAndPush::Push
                ],
                vec![
                    DirectionAndPush::Down,
                    DirectionAndPush::Left,
                    DirectionAndPush::Push,
                    DirectionAndPush::Right,
                    DirectionAndPush::Up,
                    DirectionAndPush::Push
                ],
                vec![
                    DirectionAndPush::Down,
                    DirectionAndPush::Left,
                    DirectionAndPush::Push,
                    DirectionAndPush::Up,
                    DirectionAndPush::Right,
                    DirectionAndPush::Push
                ],
            ]
        );
    }

    #[test]
    fn combine_button_options_test() {
        let mut v = vec![vec![vec![1]]];
        let mut expected = vec![vec![1]];
        assert_eq!(combine_button_options(v), expected);

        v = vec![vec![vec![1]], vec![vec![2]]];
        expected = vec![vec![1, 2]];
        assert_eq!(combine_button_options(v), expected);

        v = vec![vec![vec![1, 2]], vec![vec![3, 4]]];
        expected = vec![vec![1, 2, 3, 4]];
        assert_eq!(combine_button_options(v), expected);

        v = vec![vec![vec![1], vec![2]], vec![vec![3, 4]]];
        expected = vec![vec![1, 3, 4], vec![2, 3, 4]];
        assert_eq!(combine_button_options(v), expected);

        v = vec![vec![vec![1]], vec![vec![2], vec![3]]];
        expected = vec![vec![1, 2], vec![1, 3]];
        assert_eq!(combine_button_options(v), expected);

        v = vec![vec![vec![1], vec![2]], vec![vec![3], vec![4]]];
        expected = vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4]];
        assert_eq!(combine_button_options(v), expected);

        v = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];
        expected = vec![
            vec![1, 2, 5, 6],
            vec![1, 2, 7, 8],
            vec![3, 4, 5, 6],
            vec![3, 4, 7, 8],
        ];
        assert_eq!(combine_button_options(v), expected);
    }
}
