use crate::utils::matrix;

const start: char = 'S';
const end: char = 'E';

pub fn d16p1(file_path: &str) -> usize {
    let maze = matrix::as_char_matrix(file_path);
    0
}

pub fn d16p2(file_path: &str) -> usize {
    0
}

pub fn d16() {
    let file_path = "inputs/d16sample1.txt";
    let mut result = d16p1(file_path);
    println!("Result Day 16 Part 1: {}", result);
    result = d16p2(file_path);
    println!("Result Day 16 Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(true);
    }
}