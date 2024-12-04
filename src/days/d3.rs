use crate::utils;
use regex::Regex;

fn memory_result(v: Vec<String>) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    v.iter()
        .flat_map(|line| re.captures_iter(&line.as_str()))
        .map(|c| {
            let (_, [a_str, b_str]) = c.extract();
            println!("match: {:?},{:?}", a_str, b_str);
            let a: i32 = a_str.parse().expect("integer expected");
            let b: i32 = b_str.parse().expect("integer expected");
            a * b
        })
        .sum()
}

fn d3p1(path: &str) -> i32 {
    // I'd prefer to keep this as an iterator, but this is necessary to ensure that ownership of the Strings
    // remains outside of the iterator map step, where it would go out of scope before use
    let owned_lines: Vec<String> = utils::line_iter(path).map(Result::unwrap).collect();
    memory_result(owned_lines)
}

fn d3p2(path: &str) -> i32 {
    let enabler = "do()";
    let disabler = "don't()";

    let mut enabled = true;
    let mut enabled_lines: Vec<String> = Vec::new();

    utils::line_iter(path).map(Result::unwrap).for_each(|line| {
        let mut remaining = line.as_str();
        while !remaining.is_empty() {
            if enabled {
                // if we are currently enabled, look for the next disabler
                match remaining.match_indices(disabler).next() {
                    Some((idx, _)) => {
                        let enabled_line = &remaining[..idx];
                        enabled_lines.push(enabled_line.to_string());
                        remaining = &remaining[idx + disabler.len()..];
                        enabled = false;

                        println!(
                            "disabler found.  enabled: {}, remaining: {}",
                            enabled_line, remaining
                        );
                    }
                    None => {
                        println!("disabler not found.  remaining: {}", remaining);

                        enabled_lines.push(remaining.to_string());
                        remaining = "";
                    }
                }
            } else {
                // we are currently disabled, look for the next enabler
                match remaining.match_indices(enabler).next() {
                    Some((idx, _)) => {
                        let disabled_line = &remaining[..idx];
                        remaining = &remaining[idx + enabler.len()..];
                        enabled = true;

                        println!(
                            "enabler found.  disabled: {}, remaining: {}",
                            disabled_line, remaining
                        );
                    }
                    None => {
                        println!("enabler not found.  remaining: {}", remaining);

                        remaining = "";
                    }
                }
            }
        }
    });

    memory_result(enabled_lines)
}

pub fn d3() {
    //let path = "inputs/d3sample.txt";
    //let path = "inputs/d3sample2.txt";
    let path = "inputs/d3.txt";
    let mut result = d3p1(path);
    println!("Result Day 3 Part 1: {}", result);
    result = d3p2(path);
    println!("Result Day 3 Part 2: {}", result);
}
