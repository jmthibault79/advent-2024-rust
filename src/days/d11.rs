use crate::utils;
use core::panic;
use std::collections::HashMap;

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

    // println!("{:?} -> {:?}", n, result);
    result
}

fn apply_rules_h(h: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_hash = HashMap::new();

    for (stone, count) in h {
        for key in rules(&stone) {
            new_hash
                .entry(key)
                .and_modify(|f| *f += count)
                .or_insert(count);
        }
    }

    new_hash
}

fn apply_n_rules_h(h: HashMap<u64, u64>, n: usize) -> HashMap<u64, u64> {
    let mut iterator = h;
    for _i in 0..n {
        iterator = apply_rules_h(iterator);
    }
    iterator
}

pub fn d11p1(h: &HashMap<u64, u64>) -> u64 {
    apply_n_rules_h(h.clone(), 25)
        .iter()
        .map(|(_, count)| count)
        .sum()
}

// ok to move/consume h here
pub fn d11p2(h: HashMap<u64, u64>) -> u64 {
    apply_n_rules_h(h, 75).iter().map(|(_, count)| count).sum()
}

pub fn d11() {
    let file_path = "inputs/d11.txt";
    if let Some(stones) = utils::as_spaced_int_vec::<u64>(file_path).next() {
        let h = utils::freqs_u64(stones);
        let mut result = d11p1(&h);
        println!("Result Day 11 Part 1: {}", result);
        result = d11p2(h);
        println!("Result Day 11 Part 2: {}", result);
    } else {
        panic!("Could not parse input");
    }
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
    fn test_apply_rules_h() {
        let h = HashMap::from([(0, 1), (1, 1), (10, 1), (99, 1), (999, 1)]);

        let expected = HashMap::from([(0, 1), (1, 2), (9, 2), (2024, 1), (2021976, 1)]);

        assert_eq!(apply_rules_h(h), expected);

        let h2 = HashMap::from([(0, 2), (1, 3), (11, 4)]);
        let exp2 = HashMap::from([
            // 2 from 0, 8 from 11
            (1, 10),
            (2024, 3),
        ]);

        assert_eq!(apply_rules_h(h2), exp2);
    }

    #[test]
    fn test_apply_n_rules_h() {
        let h = HashMap::from([(125, 1), (17, 1)]);
        let h2 = h.clone();
        assert_eq!(
            apply_n_rules_h(h, 6)
                .iter()
                .map(|(_, count)| count)
                .sum::<u64>(),
            22
        );
        assert_eq!(
            apply_n_rules_h(h2, 25)
                .iter()
                .map(|(_, count)| count)
                .sum::<u64>(),
            55312
        );
    }
}
