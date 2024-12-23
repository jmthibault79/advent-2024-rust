use core::panic;

use crate::utils;

fn rules(n: &u64) -> Vec<u64> {
    let result = match n {
        0 => vec![1],
        even_digits if utils::digit_count(*n) % 2 == 0 => {
            let digits_per_half = utils::digit_count(*n) / 2;
            let divider = 10_u64.pow(digits_per_half);
            vec![even_digits / divider, even_digits % divider]
        }
        other => vec![other * 2024],
    };

    println!("{:?} -> {:?}", n, result);
    result
}

fn apply_rules(v: &Vec<u64>) -> Vec<u64> {
    v.iter().flat_map(rules).collect()
}

fn apply_rules_n(v: &Vec<u64>, n: usize) -> Vec<u64> {
    let mut iterator = v.clone();
    for _ in 0..n {
        iterator = apply_rules(&iterator);
    }
    iterator
}

pub fn d11p1(file_path: &str) -> usize {
    if let Some(stones) = utils::as_spaced_int_vec::<u64>(file_path).next() {
        apply_rules_n(&stones, 25).len()
    } else {
        panic!("Could not parse input");
    }
}

pub fn d11p2(_file_path: &str) -> usize {
    0
}

pub fn d11() {
    let file_path = "inputs/d11.txt";
    let mut result = d11p1(file_path);
    println!("Result Day 11 Part 1: {}", result);
    result = d11p2(file_path);
    println!("Result Day 11 Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rules() {
        assert_eq!(rules(&0), vec![1]);
        assert_eq!(rules(&1), vec![2024]);
        assert_eq!(rules(&11), vec![1, 1]);
        assert_eq!(rules(&123123), vec![123, 123]);
    }

    #[test]
    fn test_apply_rules() {
        assert_eq!(
            apply_rules(&vec![0, 1, 10, 99, 999]),
            vec![1, 2024, 1, 0, 9, 9, 2021976]
        );
    }

    #[test]
    fn test_apply_rules_n() {
        assert_eq!(apply_rules_n(&vec![125, 17], 6).len(), 22);
        assert_eq!(apply_rules_n(&vec![125, 17], 25).len(), 55312);
    }
}
