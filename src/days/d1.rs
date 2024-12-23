use crate::utils;

// given two vertical columns of values, sort each column and sum the absolute differences
fn d1p1(path: &str) -> u32 {
    let input = utils::as_int_pairs(path);
    let (mut l1, mut l2): (Vec<u32>, Vec<u32>) = input.unzip();
    l1.sort();
    l2.sort();
    l1.iter()
        .zip(l2)
        .map(|(x, y)| (*x as i32 - y as i32).abs() as u32)
        .sum()
}

// given two vertical columns of values, return the first column multiplied by its frequency in the second column
fn d1p2(path: &str) -> u32 {
    let input = utils::as_int_pairs(path);
    let (l1, l2): (Vec<u32>, Vec<u32>) = input.unzip();
    let l2_freqs = utils::freqs_u32(l2);
    l1.iter().map(|x| x * l2_freqs.get(x).unwrap_or(&0)).sum()
}

pub fn d1() {
    //let path = "inputs/d1sample.txt";
    let path = "inputs/d1.txt";
    let mut result = d1p1(path);
    println!("Result Day 1 Part 1: {}", result);
    result = d1p2(path);
    println!("Result Day 1 Part 2: {}", result);
}
