pub fn d20p1(_file_path: &str) -> usize {
    0
}

pub fn d20p2(_file_path: &str) -> usize {
    0
}

pub fn d20() {
    let file_path = "inputs/d20sample.txt";
    let mut result = d20p1(file_path);
    println!("Result Day 20 Part 1: {}", result);
    result = d20p2(file_path);
    println!("Result Day 20 Part 2: {}", result);
}
