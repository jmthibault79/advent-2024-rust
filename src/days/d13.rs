use crate::utils;
use regex::Regex;

fn parse_button(raw_button: &str) -> (i64, i64) {
    let re = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();

    let (_, [x_str, y_str]) = re.captures(raw_button).unwrap().extract();
    let x = x_str.parse().expect("integer expected");
    let y = y_str.parse().expect("integer expected");
    (x, y)
}

fn parse_prize(raw_prize: &str) -> (i64, i64) {
    let re = Regex::new(r"X\=(\d+), Y\=(\d+)").unwrap();

    let (_, [x_str, y_str]) = re.captures(raw_prize).unwrap().extract();
    let x = x_str.parse().expect("integer expected");
    let y = y_str.parse().expect("integer expected");
    (x, y)
}

// parse
// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400
// into (94, 22, 8400, 34, 67, 5400)
fn parse(file_path: &str) -> Vec<(i64, i64, i64, i64, i64, i64)> {
    let mut result = vec![];
    let mut lines = utils::string_iter(file_path);
    while let Some(raw_button_a) = lines.next() {
        let button_a = raw_button_a.split(": ").skip(1).next().unwrap();
        let (ax, ay) = parse_button(button_a);

        let raw_button_b = lines.next().unwrap();
        let button_b = raw_button_b.split(": ").skip(1).next().unwrap();
        let (bx, by) = parse_button(button_b);

        let raw_prize = lines.next().unwrap();
        let prize = raw_prize.split(": ").skip(1).next().unwrap();
        let (x_prize, y_prize) = parse_prize(prize);

        result.push((ax, bx, x_prize, ay, by, y_prize));

        // consume empty separator line or EOF
        lines.next();
    }

    result
}

pub fn d13p1(file_path: &str) -> u64 {
    parse(file_path)
        .iter()
        .map(|(ax, bx, x_prize, ay, by, y_prize)| {
            utils::simultaneous_equations_posint_result(ax, bx, x_prize, ay, by, y_prize)
        })
        .flatten() // drop the nones
        .map(|(a_button_presses, b_button_presses)| 3 * a_button_presses + b_button_presses)
        .sum()
}

pub fn d13p2(file_path: &str) -> u64 {
    0
}

pub fn d13() {
    // let file_path = "inputs/d13sample.txt";
    let file_path = "inputs/d13.txt";
    let mut result = d13p1(file_path);
    println!("Result Day 13 Part 1: {}", result);
    result = d13p2(file_path);
    println!("Result Day 13 Part 2: {}", result);
}