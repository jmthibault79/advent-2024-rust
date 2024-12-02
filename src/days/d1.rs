use crate::utils;

// given two vertical columns of values, sort each column and sum the absolute differences
pub fn d1p1(path: &str) -> i32 {
    let input = utils::as_int_pairs(path);
    let (mut l1, mut l2): (Vec<i32>, Vec<i32>) = input.unzip();
    l1.sort();
    l2.sort();
    l1.iter().zip(l2).map(|(x, y)| (x - y).abs()).sum()
}

// given two vertical columns of values, return the first column multiplied by its frequency in the second column
pub fn d1p2(path: &str) -> i32 {
    let input = utils::as_int_pairs(path);
    let (l1, l2): (Vec<i32>, Vec<i32>) = input.unzip();
    let l2_freqs = utils::freqs(l2);
    l1.iter().map(|x| x * l2_freqs.get(x).unwrap_or(&0)).sum()
}
