use crate::utils;

fn d5p1(path: &str) -> u32 {
    let rules = utils::string_iter(path);
    0
}

pub fn d5() {
    let path = "inputs/d4.txt";
    let mut result = d5p1(path);
    println!("Result Day 5 Part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder() {
        assert!(true);
    }
}
