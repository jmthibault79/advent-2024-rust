use core::str;
use std::cmp::Eq;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub mod matrix;
pub mod plane;

pub fn string_iter(path: &str) -> impl Iterator<Item = String> {
    let path = Path::new(path);
    let file = File::open(&path).expect(format!("couldn't open {}", path.display()).as_str());
    BufReader::new(file).lines().map(Result::unwrap)
}

fn to_int_vec(s: String) -> Vec<i32> {
    s.split_whitespace()
        .map(|x| x.parse().expect("integer expected"))
        .collect()
}

pub fn as_int_vecs(path: &str) -> impl Iterator<Item = Vec<i32>> {
    string_iter(path).map(to_int_vec)
}

fn to_2_ints(v: Vec<i32>) -> (i32, i32) {
    if v.len() != 2 {
        panic!("Expected 2 integers per line, got {}", v.len());
    }
    (v[0], v[1])
}

pub fn as_int_pairs(path: &str) -> impl Iterator<Item = (i32, i32)> {
    string_iter(path).map(to_int_vec).map(to_2_ints)
}

pub fn freqs<T: Hash + Eq>(v: Vec<T>) -> HashMap<T, i32> {
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
