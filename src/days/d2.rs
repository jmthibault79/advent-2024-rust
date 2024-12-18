use crate::utils;

// safe means monotonically increasing or decreasing and only differing by 1 to 3
fn d2p1_is_safe(v: &Vec<u32>) -> bool {
    let (first, second) = (v[0] as i32, v[1] as i32);
    let direction = match second - first {
        0 => return false,
        x if x > 3 || x < -3 => return false,
        x if x > 0 => 1,
        _ => -1,
    };

    let mut prev = v[1];
    for val in v[2..].iter() {
        let diff = *val as i32 - prev as i32;

        // if a switch in direction (or equality) is detected, return false
        if diff * direction <= 0 {
            return false;
        }

        // if the difference is outside of the bounds 1,2,3, return false
        if diff.abs() > 3 {
            return false;
        }

        prev = *val;
    }
    true
}

// count the number of "safe" rows
pub fn d2p1(path: &str) -> i32 {
    let input = utils::as_spaced_int_vec(path);
    input.filter(d2p1_is_safe).count() as i32
}

// for part 2, more lenient version of safe:
// part 1 safe -> safe
// part 1 safe if we remove a value (any) -> safe
fn d2p2_is_safe(v: &Vec<u32>) -> bool {
    d2p1_is_safe(v) || utils::subsets_removing_1(v).iter().any(d2p1_is_safe)
}

// count the number of "safe" rows
// if you allow removing one value to make the row safe
pub fn d2p2(path: &str) -> i32 {
    let input = utils::as_spaced_int_vec(path);
    input.filter(d2p2_is_safe).count() as i32
}

pub fn d2() {
    //let path = "inputs/d2sample.txt";
    let path = "inputs/d2.txt";
    let mut result = d2p1(path);
    println!("Result Day 2 Part 1: {}", result);
    result = d2p2(path);
    println!("Result Day 2 Part 2: {}", result);
}
