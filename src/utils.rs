use core::str;
use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::{self, File};
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

pub mod matrix;
pub mod plane;

pub fn string_iter(path: &str) -> impl Iterator<Item = String> {
    let path = Path::new(path);
    let file = File::open(&path).expect(format!("couldn't open {}", path.display()).as_str());
    BufReader::new(file).lines().map(Result::unwrap)
}

pub fn read_all(path: &str) -> String {
    fs::read_to_string(path).expect(format!("Could not read file {}", path).as_str())
}

fn to_spaced_int_vec<T: FromStr + Debug>(s: String) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    if s.is_empty() {
        vec![]
    } else {
        s.split_whitespace()
            .map(str::parse)
            .map(|s| s.expect("integer expected"))
            .collect()
    }
}

pub fn as_spaced_int_vec<T: FromStr + Debug>(path: &str) -> impl Iterator<Item = Vec<T>>
where
    <T as FromStr>::Err: Debug,
{
    string_iter(path).map(to_spaced_int_vec)
}

pub fn as_int_pairs(path: &str) -> impl Iterator<Item = (u32, u32)> {
    string_iter(path).map(to_spaced_int_vec).map(|v| {
        if v.len() != 2 {
            panic!("Expected 2 integers per line, got {}", v.len());
        }
        (v[0], v[1])
    })
}

pub fn freqs_u32<T: Hash + Eq>(v: Vec<T>) -> HashMap<T, u32> {
    let mut freqs = HashMap::new();
    for x in v {
        freqs.entry(x).and_modify(|f| *f += 1).or_insert(1);
    }
    freqs
}

pub fn freqs_u64<T: Hash + Eq>(v: Vec<T>) -> HashMap<T, u64> {
    let mut freqs = HashMap::new();
    for x in v {
        freqs.entry(x).and_modify(|f| *f += 1).or_insert(1);
    }
    freqs
}

pub fn subsets_removing_1<T: Clone>(v: &Vec<T>) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    for i in 0..v.len() {
        let mut subset = v[0..i].to_vec();
        subset.extend_from_slice(&v[i + 1..]);
        result.push(subset);
    }
    result
}

pub fn all_pairs<T: Clone>(v: &Vec<T>) -> Vec<(T, T)> {
    let mut result = Vec::new();
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            result.push((v[i].clone(), v[j].clone()));
        }
    }
    result
}

pub fn distinct<T>(v: Vec<T>) -> Vec<T>
where
    T: Eq + Hash + Clone,
{
    let mut seen = HashMap::new();
    for x in v {
        if !seen.contains_key(&x) {
            seen.insert(x.clone(), 1);
        }
    }
    seen.keys().cloned().collect()
}

pub fn equals(a: &str, b: &str) -> bool {
    a.chars().count() == b.chars().count() && a.chars().zip(b.chars()).all(|(a, b)| a == b)
}

pub fn digit_count(n: u64) -> u32 {
    n.ilog10() + 1
}

// solves a pair of simultaneous equations of the form:
// a1 * x + b1 * y = c1
// a2 * x + b2 * y = c2
// returns None if there is no solution (parallel lines) or if there is no positive integer solution
pub fn simultaneous_equations_posint_result(
    a1: &i64,
    b1: &i64,
    c1: &i64,
    a2: &i64,
    b2: &i64,
    c2: &i64,
) -> Option<(u64, u64)> {
    let det = a1 * b2 - a2 * b1;
    if det == 0 {
        None
    } else {
        let x_numerator = c1 * b2 - c2 * b1;
        let y_numerator = a1 * c2 - a2 * c1;
        if x_numerator % det != 0 || y_numerator % det != 0 {
            None
        } else {
            let x = x_numerator / det;
            let y = y_numerator / det;
            if x < 0 || y < 0 {
                None
            } else {
                Some((x as u64, y as u64))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_pairs_test() {
        assert_eq!(all_pairs(&vec!['a']), vec![]);
        assert_eq!(all_pairs(&vec!['a', 'b']), vec![('a', 'b')]);
        assert_eq!(
            all_pairs(&vec!['a', 'b', 'c']).sort(),
            vec![('a', 'b'), ('b', 'c'), ('a', 'c')].sort()
        );
    }

    #[test]
    fn str_equals_test() {
        assert!(equals("", ""));
        assert!(!equals("a", ""));
        assert!(!equals("", "a"));
        assert!(equals("a", "a"));
        assert!(equals("ab", "ab"));
        assert!(!equals("a", "ab"));
        assert!(!equals("ab", "a"));
        assert!(!equals("b", "ab"));
        assert!(!equals("ab", "b"));
    }

    #[test]
    fn simult_test() {
        // x = 7, y = 11, by definition
        assert_eq!(
            simultaneous_equations_posint_result(&1, &0, &7, &0, &1, &11),
            Some((7, 11))
        );

        // no solution because parallel
        assert!(simultaneous_equations_posint_result(&1, &2, &7, &2, &4, &11).is_none());

        // None because non-integer solution
        assert!(simultaneous_equations_posint_result(&1, &0, &7, &0, &2, &11).is_none());

        // None because the solution for x is negative
        assert!(simultaneous_equations_posint_result(&-1, &0, &7, &0, &1, &11).is_none());

        // examples given by AoC d13p1
        // 94a + 22b = 8400, 34a + 67b = 5400
        // a = 80, b = 40
        assert_eq!(
            simultaneous_equations_posint_result(&94, &22, &8400, &34, &67, &5400),
            Some((80, 40))
        );

        // 26a + 67b = 12748, 66a + 21b = 12176
        // NONE
        assert!(simultaneous_equations_posint_result(&26, &67, &12748, &66, &21, &12176).is_none());

        // 17a + 84b = 7870, 86a + 37b = 6450
        // a = 38, b = 86
        assert_eq!(
            simultaneous_equations_posint_result(&17, &84, &7870, &86, &37, &6450),
            Some((38, 86))
        );

        // 69a + 27b = 18641, 23a+ 71b = 10279
        // NONE
        assert!(simultaneous_equations_posint_result(&69, &27, &18641, &23, &71, &10279).is_none());

        // examples given by AoC d13p2
        assert!(simultaneous_equations_posint_result(
            &94,
            &22,
            &10000000008400,
            &34,
            &67,
            &10000000005400
        )
        .is_none());
        assert!(simultaneous_equations_posint_result(
            &26,
            &67,
            &10000000012748,
            &66,
            &21,
            &10000000012176
        )
        .is_some());
        assert!(simultaneous_equations_posint_result(
            &17,
            &84,
            &10000000007870,
            &86,
            &37,
            &10000000006450
        )
        .is_none());
        assert!(simultaneous_equations_posint_result(
            &69,
            &27,
            &10000000018641,
            &23,
            &71,
            &10000000010279
        )
        .is_some());
    }
}
