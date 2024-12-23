pub fn dXp1(file_path: &str) -> usize {
    0
}

pub fn dXp2(file_path: &str) -> usize {
    0
}

pub fn dX() {
    let file_path = "inputs/dXsample.txt";
    let mut result = dXp1(file_path);
    println!("Result Day X Part 1: {}", result);
    result = dXp2(file_path);
    println!("Result Day X Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(false);
    }
}