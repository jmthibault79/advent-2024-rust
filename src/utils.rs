use core::str;
use std::collections::HashMap;
use std::fs::File;
use std::io::Lines;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn line_iter(path: &str) -> Lines<BufReader<File>> {
    let path = Path::new(path);

    let file = File::open(&path).expect(format!("couldn't open {}", path.display()).as_str());

    BufReader::new(file).lines()
}

fn to_int_vec(s: String) -> Vec<i32> {
    s.split_whitespace()
        .map(|x| x.parse().expect("integer expected"))
        .collect()
}

fn to_2_ints(v: Vec<i32>) -> (i32, i32) {
    if v.len() != 2 {
        panic!("Expected 2 integers per line, got {}", v.len(),);
    }
    (v[0], v[1])
}

pub fn as_int_pairs(path: &str) -> impl Iterator<Item = (i32, i32)> {
    line_iter(path)
        .map(|maybe_line| to_int_vec(maybe_line.unwrap()))
        .map(to_2_ints)
}

pub fn freqs(v: Vec<i32>) -> HashMap<i32, i32> {
    let mut freqs = HashMap::new();
    for x in v {
        freqs.entry(x).and_modify(|f| *f += 1).or_insert(1);
    }
    freqs
}
