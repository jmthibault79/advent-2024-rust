use crate::utils;
use std::panic::catch_unwind;

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

// level 2 keypad
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

fn l2_shortest_between(src: &DirectionAndPush, dest: &DirectionAndPush) -> Vec<DirectionAndPush> {
    let (first_step, first_dest) = match (src, dest) {
        (s, d) if s == d => {
            return vec![];
        }

        (DirectionAndPush::Push, DirectionAndPush::Up) => {
            (DirectionAndPush::Left, DirectionAndPush::Up)
        }
        (DirectionAndPush::Push, _) => (DirectionAndPush::Down, DirectionAndPush::Right),
        (DirectionAndPush::Up, DirectionAndPush::Push) => {
            (DirectionAndPush::Right, DirectionAndPush::Push)
        }
        (DirectionAndPush::Up, _) => (DirectionAndPush::Down, DirectionAndPush::Down),

        (DirectionAndPush::Right, DirectionAndPush::Push) => {
            (DirectionAndPush::Up, DirectionAndPush::Push)
        }
        (DirectionAndPush::Right, _) => (DirectionAndPush::Left, DirectionAndPush::Down),
        (DirectionAndPush::Down, DirectionAndPush::Left) => {
            (DirectionAndPush::Left, DirectionAndPush::Left)
        }
        (DirectionAndPush::Down, DirectionAndPush::Right) => {
            (DirectionAndPush::Right, DirectionAndPush::Right)
        }
        (DirectionAndPush::Down, _) => (DirectionAndPush::Up, DirectionAndPush::Up),
        (DirectionAndPush::Left, _) => (DirectionAndPush::Right, DirectionAndPush::Down),
    };

    let mut result = vec![first_step];
    if first_dest != *dest {
        let next_steps = l2_shortest_between(&first_dest, dest);
        result.extend(next_steps);
    }

    result
}

fn l2_shortest_path(l1_path: Vec<DirectionAndPush>) -> Vec<DirectionAndPush> {
    let mut result = Vec::new();
    let mut current_pos = DirectionAndPush::Push;
    for button in l1_path {
        let mut steps = l2_shortest_between(&current_pos, &button);

        // sort them in order to group like directions, e.g. (Up, Up, Right) rather than (Up, Right, Up)
        // because we don't need to move between pushes for like-directions, decresing total path length
        steps.sort();

        // println!(
        //     "L2 steps from {:?} to {:?}: {:?}",
        //     current_pos, button, steps
        // );
        result.extend(steps);
        result.push(DirectionAndPush::Push);
        current_pos = button;
    }
    // println!("L2 complete: {:?}", result);
    result
}

pub fn get_complexity(s: String) -> usize {
    println!("s: {}", s);
    let l1_desired_buttons: Vec<char> = s.chars().collect();
    let l1_path = l1_shortest_path(l1_desired_buttons);
    // println!("L1: {:?}", l1_path);
    let l2_path = l2_shortest_path(l1_path);
    // println!("L2: {:?}", l2_path);
    println!("l2 len: {}", l2_path.len());

    // L3 is just L2 again
    let l3_path = l2_shortest_path(l2_path);
    // println!("L3: {:?}", l3_path);
    println!("l3 len: {}", l3_path.len());
    l3_path.len()
}

pub fn d21p1(file_path: &str) -> usize {
    utils::string_iter(file_path).map(get_complexity).sum()
}

pub fn d21p2(file_path: &str) -> usize {
    0
}

pub fn d21() {
    let file_path = "inputs/d21sample.txt";
    let mut result = d21p1(file_path);
    println!("Result Day 21 Part 1: {}", result);
    result = d21p2(file_path);
    println!("Result Day 21 Part 2: {}", result);
}

mod tests {
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

    fn l3_shortest(s: &str) -> Vec<DirectionAndPush> {
        l2_shortest_path(l2_shortest_path(l1_shortest_path(s.chars().collect())))
    }

    #[test]
    fn test() {
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
    fn test_lengths_for_sample() {
        let mut button_str = "029A";
        let mut shortest_path =
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A";
        assert_eq!(l3_shortest(button_str).len(), shortest_path.len());

        button_str = "980A";
        shortest_path = "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A";
        assert_eq!(l3_shortest(button_str).len(), shortest_path.len());

        button_str = "179A";
        shortest_path = "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A";
        assert_eq!(l3_shortest(button_str).len(), shortest_path.len());

        button_str = "456A";
        shortest_path = "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A";
        assert_eq!(l3_shortest(button_str).len(), shortest_path.len());

        button_str = "379A";
        shortest_path = "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A";
        assert_eq!(l3_shortest(button_str).len(), shortest_path.len());
    }

    #[test]
    fn test_prints_for_paths() {
        let mut dir_path = l1_shortest_path("029A".chars().collect());
        let mut expected_dir_path = parse_dir_str("<A^A>^^AvvvA");
        print_paths(&dir_path, &expected_dir_path);
        println!();

        dir_path = l2_shortest_path(dir_path);
        expected_dir_path = parse_dir_str("v<<A>>^A<A>AvA<^AA>A<vAAA>^A");
        print_paths(&dir_path, &expected_dir_path);

        dir_path = l2_shortest_path(dir_path);
        expected_dir_path =
            parse_dir_str("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A");
        print_paths(&dir_path, &expected_dir_path);

        // assert!(false)
    }
}
