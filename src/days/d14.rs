use crate::utils;
use crate::utils::matrix;
use regex::Regex;
use std::io;

// parse lines like p=0,4 v=3,-3 into (0, 4, 3, -3)
fn parse(file_path: &str) -> Vec<(isize, isize, isize, isize)> {
    let re = Regex::new(r"p\=(-?\d+),(-?\d+) v\=(-?\d+),(-?\d+)").unwrap();

    let mut result = vec![];

    let mut lines = utils::string_iter(file_path);
    while let Some(robot_line) = lines.next() {
        let (_, [px_str, py_str, vx_str, vy_str]) =
            re.captures(robot_line.as_str()).unwrap().extract();
        let px = px_str.parse().expect("integer expected");
        let py = py_str.parse().expect("integer expected");
        let vx = vx_str.parse().expect("integer expected");
        let vy = vy_str.parse().expect("integer expected");

        // println!("{} {} {} {}", px, py, vx, vy);
        result.push((px, py, vx, vy));
    }

    result
}

// what is the new location of the robot after N moves, with wraparound?
fn move_n(
    x_total: usize,
    y_total: usize,
    px: isize,
    py: isize,
    vx: isize,
    vy: isize,
    n: isize,
) -> (isize, isize) {
    let new_px = (px + n * vx).rem_euclid(x_total as isize);
    let new_py = (py + n * vy).rem_euclid(y_total as isize);
    (new_px, new_py)
}

fn move_all_n(
    x_total: usize,
    y_total: usize,
    robots: &Vec<(isize, isize, isize, isize)>,
    n: isize,
) -> Vec<(isize, isize)> {
    robots
        .iter()
        .map(|&(px, py, vx, vy)| {
            let (new_px, new_py) = move_n(x_total, y_total, px, py, vx, vy, n);
            (new_px, new_py)
        })
        .collect()
}

// count the number of robots in each quadrant and multiply.
// robots on the center lines are not counted.
fn safety_factor(x_total: usize, y_total: usize, robots: &Vec<(isize, isize)>) -> usize {
    let x_center = x_total / 2;
    let y_center = y_total / 2;

    let (mut low_low, mut low_high, mut high_low, mut high_high) = (0, 0, 0, 0);
    for robot in robots {
        match robot {
            (x, y) if *x < x_center as isize && *y < y_center as isize => low_low += 1,
            (x, y) if *x < x_center as isize && *y > y_center as isize => low_high += 1,
            (x, y) if *x > x_center as isize && *y < y_center as isize => high_low += 1,
            (x, y) if *x > x_center as isize && *y > y_center as isize => high_high += 1,
            _ => {}
        }
    }
    low_low * low_high * high_low * high_high
}

pub fn d14p1(file_path: &str, x_total: usize, y_total: usize) -> usize {
    let robot_starts = parse(file_path);
    let robot_ends = move_all_n(x_total, y_total, &robot_starts, 100);
    safety_factor(x_total, y_total, &robot_ends)
}

fn print_robots_after_n(
    x_total: usize,
    y_total: usize,
    robots: &Vec<(isize, isize, isize, isize)>,
    n: isize,
) -> isize {
    let mut to_print = vec![vec!['.'; x_total]; y_total];

    for robot in move_all_n(x_total, y_total, robots, n) {
        let (px, py) = robot;
        to_print[py as usize][px as usize] = '#';
    }
    matrix::pretty_print(&to_print);
    println!("{}", n);
    n
}

pub fn d14p2(file_path: &str, x_total: usize, y_total: usize) -> usize {
    let robot_starts = parse(file_path);
    let mut n = 0;

    loop {
        println!("move robots (Y/n)?");

        let mut yesno: String = String::new();
        io::stdin()
            .read_line(&mut yesno)
            .expect("Failed to read line");
        if yesno.contains("n") {
            break;
        } else {
            n += 1;
            print_robots_after_n(x_total, y_total, &robot_starts, n);
        }
    }

    // empirically, I observed the robots clustering:
    // vertically at 38, 139, 240 (cycle = 101)
    // horizontally at 88, 191 (cycle = 103)

    // where do these cycles converge?
    // n = 38 + 101 * x
    // n = 88 + 103 * x
    // x = -25, n = -2487

    n = -2487;
    // YES I do see it there
    print_robots_after_n(x_total, y_total, &robot_starts, n);

    // try converting to positive using wraparound modulo logic
    n.rem_euclid((x_total * y_total) as isize) as usize
}

pub fn d14() {
    // let file_path = "inputs/d14sample.txt";
    // let (x_total, y_total) = (11, 7); // sample input
    let file_path = "inputs/d14.txt";
    let (x_total, y_total) = (101, 103); // real input
    let mut result = d14p1(file_path, x_total, y_total);
    println!("Result Day 14 Part 1: {}", result);
    result = d14p2(file_path, x_total, y_total);
    println!("Result Day 14 Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_n() {
        assert_eq!(move_n(2, 2, 0, 0, 1, 1, 1), (1, 1));
        assert_eq!(move_n(2, 2, 0, 0, -1, -1, 1), (1, 1));
        assert_eq!(move_n(2, 2, 0, 1, 0, 1, 1), (0, 0));
        assert_eq!(move_n(2, 2, 1, 0, 1, 0, 1), (0, 0));
        assert_eq!(move_n(2, 2, 0, 0, 1, 1, 2), (0, 0));
        assert_eq!(move_n(3, 3, 0, 0, 1, 1, 5), (2, 2));
        assert_eq!(move_n(3, 3, 0, 0, -1, -1, 5), (1, 1));
    }
}
