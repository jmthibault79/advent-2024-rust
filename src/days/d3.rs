use crate::utils;
use regex::Regex;

fn d3p1(path: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    utils::line_iter(path)
        .map(|l| {
            let line = l.unwrap();
            let line_sum: i32 = re
                .captures_iter(line.as_str())
                .map(|c| {
                    let (_, [a_str, b_str]) = c.extract();
                    println!("match: {:?},{:?}", a_str, b_str);
                    let a: i32 = a_str.parse().expect("integer expected");
                    let b: i32 = b_str.parse().expect("integer expected");
                    a * b
                })
                .sum();
            line_sum
        })
        .sum()
}

pub fn d3() {
    let path = "inputs/d3sample.txt";
    //let path = "inputs/d3.txt";
    let mut result = d3p1(path);
    println!("Result Day 3 Part 1: {}", result);
    //     result = d2p2(path);
    //     println!("Result Day 2 Part 2: {}", result);
}
