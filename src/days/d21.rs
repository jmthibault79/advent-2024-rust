use crate::utils;
use std::collections::HashMap;

const START_POS: char = 'A';

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone)]
pub enum DirectionAndPush {
    Up,    // ^
    Down,  // v
    Left,  // <
    Right, // >
    Push,  // A
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
) -> Vec<Vec<DirectionAndPush>> {
    if !L1_CHARS.contains(&src) || !L1_CHARS.contains(&dest) {
        panic!("Invalid source or destination")
    }

    if src == dest {
        return vec![vec![]];
    }

    let mut my_seen = seen.clone();
    match adj.get(&(src, dest)) {
        Some(dir) => vec![vec![dir.clone()]],
        _ => {
            my_seen.push(src);

            // (next step, next step's destination)
            let next_step_options: Vec<(DirectionAndPush, char)> = L1_CHARS
                .iter()
                .flat_map(|next_dest| {
                    // I'd prefer to filter this earlier, but that causes an immutable borrow which interferes with the (mutable borrow) push
                    if (my_seen.contains(next_dest)) {
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
                let next_paths = l1_shortest_between_2(adj, &my_seen, next_dest, dest);
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
    }
}

fn l1_shortest_between(src: char, dest: char) -> Vec<DirectionAndPush> {
    let (first_step, first_dest) = match (src, dest) {
        (s, d) if !L1_CHARS.contains(&s) || !L1_CHARS.contains(&d) => {
            panic!("Invalid source or destination")
        }
        (s, d) if s == d => {
            return vec![];
        }

        ('0', 'A') => (DirectionAndPush::Right, 'A'),
        ('0', _) => (DirectionAndPush::Up, '2'),
        ('A', '0') => (DirectionAndPush::Left, '0'),
        ('A', _) => (DirectionAndPush::Up, '3'),
        ('1', d) if ['0', 'A', '2', '3'].contains(&d) => (DirectionAndPush::Right, '2'),
        ('1', _) => (DirectionAndPush::Up, '4'),
        ('2', '1') => (DirectionAndPush::Left, '1'),
        ('2', '3') => (DirectionAndPush::Right, '3'),
        ('2', d) if ['0', 'A'].contains(&d) => (DirectionAndPush::Down, '0'),
        ('2', _) => (DirectionAndPush::Up, '5'),
        ('3', d) if ['1', '2'].contains(&d) => (DirectionAndPush::Left, '2'),
        ('3', d) if ['0', 'A'].contains(&d) => (DirectionAndPush::Down, 'A'),
        ('3', _) => (DirectionAndPush::Up, '6'),
        ('4', d) if ['5', '6'].contains(&d) => (DirectionAndPush::Right, '5'),
        ('4', d) if ['7', '8', '9'].contains(&d) => (DirectionAndPush::Up, '7'),
        ('4', _) => (DirectionAndPush::Down, '1'),
        ('5', '4') => (DirectionAndPush::Left, '4'),
        ('5', '6') => (DirectionAndPush::Right, '6'),
        ('5', d) if ['7', '8', '9'].contains(&d) => (DirectionAndPush::Up, '8'),
        ('5', _) => (DirectionAndPush::Down, '2'),
        ('6', d) if ['4', '5'].contains(&d) => (DirectionAndPush::Left, '5'),
        ('6', d) if ['7', '8', '9'].contains(&d) => (DirectionAndPush::Up, '9'),
        ('6', _) => (DirectionAndPush::Down, '3'),
        ('7', d) if ['8', '9'].contains(&d) => (DirectionAndPush::Right, '8'),
        ('8', '7') => (DirectionAndPush::Left, '7'),
        ('8', '9') => (DirectionAndPush::Right, '9'),
        ('7', _) => (DirectionAndPush::Down, '4'),
        ('8', _) => (DirectionAndPush::Down, '5'),
        ('9', d) if ['7', '8'].contains(&d) => (DirectionAndPush::Left, '8'),
        ('9', _) => (DirectionAndPush::Down, '6'),

        _ => panic!(
            "uh oh, I didn't account for the case of {} -> {}",
            src, dest
        ),
    };

    let mut result = vec![first_step];
    if first_dest != dest {
        let next_steps = l1_shortest_between(first_dest, dest);
        result.extend(next_steps);
    }

    result
}

fn l1_shortest_path(l1_desired_buttons: Vec<char>) -> Vec<DirectionAndPush> {
    let mut result = Vec::new();
    let mut current_pos = START_POS;
    for button in l1_desired_buttons {
        let mut steps = l1_shortest_between(current_pos, button);

        // sort them in order to group like directions, e.g. (Up, Up, Right) rather than (Up, Right, Up)
        // because we don't need to move between pushes for like-directions, decresing total path length
        steps.sort();

        // println!("L1 steps from {} to {}: {:?}", current_pos, button, steps);
        result.extend(steps);
        result.push(DirectionAndPush::Push);
        current_pos = button;
    }
    // println!("L1 complete: {:?}", result);
    result
}

fn l1_shortest_paths(
    adj: &HashMap<(char, char), DirectionAndPush>,
    l1_desired_buttons: Vec<char>,
) -> Vec<Vec<DirectionAndPush>> {
    let mut options_per_step = vec![];
    let mut current_pos = START_POS;
    for l1_button in l1_desired_buttons {
        let mut current_button_options = Vec::new();
        let step_options_to_button = l1_shortest_between_2(adj, &vec![], current_pos, l1_button);
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
    src: &DirectionAndPush,
    dest: &DirectionAndPush,
) -> Vec<Vec<DirectionAndPush>> {
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
    if next_step_options.len() == 1 && next_step_options[0].1 == *dest {
        return vec![vec![next_step_options[0].0.clone()]];
    } else {
        let mut paths_from_here = Vec::new();

        for (next_direction, next_dest) in next_step_options {
            let next_paths = l2_shortest_between_2(&next_dest, dest);
            for mut next_path in next_paths {
                next_path.insert(0, next_direction.clone());
                paths_from_here.push(next_path.clone());
            }
        }

        paths_from_here
    }
}

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

fn l2_shortest_paths(prev_level_path: &Vec<DirectionAndPush>) -> Vec<Vec<DirectionAndPush>> {
    let mut options_per_step = vec![];
    let mut current_pos = DirectionAndPush::Push;
    for prev_level_button in prev_level_path {
        let mut current_button_options = Vec::new();
        let step_options_to_button = l2_shortest_between_2(&current_pos, &prev_level_button);
        for mut steps_option_to_button in step_options_to_button {
            steps_option_to_button.push(DirectionAndPush::Push);
            current_button_options.push(steps_option_to_button);
        }
        current_pos = prev_level_button.clone();
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

    let l2_paths: Vec<Vec<_>> = l1_paths
        .iter()
        .filter(|p| p.len() == l1_min)
        .flat_map(l2_shortest_paths)
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
        .flat_map(l2_shortest_paths)
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
    use std::panic::catch_unwind;
    use std::vec;

    use super::*;

    fn parse_dir_str(s: &str) -> Vec<DirectionAndPush> {
        s.chars()
            .map(|c| match c {
                '^' => DirectionAndPush::Up,
                'v' => DirectionAndPush::Down,
                '<' => DirectionAndPush::Left,
                '>' => DirectionAndPush::Right,
                'A' => DirectionAndPush::Push,
                _ => panic!("Invalid direction"),
            })
            .collect()
    }

    fn print_paths(p1: &[DirectionAndPush], p2: &[DirectionAndPush]) {
        println!(
            "Printing paths. P1 len = {}, P2 len = {}",
            p1.len(),
            p2.len()
        );
        let (mut p1_itr, mut p2_itr) = (0, 0);
        while p1_itr < p1.len() && p2_itr < p2.len() {
            print!("P1: ");
            while p1_itr < p1.len() {
                let p1_val = &p1[p1_itr];
                print!("{:?}, ", p1_val);
                p1_itr += 1;
                if *p1_val == DirectionAndPush::Push {
                    break;
                }
            }

            print!("P2: ");
            while p2_itr < p2.len() {
                let p2_val = &p2[p2_itr];
                print!("{:?}, ", p2_val);
                p2_itr += 1;
                if *p2_val == DirectionAndPush::Push {
                    break;
                }
            }

            println!();
        }
    }

    // fn l3_shortest(s: &str) -> Vec<DirectionAndPush> {
    //     l2_shortest_path(l2_shortest_path(l1_shortest_path(s.chars().collect())))
    // }

    #[test]
    fn test_l1_shortest_between() {
        assert!(catch_unwind(|| l1_shortest_between('1', 'q')).is_err());

        assert_eq!(l1_shortest_between('0', 'A'), vec![DirectionAndPush::Right]);
        assert_eq!(l1_shortest_between('A', '0'), vec![DirectionAndPush::Left]);
        assert_eq!(l1_shortest_between('1', '4'), vec![DirectionAndPush::Up]);
        assert_eq!(l1_shortest_between('5', '8'), vec![DirectionAndPush::Up]);
        assert_eq!(l1_shortest_between('5', '4'), vec![DirectionAndPush::Left]);
        assert_eq!(l1_shortest_between('8', '9'), vec![DirectionAndPush::Right]);

        assert_eq!(l1_shortest_between('A', 'A'), vec![]);

        assert_eq!(
            l1_shortest_between('0', '7'),
            vec![
                DirectionAndPush::Up,
                DirectionAndPush::Up,
                DirectionAndPush::Up,
                DirectionAndPush::Left
            ]
        );
        assert_eq!(
            l1_shortest_between('A', '1'),
            vec![
                DirectionAndPush::Up,
                DirectionAndPush::Left,
                DirectionAndPush::Left
            ]
        );

        assert_eq!(
            l1_shortest_between('4', '2'),
            vec![DirectionAndPush::Down, DirectionAndPush::Right]
        );
        assert_eq!(
            l1_shortest_between('9', '5'),
            vec![DirectionAndPush::Down, DirectionAndPush::Left]
        );
    }

    #[test]
    fn test_l1_shortest_between_2() {
        let adjacency = get_l1_adjacency_map();
        let seen = vec![];

        assert!(catch_unwind(|| l1_shortest_between_2(&adjacency, &seen, '1', 'q')).is_err());

        assert_eq!(
            l1_shortest_between_2(&adjacency, &seen, '0', 'A'),
            vec![vec![DirectionAndPush::Right]]
        );
        assert_eq!(
            l1_shortest_between_2(&adjacency, &seen, 'A', '0'),
            vec![vec![DirectionAndPush::Left]]
        );
        assert_eq!(
            l1_shortest_between_2(&adjacency, &seen, '1', '4'),
            vec![vec![DirectionAndPush::Up]]
        );
        assert_eq!(
            l1_shortest_between_2(&adjacency, &seen, '5', '8'),
            vec![vec![DirectionAndPush::Up]]
        );
        assert_eq!(
            l1_shortest_between_2(&adjacency, &seen, '5', '4'),
            vec![vec![DirectionAndPush::Left]]
        );
        assert_eq!(
            l1_shortest_between_2(&adjacency, &seen, '8', '9'),
            vec![vec![DirectionAndPush::Right]]
        );

        assert_eq!(
            l1_shortest_between_2(&adjacency, &seen, 'A', 'A'),
            vec![vec![]]
        );

        assert_eq!(
            l1_shortest_between_2(&adjacency, &seen, '4', '2'),
            vec![
                vec![DirectionAndPush::Down, DirectionAndPush::Right],
                vec![DirectionAndPush::Right, DirectionAndPush::Down]
            ]
        );

        assert_eq!(
            l1_shortest_between_2(&adjacency, &seen, 'A', '1'),
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
        // single option cases

        assert_eq!(
            l2_shortest_between_2(&DirectionAndPush::Push, &DirectionAndPush::Push),
            vec![vec![]]
        );
        assert_eq!(
            l2_shortest_between_2(&DirectionAndPush::Left, &DirectionAndPush::Down),
            vec![vec![DirectionAndPush::Right]]
        );
        assert_eq!(
            l2_shortest_between_2(&DirectionAndPush::Left, &DirectionAndPush::Right),
            vec![vec![DirectionAndPush::Right, DirectionAndPush::Right]]
        );

        // multiple option cases

        assert_eq!(
            l2_shortest_between_2(&DirectionAndPush::Down, &DirectionAndPush::Push),
            vec![
                vec![DirectionAndPush::Right, DirectionAndPush::Up],
                vec![DirectionAndPush::Up, DirectionAndPush::Right]
            ]
        );

        assert_eq!(
            l2_shortest_between_2(&DirectionAndPush::Push, &DirectionAndPush::Down),
            vec![
                vec![DirectionAndPush::Left, DirectionAndPush::Down],
                vec![DirectionAndPush::Down, DirectionAndPush::Left],
            ]
        );

        assert_eq!(
            l2_shortest_between_2(&DirectionAndPush::Left, &DirectionAndPush::Push),
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
            l2_shortest_between_2(&DirectionAndPush::Push, &DirectionAndPush::Left),
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
        // single button

        assert_eq!(
            l2_shortest_paths(&vec![DirectionAndPush::Push]),
            vec![vec![DirectionAndPush::Push]]
        );
        assert_eq!(
            l2_shortest_paths(&vec![DirectionAndPush::Up]),
            vec![vec![DirectionAndPush::Left, DirectionAndPush::Push]]
        );
        assert_eq!(
            l2_shortest_paths(&vec![DirectionAndPush::Right]),
            vec![vec![DirectionAndPush::Down, DirectionAndPush::Push]]
        );
        assert_eq!(
            l2_shortest_paths(&vec![DirectionAndPush::Down]),
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
            l2_shortest_paths(&vec![DirectionAndPush::Left]),
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
            l2_shortest_paths(&vec![DirectionAndPush::Push, DirectionAndPush::Push]),
            vec![vec![DirectionAndPush::Push, DirectionAndPush::Push]]
        );

        assert_eq!(
            l2_shortest_paths(&vec![DirectionAndPush::Push, DirectionAndPush::Up]),
            vec![vec![
                DirectionAndPush::Push,
                DirectionAndPush::Left,
                DirectionAndPush::Push
            ]]
        );

        assert_eq!(
            l2_shortest_paths(&vec![DirectionAndPush::Up, DirectionAndPush::Down]),
            vec![vec![
                DirectionAndPush::Left,
                DirectionAndPush::Push,
                DirectionAndPush::Down,
                DirectionAndPush::Push
            ]]
        );

        assert_eq!(
            l2_shortest_paths(&vec![DirectionAndPush::Down, DirectionAndPush::Push]),
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
    // #[test]
    // fn test_lengths_for_sample() {
    //     let mut button_str = "029A";
    //     let mut shortest_path =
    //         "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A";
    //     assert_eq!(l3_shortest(button_str).len(), shortest_path.len());

    //     button_str = "980A";
    //     shortest_path = "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A";
    //     assert_eq!(l3_shortest(button_str).len(), shortest_path.len());

    //     button_str = "179A";
    //     shortest_path = "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A";
    //     assert_eq!(l3_shortest(button_str).len(), shortest_path.len());

    //     button_str = "456A";
    //     shortest_path = "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A";
    //     assert_eq!(l3_shortest(button_str).len(), shortest_path.len());

    //     button_str = "379A";
    //     shortest_path = "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A";
    //     assert_eq!(l3_shortest(button_str).len(), shortest_path.len());
    // }

    // #[test]
    // fn test_prints_for_paths() {
    //     let mut dir_path = l1_shortest_path("029A".chars().collect());
    //     let mut expected_dir_path = parse_dir_str("<A^A>^^AvvvA");
    //     print_paths(&dir_path, &expected_dir_path);
    //     println!();

    //     dir_path = l2_shortest_path(dir_path);
    //     expected_dir_path = parse_dir_str("v<<A>>^A<A>AvA<^AA>A<vAAA>^A");
    //     print_paths(&dir_path, &expected_dir_path);

    //     dir_path = l2_shortest_path(dir_path);
    //     expected_dir_path =
    //         parse_dir_str("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A");
    //     print_paths(&dir_path, &expected_dir_path);

    //     // assert!(false)
    //    }
}
