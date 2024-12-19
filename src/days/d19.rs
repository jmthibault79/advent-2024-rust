use crate::utils;

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

fn patterns_can_create(design: &String, patterns: &Vec<String>) -> bool {
    pattern_combos(design, patterns) > 0
}

fn pattern_combos(design: &String, patterns: &Vec<String>) -> usize {
    patterns
        .iter()
        .map(|pattern| {
            if design == pattern {
                1
            } else if design.starts_with(pattern) {
                pattern_combos(&design[pattern.len()..].to_string(), patterns)
            } else {
                0
            }
        })
        .sum()
}

pub fn d19p1(file_path: &str) -> usize {
    let (patterns, designs) = parse(file_path);
    designs
        .iter()
        .enumerate()
        .filter(|(idx, design)| {
            println!("Checking design {}: {}", idx, design);
            patterns_can_create(design, &patterns)
        })
        .count()
}

pub fn d19p2(file_path: &str) -> usize {
    let (patterns, designs) = parse(file_path);
    designs
        .iter()
        .enumerate()
        .map(|(idx, design)| {
            println!("Checking design {}: {}", idx, design);
            pattern_combos(design, &patterns)
        })
        .sum()
}

pub fn d19() {
    let file_path = "inputs/d19sample.txt";
    //let file_path = "inputs/d19.txt";
    let mut result = d19p1(file_path);
    println!("Result Day 19 Part 1: {}", result);
    result = d19p2(file_path);
    println!("Result Day 19 Part 2: {}", result);
}

mod tests {
    use super::*;

    #[test]
    fn can_create() {
        assert!(patterns_can_create(
            &"a".to_string(),
            &vec!["a".to_string()]
        ));
        assert!(!patterns_can_create(
            &"a".to_string(),
            &vec!["b".to_string()]
        ));
        assert!(patterns_can_create(
            &"ab".to_string(),
            &vec!["a".to_string(), "b".to_string()]
        ));
        assert!(patterns_can_create(
            &"ababaaaabbbababa".to_string(),
            &vec!["a".to_string(), "b".to_string()]
        ));
    }
}
