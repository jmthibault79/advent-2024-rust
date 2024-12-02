mod days;
mod utils;

fn main() {
    //let path = "inputs/d1sample.txt";
    let path = "inputs/d1.txt";
    let result1 = days::d1p1(path);
    println!("Result 1: {}", result1);
    let result2 = days::d1p2(path);
    println!("Result 2: {}", result2);
}
