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
}
