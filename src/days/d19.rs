use crate::utils;
use std::collections::HashMap;

fn parse(file_path: &str) -> (Vec<String>, Vec<String>) {
    let mut iter = utils::string_iter(file_path);
    let patterns = iter
        .next()
        .expect("Missing pattern header")
        .split(", ")
        .map(str::to_string)
        .collect();
    let empty_line_to_skip = iter.next().expect("Missing empty line");
    assert!(empty_line_to_skip.is_empty());
    let designs = iter.collect();
    (patterns, designs)
}

fn patterns_can_create<'a>(
    design: &'a String,
    pattern_slices: &Vec<&'a str>,
    memo: &mut HashMap<(&'a str, &'a str), usize>,
) -> bool {
    pattern_combos(design, &pattern_slices, memo) > 0
}

fn pattern_combos<'a>(
    design: &'a str,
    patterns: &Vec<&'a str>,
    memo: &mut HashMap<(&'a str, &'a str), usize>,
) -> usize {
    patterns
        .iter()
        .map(|pattern| {
            if utils::equals(design, *pattern) {
                1
            } else {
                memo.get(&(design, *pattern)).copied().unwrap_or_else(|| {
                    let result = {
                        if design.starts_with(pattern) {
                            pattern_combos(&design[pattern.chars().count()..], patterns, memo)
                        } else {
                            0
                        }
                    };
                    memo.insert((design, *pattern), result);
                    result
                })
            }
        })
        .sum()
}

pub fn d19p1(file_path: &str) -> usize {
    let (patterns, designs) = parse(file_path);
    let pattern_slices: Vec<&str> = patterns.iter().map(String::as_str).collect();

    let mut combo_memoizer: HashMap<(&str, &str), usize> = HashMap::new();

    designs
        .iter()
        .enumerate()
        .filter(|(idx, design)| {
            println!("Checking design {}: {}", idx, design);
            patterns_can_create(design, &pattern_slices, &mut combo_memoizer)
        })
        .count()
}

pub fn d19p2(file_path: &str) -> usize {
    let (patterns, designs) = parse(file_path);
    let pattern_slices: Vec<&str> = patterns.iter().map(String::as_str).collect();

    let mut combo_memoizer: HashMap<(&str, &str), usize> = HashMap::new();

    designs
        .iter()
        .enumerate()
        .map(|(idx, design)| {
            println!("Checking design {}: {}", idx, design);
            pattern_combos(design, &pattern_slices, &mut combo_memoizer)
        })
        .sum()
}

pub fn d19() {
    //let file_path = "inputs/d19sample.txt";
    let file_path = "inputs/d19.txt";
    let mut result = d19p1(file_path);
    println!("Result Day 19 Part 1: {}", result);
    result = d19p2(file_path);
    println!("Result Day 19 Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        assert!(patterns_can_create(
            &"a".to_string(),
            &vec!["a"],
            &mut HashMap::new()
        ));
        assert!(!patterns_can_create(
            &"a".to_string(),
            &vec!["b"],
            &mut HashMap::new()
        ));
        assert!(patterns_can_create(
            &"ab".to_string(),
            &vec!["a", "b"],
            &mut HashMap::new()
        ));
        assert!(patterns_can_create(
            &"ababaaaabbbababa".to_string(),
            &vec!["a", "b"],
            &mut HashMap::new()
        ));
    }
}
